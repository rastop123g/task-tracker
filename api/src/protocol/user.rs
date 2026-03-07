use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "User")]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub avatar_preview: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Update User")]
pub struct UpdateUserRequest {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Change password")]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}
