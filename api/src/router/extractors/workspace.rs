use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::{
    app_resources::AppResources,
    entity::{user::UserEntity, workspace::WorkspaceEntity},
    error::{ApiError, bad_request::BadRequestError, forbidden::ForbiddenError},
    router::{
        extractors::{auth::UserAuth, req_ctx::Ctx},
        path_params::WorkspaceIdFromPathParams,
    },
};

#[derive(Debug, Clone)]
struct CachedWorkspaceFromPath(WorkspaceEntity);

#[derive(Debug, Clone)]
pub struct WorkspaceFromPath<T: WorkspaceIdFromPathParams> {
    pub workspace: WorkspaceEntity,
    _marker: std::marker::PhantomData<T>,
}

impl<S, T> FromRequestParts<S> for WorkspaceFromPath<T>
where
    S: Send + Sync,
    AppResources: FromRef<S>,
    T: WorkspaceIdFromPathParams + Send + Sync + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cache
        let workspace = parts.extensions.get::<CachedWorkspaceFromPath>();
        if let Some(workspace) = workspace {
            return Ok(WorkspaceFromPath {
                workspace: workspace.0.clone(),
                _marker: std::marker::PhantomData,
            });
        }
        let ctx: Ctx = Ctx::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        let Path(params): Path<T> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::BadRequest(BadRequestError::BadPathParams))?;
        let workspace_id = params.workspace_id();
        let workspace = ctx.workspace_service().get(&workspace_id).await?;
        //cache workspace
        parts
            .extensions
            .insert(CachedWorkspaceFromPath(workspace.clone()));
        Ok(WorkspaceFromPath {
            workspace,
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WorkspaceWithAdmin<P> {
    pub workspace: WorkspaceEntity,
    pub admin: UserEntity,
    _phantom: std::marker::PhantomData<P>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceMember<P> {
    pub workspace: WorkspaceEntity,
    pub member: UserEntity,
    _phantom: std::marker::PhantomData<P>,
}

impl<S, P> FromRequestParts<S> for WorkspaceWithAdmin<P>
where
    S: Send + Sync,
    AppResources: FromRef<S>,
    P: WorkspaceIdFromPathParams + Send + Sync + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let UserAuth(user): UserAuth = UserAuth::from_request_parts(parts, state).await?;
        let WorkspaceFromPath { workspace, .. }: WorkspaceFromPath<P> =
            WorkspaceFromPath::from_request_parts(parts, state).await?;
        if workspace.admin != user.id {
            return Err(ApiError::Forbidden(ForbiddenError::WorkspaceAdminOnly));
        }
        Ok(WorkspaceWithAdmin {
            workspace,
            admin: user,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<S, P> FromRequestParts<S> for WorkspaceMember<P>
where
    S: Send + Sync,
    AppResources: FromRef<S>,
    P: WorkspaceIdFromPathParams + Send + Sync + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: Ctx = Ctx::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        let UserAuth(user): UserAuth = UserAuth::from_request_parts(parts, state).await?;
        let WorkspaceFromPath { workspace, .. }: WorkspaceFromPath<P> =
            WorkspaceFromPath::from_request_parts(parts, state).await?;
        let is_member = ctx
            .workspace_member_service()
            .check_member(&workspace.id, &user.id)
            .await?;
        if !is_member {
            return Err(ApiError::Forbidden(ForbiddenError::UserNotMember));
        }
        Ok(WorkspaceMember {
            workspace,
            member: user,
            _phantom: std::marker::PhantomData,
        })
    }
}
