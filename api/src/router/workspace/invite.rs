use axum::{Json, Router, extract::Query};
use utoipa::OpenApi;

use crate::{
    app_resources::AppResources,
    error::ApiResult,
    protocol::{
        error::{BadRequestErrorResponse, ForbiddenErrorResponse, UnauthotizedErrorResponse},
        user::{SearchUserRequest, UserListItemResponse},
        workspace_invite::{CreateInviteRequest, DeleteInviteRequest, WorkspaceInviteResponse},
    },
    router::{
        extractors::{
            auth::UserAuth,
            req_ctx::Ctx,
            workspace::{WorkspaceFromPath, WorkspaceWithAdmin},
        },
        path_params::WorkspacePathParams,
    },
};

pub fn invite_router() -> Router<AppResources> {
    Router::new()
        .route("/list", axum::routing::get(get_workspace_invitations))
        .route("/list-for-invite", axum::routing::get(get_users_for_invite))
        .route("/accept", axum::routing::post(accept_invite))
        .route("/", axum::routing::post(create_invite))
        .route("/", axum::routing::delete(delete_invite))
}

#[utoipa::path(
    get,
    path = "/list",
    tag = "workspace-invite",
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
    responses(
        (status = 200, description = "OK", body = Vec<WorkspaceInviteResponse>),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Get invitations
pub async fn get_workspace_invitations(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
) -> ApiResult<Json<Vec<WorkspaceInviteResponse>>> {
    let invitations = ctx
        .workspace_invite_service()
        .get_workspace_invitations(&wa.workspace.id)
        .await?;
    Ok(Json(invitations.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "",
    tag = "workspace-invite",
    request_body = CreateInviteRequest,
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
    responses(
        (status = 200, description = "OK", body = WorkspaceInviteResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
    ),
)]
/// Invite user to workspace
pub async fn create_invite(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    Json(req): Json<CreateInviteRequest>,
) -> ApiResult<Json<WorkspaceInviteResponse>> {
    let invite = ctx
        .workspace_invite_service()
        .create(&wa.workspace.id, &req.user_id)
        .await?;
    Ok(Json(invite.into()))
}

#[utoipa::path(
    delete,
    path = "",
    tag = "workspace-invite",
    request_body = DeleteInviteRequest,
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
        (status = 404, description = "Not Found"),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
    ),
)]
/// Delete invite to workspace
pub async fn delete_invite(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    Json(req): Json<DeleteInviteRequest>,
) -> ApiResult<()> {
    ctx.workspace_invite_service()
        .delete(&wa.workspace.id, &req.user_id)
        .await?;
    Ok(())
}

#[utoipa::path(
    get,
    path = "/list-for-invite",
    tag = "workspace-invite",
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        SearchUserRequest,
    ),
    responses(
        (status = 200, description = "OK", body = Vec<UserListItemResponse>),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
    ),
)]
/// Search users
pub async fn get_users_for_invite(
    ctx: Ctx,
    w: WorkspaceWithAdmin<WorkspacePathParams>,
    Query(req): Query<SearchUserRequest>,
) -> ApiResult<Json<Vec<UserListItemResponse>>> {
    let users = ctx
        .workspace_invite_service()
        .users_for_invite_search(req.search, req.limit, req.offset, &w.workspace.id)
        .await?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    post,
    path = "/accept",
    tag = "workspace-invite",
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
        (status = 403, description = "Forbidden", body = ForbiddenErrorResponse),
        (status = 404, description = "Not Found"),
    ),
)]
/// Accept invite
pub async fn accept_invite(
    ctx: Ctx,
    w: WorkspaceFromPath<WorkspacePathParams>,
    UserAuth(user): UserAuth,
) -> ApiResult<()> {
    ctx.workspace_invite_service()
        .accept_invite(&w.workspace.id, &user.id)
        .await?;
    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_workspace_invitations,
        create_invite,
        delete_invite,
        get_users_for_invite,
        accept_invite,
    ),
    components(schemas()),
    tags((name = "workspace-invite", description = "Workspace Invite")),
)]
pub struct WorkspaceInviteApiDoc;
