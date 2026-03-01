use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    #[ts(type = "string | undefined")]
    pub avatar_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct LoginResponse {
    #[ts(type = "string")]
    pub user_id: Uuid,
    pub email: String,
    #[ts(type = "string | undefined")]
    pub avatar_id: Option<String>,
    pub token: String,
    pub refresh_token: String,
    #[ts(type = "string")]
    pub token_exp: DateTime<Utc>,
    #[ts(type = "string")]
    pub refresh_exp: DateTime<Utc>,
    #[ts(type = "string")]
    pub server_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ts_rs::TS)]
#[ts(export)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: String,
    #[ts(type = "string")]
    pub token_exp: DateTime<Utc>,
    #[ts(type = "string")]
    pub refresh_exp: DateTime<Utc>,
    #[ts(type = "string")]
    pub server_time: DateTime<Utc>,
}
