use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::{
    app_resources::AppResources,
    entity::status::StatusEntity,
    error::{ApiError, bad_request::BadRequestError},
    router::{
        extractors::{req_ctx::Ctx, workspace::WorkspaceFromPath},
        path_params::{StatusIdFromPathParams, WorkspaceIdFromPathParams},
    },
};

#[derive(Debug, Clone)]
pub struct StatusFromPath<P: StatusIdFromPathParams + WorkspaceIdFromPathParams> {
    pub status: StatusEntity,
    _marker: std::marker::PhantomData<P>,
}

#[derive(Debug, Clone)]
pub struct CachedStatusFromPath(StatusEntity);

impl<S, T> FromRequestParts<S> for StatusFromPath<T>
where
    S: Send + Sync,
    AppResources: FromRef<S>,
    T: StatusIdFromPathParams + WorkspaceIdFromPathParams + Send + Sync + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cache
        let status = parts.extensions.get::<CachedStatusFromPath>();
        if let Some(status) = status {
            return Ok(StatusFromPath {
                status: status.0.clone(),
                _marker: std::marker::PhantomData,
            });
        }
        let ctx: Ctx = Ctx::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let WorkspaceFromPath { workspace, .. }: WorkspaceFromPath<T> =
            WorkspaceFromPath::from_request_parts(parts, state).await?;

        let Path(params): Path<T> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::BadRequest(BadRequestError::BadPathParams))?;
        let status_id = params.status_id();
        let status = ctx
            .workspace_statuses_service()
            .get_status(&status_id)
            .await?;
        if status.workspace != workspace.id {
            return Err(ApiError::NotFound("status".to_string()));
        }
        //cache status
        parts
            .extensions
            .insert(CachedStatusFromPath(status.clone()));
        Ok(StatusFromPath {
            status,
            _marker: std::marker::PhantomData,
        })
    }
}
