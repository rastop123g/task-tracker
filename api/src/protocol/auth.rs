use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult, validation::ValidationError},
    utils::{AppTrim, FieldValidate},
    validation::{AppValidateEmail, ValidateStringLength},
};

#[derive(Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Register a new user")]
pub struct RegisterRequest {
    /// User name (full)
    pub name: String,
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

impl FieldValidate for RegisterRequest {
    fn field_validate(&self) -> ApiResult<()> {
        let mut errs = Vec::new();
        if let Err(e) = self.name.length(3, 128) {
            errs.push(ValidationError("RegisterRequest.name", e));
        }
        if let Err(e) = self.email.validate_email() {
            errs.push(ValidationError("RegisterRequest.email", e));
        }
        if let Err(e) = self.password.length(8, 128) {
            errs.push(ValidationError("RegisterRequest.password", e));
        }
        if errs.len() > 0 {
            return Err(ApiError::Validation(errs));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Login user request")]
pub struct LoginRequest {
    /// User email
    pub email: String,
    /// User password
    pub password: String,
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
