use aws_sdk_s3::{error::SdkError, primitives::ByteStream};
use axum::{
    Json, Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::post,
};
use nanoid::nanoid;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    cache::RedisCache,
    db::{self, user::DBUser},
    entity::user::UserEntity,
    error::{ApiError, ApiErrorResponse, ApiResult, bad_request::BadRequestError},
    protocol::error::{ForbiddenErrorResponse, UnauthotizedErrorResponse},
    router::extractors::auth::UserAuth,
};

pub fn avatar_router(_res: AppResources) -> Router<AppResources> {
    Router::new()
        .route(
            "/",
            post(upload_avatar).layer(DefaultBodyLimit::max(1024 * 1024 * 20)), // 20MB
        )
        .route("/{user_id}", axum::routing::get(get_avatar))
        .route("/", axum::routing::delete(delete_avatar))
}

#[derive(ToSchema)]
pub struct UploadAvatar {
    /// File to upload
    #[schema(value_type = String, format = Binary)]
    pub avatar: Vec<u8>,
}

#[utoipa::path(
    post,
    path = "",
    tag = "avatar",
    request_body(content_type = "multipart/form-data", content = UploadAvatar),
    security(("api_key" = [])),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ApiErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
    ),
)]
async fn upload_avatar(
    State(app): State<AppResources>,
    UserAuth(mut user): UserAuth,
    req: Multipart,
) -> ApiResult<impl IntoResponse> {
    let storage_key = upload_avatar_data(req, &app, "avatar").await?;
    let upd_user = db::user::DBUpdateUser {
        avatar: Some(storage_key.clone()),
        avatar_preview: Some(storage_key.clone()),
        ..Default::default()
    };
    let mut conn = app.db.acquire().await?;
    let updated = upd_user.update(&user.id, &mut conn).await?;
    let current_avatar = user.avatar.clone();
    if let Some(updated) = updated {
        user = UserEntity::from(updated);
        user.cache(&app.redis).await?;
    }
    if let Some(ref avatar) = current_avatar {
        app.s3
            .delete_object()
            .bucket(&app.config.s3.bucket)
            .key(avatar)
            .send()
            .await
            .map_err(|_| ApiError::InternalServerError)?;
    }
    Ok(Json(serde_json::json!({"status": "ok"})))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "avatar",
    responses(
        (status = 200, description = "OK"),
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
    ),
)]
async fn get_avatar(
    State(app): State<AppResources>,
    Path(user_id): Path<Uuid>,
    headers: HeaderMap,
) -> ApiResult<Response<Body>> {
    let mut conn = app.db.acquire().await?;
    let user = db::user::DBUser::get(&user_id, &mut conn).await?;
    if let Some(user) = user {
        let storage_key = user
            .avatar
            .ok_or(ApiError::NotFound("avatar".to_string()))?;
        if let Some(client_etag) = headers.get(header::IF_NONE_MATCH) {
            if client_etag.to_str().unwrap_or("") == storage_key {
                return Ok(Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .header(header::ETAG, format!("\"{storage_key}\""))
                    .body(Body::empty())
                    .map_err(|_| ApiError::InternalServerError)?);
            }
        }
        let resp = app
            .s3
            .get_object()
            .bucket(&app.config.s3.bucket)
            .key(&storage_key)
            .send()
            .await;
        let obj = match resp {
            Ok(obj) => Ok(obj),
            Err(e) => match e {
                SdkError::ResponseError(e) => {
                    if e.raw().status() == StatusCode::NOT_FOUND.into() {
                        Err(ApiError::NotFound("avatar".to_string()))
                    } else {
                        Err(ApiError::InternalServerError)
                    }
                }
                _ => Err(ApiError::InternalServerError),
            },
        }?;
        let data = obj
            .body
            .collect()
            .await
            .map_err(|_| ApiError::InternalServerError)?
            .into_bytes();
        let content_type = obj
            .content_type
            .unwrap_or("application/octet-stream".to_string());
        let body = Body::from(data);
        let response = axum::http::Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, content_type)
            .header(header::ETAG, format!("\"{storage_key}\""))
            .header(header::CACHE_CONTROL, "public, max-age=31536000")
            .body(body)
            .map_err(|_| ApiError::InternalServerError)?;
        Ok(response)
    } else {
        Err(ApiError::NotFound("user".to_string()))
    }
}

#[utoipa::path(
    delete,
    path = "",
    tag = "avatar",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ApiErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
async fn delete_avatar(
    State(app): State<AppResources>,
    UserAuth(mut user): UserAuth,
) -> ApiResult<impl IntoResponse> {
    if let Some(ref storage_key) = user.avatar {
        app.s3
            .delete_object()
            .bucket(&app.config.s3.bucket)
            .key(storage_key)
            .send()
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        let mut conn = app.db.acquire().await?;
        DBUser::remove_avatar(&user.id, &mut conn).await?;
        user.avatar = None;
        user.avatar_preview = None;
        user.cache(&app.redis).await?;
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::OK)
    }
}

pub async fn upload_avatar_data(
    mut req: Multipart,
    app: &AppResources,
    key: &str,
) -> ApiResult<String> {
    while let Some(field) = req
        .next_field()
        .await
        .map_err(|_| ApiError::InternalServerError)?
    {
        let name = field.name().ok_or(ApiError::InternalServerError)?;
        if name != key {
            continue;
        }
        let content_type = field.content_type().unwrap_or("");

        // Проверяем MIME
        if !content_type.starts_with("image/") {
            return Err(ApiError::CustomHttp(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported mime type".to_string(),
            ));
        }

        let ext = match content_type {
            "image/png" => "png",
            "image/jpeg" | "image/jpg" => "jpg",
            "image/gif" => "gif",
            _ => {
                return Err(ApiError::CustomHttp(
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    "unsupported mime type".to_string(),
                ));
            }
        };
        let content_type = content_type.to_string();

        let data = field
            .bytes()
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let storage_key = format!("{}.{}", nanoid!(16), ext);

        app.s3
            .put_object()
            .content_type(content_type)
            .bucket(&app.config.s3.bucket)
            .key(&storage_key)
            .body(ByteStream::from(data))
            .send()
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        return Ok(storage_key);
    }
    Err(ApiError::BadRequest(BadRequestError::AvatarMissing))
}

#[derive(OpenApi)]
#[openapi(
    paths(get_avatar, upload_avatar, delete_avatar),
    components(schemas(ApiErrorResponse, UploadAvatar, UnauthotizedErrorResponse)),
    tags((name = "avatar", description = "Upload/Get Avatar for users")),
)]
pub struct AvatarApiDoc;
