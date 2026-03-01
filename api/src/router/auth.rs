use axum::{
    Json, Router, body::Body, extract::{Query, State}, http::{Request, StatusCode}, middleware::Next, response::Response
};
use chrono::{Duration, Utc};
use protocol::auth::{
    LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, RegisterRequest,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    app_resources::AppResources,
    db::{self, user::User},
    error::{ApiError, ApiResult, unauthotized::UnauthotizedError},
    jwt,
};

pub fn auth_router() -> Router<AppResources> {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/refresh", axum::routing::post(refresh_token))
        .route("/verify", axum::routing::get(verify_email))
}

pub async fn register(
    State(app): State<AppResources>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<String> {
    let mut conn = app.db.acquire().await?;
    let users = User::get_by_email(&req.email, &mut conn).await?;
    if users.iter().any(|u| u.confirmed == true) {
        return Err(ApiError::BadRequest("email already used".to_string()));
    }
    let new_user = db::user::NewUser::try_from(req)?;
    let created = new_user.create(&mut conn).await?;
    let confirmation_token_exp = Utc::now() + Duration::hours(3);
    let confirmation_token = jwt::create(&created.id, confirmation_token_exp, &app.config)?;
    let link = format!(
        "{}?token={}",
        app.config.validate_email_prefix, confirmation_token
    );
    tracing::debug!("confirm email link: {}", link);
    // TODO: send email task
    Ok("ok".into())
}

pub async fn login(
    State(app): State<AppResources>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let email = req.email;
    let password = Sha256::digest(req.password.as_bytes());
    let hex_password = hex::encode(password);
    let mut conn = app.db.acquire().await?;
    let users = db::user::User::check_credentials(&email, &hex_password, &mut conn).await?;
    if users.len() == 0 {
        return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
    }
    if users.iter().all(|user| user.confirmed == false) {
        return Err(ApiError::Unauthorized(UnauthotizedError::EmailNotConfirmed));
    }
    let user = users.into_iter().find(|user| user.confirmed == true);
    if let Some(user) = user {
        if user.deleted_at.is_some() {
            return Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted));
        }
        let token_exp = Utc::now() + chrono::Duration::days(3);
        let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
        let token = jwt::create(&user.id, token_exp, &app.config)?;
        let refresh_token = jwt::create(&user.id, refresh_token_exp, &app.config)?;
        Ok(Json(LoginResponse {
            user_id: user.id,
            avatar_id: user.avatar,
            token,
            refresh_token,
            email: user.email,
            token_exp: token_exp,
            refresh_exp: refresh_token_exp,
            server_time: Utc::now(),
        }))
    } else {
        return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
    }
}

pub async fn refresh_token(
    State(app): State<AppResources>,
    Json(req): Json<RefreshTokenRequest>,
) -> ApiResult<Json<RefreshTokenResponse>> {
    let user_id = jwt::verify(&req.refresh_token, &app.config)?;
    let mut conn = app.db.acquire().await?;
    let user = db::user::User::get(&user_id, &mut conn).await?;
    if let Some(user) = user {
        if user.deleted_at.is_some() {
            return Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted));
        }
        if user.confirmed == false {
            return Err(ApiError::Unauthorized(UnauthotizedError::EmailNotConfirmed));
        }
        let token_exp = Utc::now() + chrono::Duration::days(3);
        let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
        let token = jwt::create(&user.id, token_exp, &app.config)?;
        let refresh_token = jwt::create(&user.id, refresh_token_exp, &app.config)?;
        Ok(Json(RefreshTokenResponse {
            token,
            refresh_token,
            token_exp: token_exp,
            refresh_exp: refresh_token_exp,
            server_time: Utc::now(),
        }))
    } else {
        return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyEmailResponse {
    pub status: String,
}

pub async fn verify_email(
    State(app): State<AppResources>,
    Query(req): Query<VerifyEmailRequest>,
) -> ApiResult<Json<VerifyEmailResponse>> {
    let token = req.token;
    let user_id = jwt::verify(&token, &app.config)?;
    let mut conn = app.db.acquire().await?;
    let user = db::user::User::get(&user_id, &mut conn).await?;
    if let Some(user) = user {
        if user.deleted_at.is_some() {
            return Err(ApiError::CustomHttp(
                StatusCode::FORBIDDEN,
                "user deleted".to_string(),
            ));
        }
        if user.confirmed == true {
            return Err(ApiError::CustomHttp(
                StatusCode::FORBIDDEN,
                "email already confirmed".to_string(),
            ));
        }
        User::confirm_email(&user_id, &mut conn).await?;
        Ok(Json(VerifyEmailResponse {
            status: "ok".to_string(),
        }))
    } else {
        return Err(ApiError::CustomHttp(
            StatusCode::NOT_FOUND,
            "user not found".to_string(),
        ));
    }
}

pub async fn auth_middleware(
    State(state): State<AppResources>,
    mut req: Request<Body>,
    next: Next,
) -> ApiResult<Response> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized(UnauthotizedError::MissingToken))?;

    let user_id = jwt::verify(token, &state.config)?;

    req.extensions_mut().insert(AuthUser(user_id));

    Ok(next.run(req).await)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthUser(Uuid);
