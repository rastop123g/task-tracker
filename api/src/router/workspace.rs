use axum::{Json, Router, extract::State};
use utoipa::OpenApi;

use crate::{
    app_resources::AppResources,
    entity::workspace::CreateWorkspaceEntity,
    error::ApiResult,
    protocol::{
        error::ValidationErrorResponse,
        workspace::{CreateWorkspaceRequest, WorkspaceWithStatusesAndTagsResponse},
    },
    router::{
        extractors::auth::UserAuth,
        workspace::{avatar::workspace_avatar_router, invite::invite_router},
    },
    utils::{AppTrim, FieldValidate},
};

pub mod avatar;
pub mod invite;

pub fn workspace_router() -> Router<AppResources> {
    Router::new()
        .route("/create", axum::routing::post(create))
        .nest("/{workspace_id}/avatar", workspace_avatar_router())
        .nest("/{workspace_id}/invite", invite_router())
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "workspace",
    request_body = CreateWorkspaceRequest,
    responses(
        (status = 200, description = "OK", body = WorkspaceWithStatusesAndTagsResponse),
        (status = 400, description = "Bad Request", body = ValidationErrorResponse),
    ),
)]
/// Create new workspace
pub async fn create(
    State(app): State<AppResources>,
    UserAuth(user): UserAuth,
    Json(mut req): Json<CreateWorkspaceRequest>,
) -> ApiResult<Json<WorkspaceWithStatusesAndTagsResponse>> {
    req.app_trim();
    req.field_validate()?;
    let e: CreateWorkspaceEntity = req.try_into()?;
    let (workspace, statuses, tags) = app.workspace_service.create(&user, e).await?;
    Ok(Json(WorkspaceWithStatusesAndTagsResponse {
        id: workspace.id,
        name: workspace.name,
        admin: workspace.admin,
        created_at: workspace.created_at,
        updated_at: workspace.updated_at,
        deleted_at: workspace.deleted_at,
        statuses: statuses.into_iter().map(Into::into).collect(),
        tags: tags.into_iter().map(Into::into).collect(),
    }))
}

use self::avatar::WorkspaceAvatarApiDoc;
use self::invite::WorkspaceInviteApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(create),
    components(schemas(CreateWorkspaceRequest, WorkspaceWithStatusesAndTagsResponse)),
    tags((name = "workspace", description = "Workspace")),
    nest(
        (path = "/{workspace_id}/avatar", api = WorkspaceAvatarApiDoc),
        (path = "/{workspace_id}/invite", api = WorkspaceInviteApiDoc),
    ),
)]
pub struct WorkspaceApiDoc;
