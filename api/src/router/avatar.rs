use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json, Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::{HeaderMap, StatusCode, header},
    middleware,
    response::{IntoResponse, Response},
    routing::post,
};
use nanoid::nanoid;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    app_resources::AppResources, db, error::{ApiError, ApiErrorResponse, ApiResult}, protocol::error::UnauthotizedErrorResponse, router::auth::auth_middleware
};

pub fn avatar_router(res: AppResources) -> Router<AppResources> {
    Router::new()
        .route(
            "/{user_id}",
            post(upload_avatar).layer(middleware::from_fn_with_state(res, auth_middleware)),
        )
        .route("/{user_id}", axum::routing::get(get_avatar))
        //TODO: delete avatar
        .layer(DefaultBodyLimit::max(1024 * 1024 * 20)) // 20MB
}

#[derive(ToSchema)]
pub struct UploadAvatar {
    /// File to upload
    #[schema(value_type = String, format = Binary)]
    pub avatar: Vec<u8>,
}

#[utoipa::path(
    post,
    path = "/{user_id}",
    tag = "avatar",
    request_body(content_type = "multipart/form-data", content = UploadAvatar),
    security(("api_key" = [])),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = ApiErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ApiErrorResponse),
    ),
    params(
        ("user_id" = Uuid, Path, description = "User ID"),
    ),
)]
async fn upload_avatar(
    State(app): State<AppResources>,
    Path(user_id): Path<Uuid>,
    mut req: Multipart,
) -> ApiResult<impl IntoResponse> {
    //TODO: check user_id
    //TODO: check user_id == auth_user_id
    //TODO: Remove old avatar from s3
    while let Some(field) = req
        .next_field()
        .await
        .map_err(|_| ApiError::InternalServerError)?
    {
        let name = field.name().ok_or(ApiError::InternalServerError)?;
        if name != "avatar" {
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

        let mut conn = app.db.acquire().await?;
        let upd_user = db::user::DBUpdateUser {
            avatar: Some(storage_key.clone()),
            avatar_preview: Some(storage_key.clone()),
            ..Default::default()
        };
        upd_user.update(&user_id, &mut conn).await?;
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
        let obj = app
            .s3
            .get_object()
            .bucket(&app.config.s3.bucket)
            .key(&storage_key)
            .send()
            .await
            .map_err(|_| ApiError::InternalServerError)?;
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

#[derive(OpenApi)]
#[openapi(
    paths(get_avatar, upload_avatar),
    components(schemas(ApiErrorResponse, UploadAvatar, UnauthotizedErrorResponse)),
    tags((name = "avatar", description = "Upload/Get Avatar for users")),
)]
pub struct AvatarApiDoc;
