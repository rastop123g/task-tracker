use axum::{
    extract::{FromRef, FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::{
    app_resources::AppResources,
    entity::tag::TagEntity,
    error::ApiError,
    router::{
        extractors::{req_ctx::Ctx, workspace::WorkspaceFromPath},
        path_params::{TagIdFromPathParams, WorkspaceIdFromPathParams},
    },
};

#[derive(Debug, Clone)]
pub struct TagFromPath<P: TagIdFromPathParams + WorkspaceIdFromPathParams + DeserializeOwned> {
    pub tag: TagEntity,
    _marker: std::marker::PhantomData<P>,
}

#[derive(Debug, Clone)]
pub struct CachedTagFromPath(TagEntity);

impl<S, T> FromRequestParts<S> for TagFromPath<T>
where
    S: Send + Sync,
    AppResources: FromRef<S>,
    T: TagIdFromPathParams + WorkspaceIdFromPathParams + DeserializeOwned + Send + Sync,
{
    type Rejection = crate::error::ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cache
        let tag = parts.extensions.get::<CachedTagFromPath>();
        if let Some(tag) = tag {
            return Ok(TagFromPath {
                tag: tag.0.clone(),
                _marker: std::marker::PhantomData,
            });
        }
        //NOTE: В целом как будто можно сделать и так, чтобы методы работы с тегами были без
        //упоминания workspace, т.к. принадлежноть к workspace роли не играет, но в таком случае
        //мы автоматически блокируем работу с тегами в удаленных workspace что может иметь смысл
        let WorkspaceFromPath { workspace, .. }: WorkspaceFromPath<T> =
            WorkspaceFromPath::from_request_parts(parts, state).await?;

        let ctx: Ctx = Ctx::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;
        let Path(params): Path<T> = Path::from_request_parts(parts, state).await.map_err(|_| {
            crate::error::ApiError::BadRequest(
                crate::error::bad_request::BadRequestError::BadPathParams,
            )
        })?;
        let tag_id = params.tag_id();
        let tag = ctx.tag_service().get(&tag_id).await?;
        if tag.workspace != workspace.id {
            return Err(crate::error::ApiError::NotFound("tag".to_string()));
        }
        //cache tag
        parts.extensions.insert(CachedTagFromPath(tag.clone()));
        Ok(TagFromPath {
            tag,
            _marker: std::marker::PhantomData,
        })
    }
}
