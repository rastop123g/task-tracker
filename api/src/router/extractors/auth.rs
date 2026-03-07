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

pub struct UserAuth(pub UserEntity);
pub struct AdminAuth(pub Uuid);

#[derive(Debug, Clone)]
pub struct Auth<T> {
    pub user_id: Uuid,
    _role: std::marker::PhantomData<T>,
}

impl<S> FromRequestParts<S> for UserAuth
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
        let mut conn = app.db.acquire().await?;
        let user = UserEntity::get_by_id(&user_id, &app.redis, &mut conn).await?;
        if let Some(user) = user {
            user.check_user()?;
            Ok(UserAuth(user))
        } else {
            Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted))
        }
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
