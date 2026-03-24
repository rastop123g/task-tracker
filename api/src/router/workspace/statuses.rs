use axum::Json;

use crate::{
    app_resources::AppResources,
    error::ApiResult,
    protocol::{
        error::BadRequestErrorResponse,
        status::{CreateStatusRequest, StatusResponse, UpdateStatusRequest},
    },
    router::{
        extractors::{
            app_json::AppJson,
            req_ctx::Ctx,
            status::StatusFromPath,
            workspace::{WorkspaceMember, WorkspaceWithAdmin},
        },
        path_params::{StatusPathParams, WorkspacePathParams},
    },
};

pub fn statuses_router() -> axum::Router<AppResources> {
    axum::Router::new()
        .route("/list", axum::routing::get(get_statuses))
        .route("/", axum::routing::post(create_status))
        .route("/{status_id}", axum::routing::get(get_status))
        .route("/{status_id}", axum::routing::patch(update_status))
        .route("/{status_id}", axum::routing::delete(delete_status))
}

#[utoipa::path(
    get,
    path = "/list",
    description = "Get statuses for workspace, only members can get",
    tag = "workspace-statuses",
    responses(
        (status = 200, description = "OK", body = Vec<StatusResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Get statuses for workspace
pub async fn get_statuses(
    ctx: Ctx,
    wm: WorkspaceMember<WorkspacePathParams>,
) -> ApiResult<Json<Vec<StatusResponse>>> {
    let statuses = ctx
        .workspace_statuses_service()
        .get_statuses(&wm.workspace.id)
        .await?;
    Ok(Json(statuses.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/{status_id}",
    tag = "workspace-statuses",
    responses(
        (status = 200, description = "OK", body = StatusResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("status_id" = Uuid, Path, description = "Status ID"),
    ),
)]
/// Get status
pub async fn get_status(
    _wa: WorkspaceMember<StatusPathParams>,
    s: StatusFromPath<StatusPathParams>,
) -> ApiResult<Json<StatusResponse>> {
    Ok(Json(s.status.into()))
}

#[utoipa::path(
    post,
    path = "",
    description = "Create status, only admin can create",
    tag = "workspace-statuses",
    request_body = CreateStatusRequest,
    responses(
        (status = 200, description = "OK", body = StatusResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Create status
pub async fn create_status(
    ctx: Ctx,
    wa: WorkspaceWithAdmin<WorkspacePathParams>,
    AppJson(req): AppJson<CreateStatusRequest>,
) -> ApiResult<Json<StatusResponse>> {
    let status = ctx
        .workspace_statuses_service()
        .create_status(&wa.workspace.id, req.into())
        .await?;
    Ok(Json(status.into()))
}

#[utoipa::path(
    patch,
    path = "/{status_id}",
    description = "Update status, only admin can update",
    tag = "workspace-statuses",
    request_body = UpdateStatusRequest,
    responses(
        (status = 200, description = "OK", body = StatusResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("status_id" = Uuid, Path, description = "Status ID"),
    ),
)]
/// Update status
pub async fn update_status(
    ctx: Ctx,
    _wa: WorkspaceWithAdmin<StatusPathParams>,
    s: StatusFromPath<StatusPathParams>,
    AppJson(req): AppJson<UpdateStatusRequest>,
) -> ApiResult<Json<StatusResponse>> {
    let status = ctx
        .workspace_statuses_service()
        .update_status(&s.status.id, req.into())
        .await?;
    Ok(Json(status.into()))
}

#[utoipa::path(
    delete,
    path = "/{status_id}",
    description = "Delete status, only admin can delete",
    tag = "workspace-statuses",
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("status_id" = Uuid, Path, description = "Status ID"),
    ),
)]
/// Delete status
pub async fn delete_status(
    ctx: Ctx,
    _wa: WorkspaceWithAdmin<StatusPathParams>,
    s: StatusFromPath<StatusPathParams>,
) -> ApiResult<()> {
    ctx.workspace_statuses_service()
        .delete_status(&s.status.id)
        .await?;
    Ok(())
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_statuses, get_status, create_status, update_status, delete_status),
    components(),
    tags((name = "workspace-statuses", description = "Statuses")),
)]
pub struct StatusesApiDoc;
