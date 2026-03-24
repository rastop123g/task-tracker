use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    protocol::user::UserName,
    utils::AppTrim,
    validation::{AppValidateEmail, ValidateBody, ValidateBodyResult, ValidateStringLength},
};

#[derive(Serialize, Clone, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Register a new user")]
pub struct RegisterRequest {
    /// User name (full)
    pub name: UserName,
    /// User email
    pub email: String,
    /// User password
    pub password: String,
}

impl AppTrim for RegisterRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
        self.email.app_trim();
        self.password.app_trim();
    }
}

impl ValidateBody for RegisterRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name
            .validate_body()
            .and(
                self.email
                    .validate_email()
                    .into_validate_body_result("RegisterRequest.email"),
            )
            .and(
                self.password
                    .length(8, 128)
                    .into_validate_body_result("RegisterRequest.password"),
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Login user request")]
pub struct LoginRequest {
    /// User email
    pub email: String,
    /// User password
    pub password: String,
}

impl AppTrim for LoginRequest {
    fn app_trim(&mut self) {
        self.email.app_trim();
        self.password.app_trim();
    }
}

impl ValidateBody for LoginRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.email
            .max_length(256)
            .into_validate_body_result("LoginRequest.email")
            .and(
                self.password
                    .max_length(256)
                    .into_validate_body_result("LoginRequest.password"),
            )
    }
}

#[derive(Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Login response")]
pub struct LoginResponse {
    /// User id (uuid)
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// access token
    pub token: String,
    /// refresh token
    pub refresh_token: String,
    /// token expiration date
    pub token_exp: DateTime<Utc>,
    /// refresh token expiration date
    pub refresh_exp: DateTime<Utc>,
    /// server time for calculating time difference
    pub server_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Refresh token request")]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Refresh token response")]
pub struct RefreshTokenResponse {
    /// access token
    pub token: String,
    /// refresh token
    pub refresh_token: String,
    /// token expiration date
    pub token_exp: DateTime<Utc>,
    /// refresh token expiration date
    pub refresh_exp: DateTime<Utc>,
    /// server time for calculating time difference
    pub server_time: DateTime<Utc>,
}
