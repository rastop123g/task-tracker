use axum::Json;

use crate::{
    app_resources::AppResources,
    error::ApiResult,
    protocol::{
        error::BadRequestErrorResponse,
        tag::{CreateTagRequest, TagResponse, UpdateTagRequest},
    },
    router::{
        extractors::{
            app_json::AppJson,
            req_ctx::Ctx,
            tag::TagFromPath,
            workspace::{WorkspaceMember, WorkspaceWithAdmin},
        },
        path_params::{TagPathParams, WorkspacePathParams},
    },
};

pub fn tags_router() -> axum::Router<AppResources> {
    axum::Router::new()
        .route("/list", axum::routing::get(get_tags))
        .route("/", axum::routing::post(create_tag))
        .route("/{tag_id}", axum::routing::get(get_tag))
        .route("/{tag_id}", axum::routing::patch(update_tag))
        .route("/{tag_id}", axum::routing::delete(delete_tag))
}

#[utoipa::path(
    get,
    path = "/list",
    description = "Get tags for workspace, only members can get",
    tag = "workspace-tags",
    responses(
        (status = 200, description = "OK", body = Vec<TagResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Get tags for workspace
pub async fn get_tags(
    ctx: Ctx,
    wm: WorkspaceMember<WorkspacePathParams>,
) -> ApiResult<Json<Vec<TagResponse>>> {
    let tags = ctx.tag_service().get_list(&wm.workspace.id).await?;
    Ok(Json(tags.into_iter().map(Into::into).collect()))
}

#[utoipa::path(
    get,
    path = "/{tag_id}",
    tag = "workspace-tags",
    responses(
        (status = 200, description = "OK", body = TagResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("tag_id" = Uuid, Path, description = "Tag ID"),
    ),
)]
/// Get tag
pub async fn get_tag(
    _wm: WorkspaceMember<WorkspacePathParams>,
    t: TagFromPath<TagPathParams>,
) -> ApiResult<Json<TagResponse>> {
    Ok(Json(t.tag.into()))
}

#[utoipa::path(
    post,
    path = "",
    description = "Create tag, only admin can create",
    tag = "workspace-tags",
    request_body = CreateTagRequest,
    responses(
        (status = 200, description = "OK", body = TagResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
    ),
)]
/// Create tag
pub async fn create_tag(
    ctx: Ctx,
    wm: WorkspaceWithAdmin<WorkspacePathParams>,
    AppJson(req): AppJson<CreateTagRequest>,
) -> ApiResult<Json<TagResponse>> {
    let tag = ctx
        .tag_service()
        .create(req.into(), &wm.workspace.id)
        .await?;
    Ok(Json(tag.into()))
}

#[utoipa::path(
    patch,
    path = "/{tag_id}",
    description = "Update tag, only admin can update",
    tag = "workspace-tags",
    request_body = UpdateTagRequest,
    responses(
        (status = 200, description = "OK", body = TagResponse),
        (status = 400, description = "Bad Request", body = BadRequestErrorResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("tag_id" = Uuid, Path, description = "Tag ID"),
    ),
)]
/// Update tag
pub async fn update_tag(
    ctx: Ctx,
    _wa: WorkspaceWithAdmin<TagPathParams>,
    t: TagFromPath<TagPathParams>,
    AppJson(req): AppJson<UpdateTagRequest>,
) -> ApiResult<Json<TagResponse>> {
    let tag = ctx.tag_service().update(&t.tag.id, req.into()).await?;
    Ok(Json(tag.into()))
}

#[utoipa::path(
    delete,
    path = "/{tag_id}",
    description = "Delete tag, only admin can delete",
    tag = "workspace-tags",
    responses(
        (status = 200, description = "OK", body = TagResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Not Found"),
    ),
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("tag_id" = Uuid, Path, description = "Tag ID"),
    ),
)]
/// Delete tag
pub async fn delete_tag(
    ctx: Ctx,
    _wa: WorkspaceWithAdmin<TagPathParams>,
    t: TagFromPath<TagPathParams>,
) -> ApiResult<Json<TagResponse>> {
    let tag = ctx.tag_service().delete(&t.tag.id).await?;
    Ok(Json(tag.into()))
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_tags, get_tag, create_tag, update_tag, delete_tag),
    components(),
    tags((name = "workspace-tags", description = "Tags")),
)]
pub struct TagsApiDoc;
