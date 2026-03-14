use crate::{
    protocol::{
        auth::{
            LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
        },
        error::{UnauthotizedErrorResponse, ValidationErrorResponse},
    },
    utils::{AppTrim, FieldValidate},
};
use axum::{
    Json, Router,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::{app_resources::AppResources, error::ApiResult};

pub fn auth_router() -> Router<AppResources> {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/refresh", axum::routing::post(refresh_token))
        .route("/verify", axum::routing::get(verify_email))
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "OK", body = String),
        (status = 400, description = "Bad Request", body = ValidationErrorResponse),
    ),
)]
/// Register new user
pub async fn register(
    State(app): State<AppResources>,
    Json(mut req): Json<RegisterRequest>,
) -> ApiResult<String> {
    req.app_trim();
    req.field_validate()?;
    app.auth_service.register(req.try_into()?).await?;
    Ok("ok".into())
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "OK", body = LoginResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Login user
pub async fn login(
    State(app): State<AppResources>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let res = app.auth_service.login(req.email, req.password).await?;
    Ok(Json(res.into()))
}

#[utoipa::path(
    post,
    path = "/refresh",
    tag = "auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "OK", body = RefreshTokenResponse),
        (status = 401, description = "Unauthorized", body = UnauthotizedErrorResponse),
    ),
)]
/// Refresh user token
pub async fn refresh_token(
    State(app): State<AppResources>,
    Json(req): Json<RefreshTokenRequest>,
) -> ApiResult<Json<RefreshTokenResponse>> {
    let res = app.auth_service.refresh(&req.refresh_token).await?;
    Ok(Json(res.into()))
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct VerifyEmailRequest {
    /// Token
    pub token: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(description = "Verify Email Response")]
pub struct VerifyEmailResponse {
    /// Status
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/verify",
    tag = "auth",
    params(
        VerifyEmailRequest,
    ),
    responses(
        (status = 200, description = "OK", body = VerifyEmailResponse),
        (status = 404, description = "Not Found"),
        (status = 403, description = "Forbidden"),
    ),
)]
/// Verify email
pub async fn verify_email(
    State(app): State<AppResources>,
    Query(req): Query<VerifyEmailRequest>,
) -> ApiResult<Json<VerifyEmailResponse>> {
    let token = req.token;
    app.auth_service.verify_email(&token).await?;
    Ok(Json(VerifyEmailResponse {
        status: "ok".to_string(),
    }))
}

#[derive(OpenApi)]
#[openapi(
    paths(register, login, refresh_token, verify_email),
    components(schemas(RegisterRequest, LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, VerifyEmailResponse, UnauthotizedErrorResponse)),
    tags((name = "auth", description = "Authentication")),
)]
pub struct AuthApiDoc;
