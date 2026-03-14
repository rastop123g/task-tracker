use axum::{
    extract::{FromRef, FromRequestParts, Path, State},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::{
    app_resources::AppResources,
    entity::{user::UserEntity, workspace::WorkspaceEntity},
    error::{ApiError, bad_request::BadRequestError, forbidden::ForbiddenError},
    router::{extractors::auth::UserAuth, path_params::WorkspaceIdFromPathParams},
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
        let State(app): State<AppResources> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        let Path(params): Path<T> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::BadRequest(BadRequestError::BadPathParams))?;
        let workspace_id = params.workspace_id();
        let workspace = app.workspace_service.get(&workspace_id).await?;
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
pub struct WorkspaceMember {
    pub workspace: WorkspaceEntity,
    pub member: UserEntity,
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
