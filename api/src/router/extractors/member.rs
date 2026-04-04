use axum::extract::{FromRef, FromRequestParts, Path};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{app_resources::AppResources, entity::workspace_member::WorkspaceMemberEntity, error::ApiError, router::{extractors::req_ctx::Ctx, path_params::{UserIdFromPathParams, WorkspaceIdFromPathParams}}};

pub struct MemberFromPath<P: UserIdFromPathParams + WorkspaceIdFromPathParams> {
    pub member: WorkspaceMemberEntity,
    _marker: std::marker::PhantomData<P>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMemberFromPath(WorkspaceMemberEntity);

impl<S, T> FromRequestParts<S> for MemberFromPath<T>
    where
        S: Send + Sync,
        AppResources: FromRef<S>,
        T: UserIdFromPathParams + WorkspaceIdFromPathParams + Send + Sync + DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cache
        let member = parts.extensions.get::<CachedMemberFromPath>();
        if let Some(member) = member {
            return Ok(MemberFromPath {
                member: member.0.clone(),
                _marker: std::marker::PhantomData,
            });
        }
        let ctx: Ctx = crate::router::extractors::req_ctx::Ctx::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let Path(params): Path<T> = Path::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::BadRequest(crate::error::bad_request::BadRequestError::BadPathParams))?;
        let user_id = params.user_id();
        let workspace_id = params.workspace_id();
        let member = ctx
            .workspace_member_service()
            .get(&workspace_id, &user_id)
            .await?;
        //cache member
        parts
            .extensions
            .insert(CachedMemberFromPath(member.clone()));
        Ok(MemberFromPath {
            member,
            _marker: std::marker::PhantomData,
        })
    }
}
