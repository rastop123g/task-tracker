use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

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
    #[ts(type = "string")]
    pub user_id: Uuid,
    pub email: String,
    /// access token
    pub token: String,
    /// refresh token
    pub refresh_token: String,
    /// token expiration date
    #[ts(type = "string")]
    pub token_exp: DateTime<Utc>,
    /// refresh token expiration date
    #[ts(type = "string")]
    pub refresh_exp: DateTime<Utc>,
    /// server time for calculating time difference
    #[ts(type = "string")]
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
    #[ts(type = "string")]
    pub token_exp: DateTime<Utc>,
    /// refresh token expiration date
    #[ts(type = "string")]
    pub refresh_exp: DateTime<Utc>,
    /// server time for calculating time difference
    #[ts(type = "string")]
    pub server_time: DateTime<Utc>,
}
