use axum::{Json, Router};
use utoipa::OpenApi;

use crate::{
    app_resources::AppResources,
    error::ApiResult,
    protocol::workspace_member::WorkspaceMemberResponse,
    router::{extractors::{member::MemberFromPath, req_ctx::Ctx, workspace::{WorkspaceAdmin, WorkspaceMember}}, path_params::{MemberPathParams, WorkspacePathParams}},
};

pub fn members_router() -> Router<AppResources> {
    Router::new().route("/list", axum::routing::get(get_list))
        .route("/self", axum::routing::delete(leave_member))
        .route("/{user_id}", axum::routing::get(get_member))
        .route("/{user_id}", axum::routing::delete(delete_member))
}

#[utoipa::path(
    get,
    path = "/list",
    tag = "workspace-member",
    description = "Get list of workspace members",
    responses(
        (status = 200, description = "OK", body = Vec<WorkspaceMemberResponse>),
    ),
    params(
        ("workspace_id" = String, Path, description = "Workspace ID"),
    ),
)]
/// Workspace members
pub async fn get_list(
    ctx: Ctx,
    wa: WorkspaceMember<WorkspacePathParams>,
) -> ApiResult<Json<Vec<WorkspaceMemberResponse>>> {
    let members = ctx.workspace_member_service().get_list(&wa.workspace.id).await?;
    Ok(Json(members.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "workspace-member",
    description = "Get workspace member",
    responses(
        (status = 200, description = "OK", body = WorkspaceMemberResponse),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("user_id" = Uuid, Path, description = "User ID"),
    ),
)]
/// Get member
pub async fn get_member(
    _: WorkspaceMember<MemberPathParams>,
    MemberFromPath{member,..}:MemberFromPath<MemberPathParams>,
) -> ApiResult<Json<WorkspaceMemberResponse>> {
    Ok(Json(member.into()))
}

#[utoipa::path(
    delete,
    path = "/{user_id}",
    tag = "workspace-member",
    description = "Delete member from workspace",
    responses(
        (status = 200, description = "OK"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("user_id" = Uuid, Path, description = "User ID"),
    ),
)]
/// Delete member
pub async fn delete_member(
    ctx: Ctx,
    wa: WorkspaceAdmin<MemberPathParams>,
    MemberFromPath{member,..}:MemberFromPath<MemberPathParams>,
) -> ApiResult<()> {
    ctx.workspace_member_service()
        .delete(&wa.workspace.id, &member.user.id)
        .await?;
    Ok(())
}

#[utoipa::path(
    delete,
    path = "/self",
    tag = "workspace-member",
    description = "Leave from workspace, by self account",
    responses(
        (status = 200, description = "OK"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Leave from workspace
pub async fn leave_member(
    ctx: Ctx,
    wa: WorkspaceMember<WorkspacePathParams>,
) -> ApiResult<()> {
    ctx.workspace_member_service()
        .delete_self(&wa.workspace.id, &wa.member.id)
        .await?;
    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(get_list, get_member, delete_member, leave_member),
    components(),
    tags((name = "workspace-member", description = "Workspace members")),
)]
pub struct WorkspaceMemberApiDoc;
