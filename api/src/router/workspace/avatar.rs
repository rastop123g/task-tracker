use aws_sdk_s3::error::SdkError;
use axum::{
    Json, Router,
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::{HeaderMap, Response, StatusCode, header},
    routing::{delete, post},
};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    db::workspace::DBWorkspace,
    entity::workspace::WorkspaceEntity,
    error::{ApiError, ApiResult},
    protocol::{
        error::{BadRequestErrorResponse, ForbiddenErrorResponse, UnauthotizedErrorResponse},
        workspace::WorkspaceResponse,
    },
    router::{
        avatar::UploadAvatar, extractors::workspace::WorkspaceWithAdmin,
        path_params::WorkspacePathParams,
    },
};

pub fn workspace_avatar_router() -> Router<crate::app_resources::AppResources> {
    Router::new()
        .route(
            "/",
            post(upload_workspace_avatar).layer(DefaultBodyLimit::max(1024 * 1024 * 20)),
        )
        .route("/", delete(delete_workspace_avatar))
        .route("/", axum::routing::get(get_avatar))
}

#[utoipa::path(
    post,
    path = "",
    description = "Upload workspace avatar, only admin can upload",
    tag = "workspace_avatar",
    request_body(content_type = "multipart/form-data", content = UploadAvatar),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
    responses(
        (status = 200, description = "OK", body = WorkspaceResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
    ),
)]
/// Upload workspace avatar
async fn upload_workspace_avatar(
    State(app): State<AppResources>,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    req: Multipart,
) -> ApiResult<Json<WorkspaceResponse>> {
    let mut workspace = wa.workspace;
    let storage_key = crate::router::avatar::upload_avatar_data(req, &app, "avatar").await?;
    let prev_avatar = workspace.avatar.clone();
    workspace.avatar = Some(storage_key.clone());
    let mut conn = app.db.acquire().await?;
    let res = DBWorkspace::from(workspace)
        .update_avatar(&mut conn)
        .await?;
    if let Some(workspace) = res {
        if let Some(ref prev_avatar) = prev_avatar {
            app.s3
                .delete_object()
                .bucket(&app.config.s3.bucket)
                .key(prev_avatar)
                .send()
                .await
                .map_err(|_| ApiError::InternalServerError)?;
        }
        let workspace: WorkspaceEntity = workspace.into();
        Ok(Json(workspace.into()))
    } else {
        Err(ApiError::NotFound("workspace".to_string()))
    }
}

#[utoipa::path(
    delete,
    path = "",
    description = "Delete workspace avatar, only admin can delete",
    tag = "workspace_avatar",
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Delete workspace avatar
async fn delete_workspace_avatar(
    State(app): State<AppResources>,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let workspace = wa.workspace;
    if let Some(ref avatar) = workspace.avatar {
        app.s3
            .delete_object()
            .bucket(&app.config.s3.bucket)
            .key(avatar)
            .send()
            .await
            .map_err(|_| ApiError::InternalServerError)?;
    }
    let mut conn = app.db.acquire().await?;
    let res = DBWorkspace::from(workspace).reset_avatar(&mut conn).await?;
    if let Some(workspace) = res {
        let workspace: WorkspaceEntity = workspace.into();
        Ok(Json(workspace.into()))
    } else {
        Err(ApiError::NotFound("workspace".to_string()))
    }
}

#[utoipa::path(
    get,
    path = "",
    description = "Get workspace avatar, public, includes etag",
    tag = "workspace_avatar",
    responses(
        (status = 200, description = "OK"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Get workspace avatar
async fn get_avatar(
    State(app): State<AppResources>,
    Path(workspace_id): Path<Uuid>,
    headers: HeaderMap,
) -> ApiResult<Response<Body>> {
    let mut conn = app.db.acquire().await?;
    let workspace = DBWorkspace::get(&workspace_id, &mut conn).await?;
    if let Some(workspace) = workspace {
        let storage_key = workspace
            .avatar
            .ok_or(ApiError::NotFound("avatar".to_string()))?;
        if let Some(client_etag) = headers.get(header::IF_NONE_MATCH)
            && client_etag.to_str().unwrap_or("") == storage_key {
                return Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .header(header::ETAG, format!("\"{storage_key}\""))
                    .body(Body::empty())
                    .map_err(|_| ApiError::InternalServerError);
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
        Err(ApiError::NotFound("workspace".to_string()))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(upload_workspace_avatar, delete_workspace_avatar, get_avatar),
    components(schemas(UploadAvatar, UnauthotizedErrorResponse, BadRequestErrorResponse, ForbiddenErrorResponse)),
    tags((name = "workspace_avatar", description = "Workspace Avatar")),
)]
pub struct WorkspaceAvatarApiDoc;
