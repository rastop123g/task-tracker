use axum::{
    Json, Router,
    extract::{Path, State},
};
use sha2::{Digest, Sha256};
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    error::{ApiError, ApiResult, bad_request::BadRequestError},
    protocol::{
        error::UnauthotizedErrorResponse,
        user::{ChangePasswordRequest, UpdateUserRequest, UserResponse},
    },
    router::extractors::auth::UserAuth,
};

pub fn user_router() -> Router<AppResources> {
    Router::new()
        .route("/me", axum::routing::get(get_me))
        .route("/me", axum::routing::put(update_user))
        .route("/me/password", axum::routing::put(change_password))
        .route("/{user_id}", axum::routing::get(get_user))
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "user",
    responses(
        (status = 200, description = "OK", body = UserResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Get current user
pub async fn get_me(UserAuth(user): UserAuth) -> ApiResult<Json<UserResponse>> {
    Ok(Json(user.into()))
}

#[utoipa::path(
    put,
    path = "/me",
    tag = "user",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "OK", body = UserResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Update current user
pub async fn update_user(
    State(app): State<AppResources>,
    UserAuth(user): UserAuth,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
    let updated = app.user_service.update(&user.id, req.name).await?;
    Ok(Json(updated.into()))
}

#[utoipa::path(
    put,
    path = "/me/password",
    tag = "user",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "OK", body = UserResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Change password
pub async fn change_password(
    State(app): State<AppResources>,
    UserAuth(user): UserAuth,
    Json(req): Json<ChangePasswordRequest>,
) -> ApiResult<Json<UserResponse>> {
    let password = Sha256::digest(req.old_password.as_bytes());
    let hex_password = hex::encode(password);
    if hex_password != user.password {
        return Err(ApiError::BadRequest(BadRequestError::OldPasswordWrong));
    }
    let updated = app
        .user_service
        .change_password(&user.id, req.new_password)
        .await?;
    Ok(Json(updated.into()))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "user",
    responses(
        (status = 200, description = "OK", body = UserResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Get user
pub async fn get_user(
    State(app): State<AppResources>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<UserResponse>> {
    let user = app.user_service.get(&user_id).await?;
    if let Some(user) = user {
        if user.deleted_at.is_some() {
            return Err(ApiError::NotFound("user".to_string()));
        }
        if user.confirmed == false {
            return Err(ApiError::NotFound("user".to_string()));
        }
        Ok(Json(user.into()))
    } else {
        return Err(ApiError::NotFound("user".to_string()))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_me,
        update_user,
        change_password,
        get_user,
    ),
    components(schemas(UserResponse, UpdateUserRequest, ChangePasswordRequest, UnauthotizedErrorResponse)),
    tags((name = "user", description = "User")),
)]
pub struct UserApiDoc;
