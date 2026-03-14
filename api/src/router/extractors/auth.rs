use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    entity::user::UserEntity,
    error::{ApiError, unauthotized::UnauthotizedError},
    jwt,
};

#[derive(Debug, Clone)]
pub struct UserAuth(pub UserEntity);
#[derive(Debug, Clone)]
pub struct AdminAuth(pub Uuid);

impl<S> FromRequestParts<S> for UserAuth
where
    S: Send + Sync,
    AppResources: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //check cached
        let user_auth = parts.extensions.get::<UserAuth>();
        if let Some(user_auth) = user_auth {
            return Ok(user_auth.clone());
        }
        let State(app): State<AppResources> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

        let user_id = jwt::verify(token, &app.config)?;
        let user = app.user_service.get(&user_id).await.map_err(|e| match e {
            ApiError::NotFound(_) => ApiError::Unauthorized(UnauthotizedError::InvalidToken),
            _ => ApiError::InternalServerError,
        })?;
        let user_auth = UserAuth(user);
        parts.extensions.insert(user_auth.clone());
        Ok(user_auth)
    }
}

impl<S> FromRequestParts<S> for AdminAuth
where
    S: Send + Sync,
    AppResources: FromRef<S>,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(app): State<AppResources> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::InternalServerError)?;

        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

        let user_id = jwt::verify(token, &app.config)?;
        //TODO: check if user is admin
        Err(ApiError::Unauthorized(UnauthotizedError::InvalidToken))?;

        Ok(AdminAuth(user_id))
    }
}
