use axum::{Json, Router};
use utoipa::OpenApi;

use crate::{
    app_resources::AppResources,
    entity::workspace::CreateWorkspaceEntity,
    error::ApiResult,
    protocol::{
        error::BadRequestErrorResponse,
        workspace::{
            ChangeAdminRequest, CreateWorkspaceRequest, UpdateWorkspaceRequest, WorkspaceResponse,
            WorkspaceWithStatusesAndTagsResponse,
        },
    },
    router::{
        extractors::{
            app_json::AppJson,
            auth::UserAuth,
            req_ctx::Ctx,
            workspace::{WorkspaceMember, WorkspaceWithAdmin},
        },
        path_params::WorkspacePathParams,
        workspace::{
            avatar::workspace_avatar_router, invite::invite_router, statuses::statuses_router,
            tags::tags_router,
        },
    },
};

pub mod avatar;
pub mod invite;
pub mod statuses;
pub mod tags;

pub fn workspace_router() -> Router<AppResources> {
    Router::new()
        .route("/create", axum::routing::post(create))
        .route("/{workspace_id}", axum::routing::get(get))
        .route("/{workspace_id}", axum::routing::patch(update))
        .route(
            "/{workspace_id}/change-admin",
            axum::routing::post(change_admin),
        )
        .nest("/{workspace_id}/avatar", workspace_avatar_router())
        .nest("/{workspace_id}/invite", invite_router())
        .nest("/{workspace_id}/status", statuses_router())
        .nest("/{workspace_id}/tag", tags_router())
}

#[utoipa::path(
    get,
    path = "/{workspace_id}",
    tag = "workspace",
    responses(
        (status = 200, description = "OK", body = WorkspaceResponse),
        (status = 404, description = "Not Found"),
    ),
)]
/// Get workspace
pub async fn get(wa: WorkspaceMember<WorkspacePathParams>) -> ApiResult<Json<WorkspaceResponse>> {
    Ok(Json(wa.workspace.into()))
}

#[utoipa::path(
    post,
    path = "/create",
    tag = "workspace",
    request_body = CreateWorkspaceRequest,
    responses(
        (status = 200, description = "OK", body = WorkspaceWithStatusesAndTagsResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
    ),
)]
/// Create new workspace
pub async fn create(
    ctx: Ctx,
    UserAuth(user): UserAuth,
    AppJson(req): AppJson<CreateWorkspaceRequest>,
) -> ApiResult<Json<WorkspaceWithStatusesAndTagsResponse>> {
    let e: CreateWorkspaceEntity = req.try_into()?;
    let (workspace, statuses, tags) = ctx.workspace_service().create(&user, e).await?;
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

#[utoipa::path(
    patch,
    path = "/{workspace_id}",
    tag = "workspace",
    request_body = UpdateWorkspaceRequest,
    responses(
        (status = 200, description = "OK", body = WorkspaceResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 404, description = "Not Found"),
    ),
)]
/// Update workspace
pub async fn update(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    AppJson(req): AppJson<UpdateWorkspaceRequest>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let workspace = ctx
        .workspace_service()
        .update(&wa.workspace.id, req.name.into())
        .await?;
    Ok(Json(workspace.into()))
}

#[utoipa::path(
    post,
    path = "/{workspace_id}/change-admin",
    tag = "workspace",
    request_body = ChangeAdminRequest,
    responses(
        (status = 200, description = "OK", body = WorkspaceResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 404, description = "Not Found"),
    ),
)]
/// Change workspace admin
pub async fn change_admin(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    AppJson(req): AppJson<ChangeAdminRequest>,
) -> ApiResult<Json<WorkspaceResponse>> {
    let workspace = ctx
        .workspace_service()
        .change_admin(&wa.workspace.id, &req.admin)
        .await?;
    Ok(Json(workspace.into()))
}

use self::avatar::WorkspaceAvatarApiDoc;
use self::invite::WorkspaceInviteApiDoc;
use self::statuses::StatusesApiDoc;
use self::tags::TagsApiDoc;

#[derive(OpenApi)]
#[openapi(
    paths(get,create,update, change_admin),
    components(schemas(CreateWorkspaceRequest, WorkspaceWithStatusesAndTagsResponse)),
    tags((name = "workspace", description = "Workspace")),
    nest(
        (path = "/{workspace_id}/avatar", api = WorkspaceAvatarApiDoc),
        (path = "/{workspace_id}/invite", api = WorkspaceInviteApiDoc),
        (path = "/{workspace_id}/status", api = StatusesApiDoc),
        (path = "/{workspace_id}/tag", api = TagsApiDoc),
    ),
)]
pub struct WorkspaceApiDoc;
