use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
    utils::AppTrim,
    validation::{ValidateBody, ValidateBodyResult, ValidateStringLength},
};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "User name")]
pub struct UserName(pub String);

impl From<UserName> for String {
    fn from(name: UserName) -> Self {
        name.0
    }
}

impl ValidateBody for UserName {
    fn validate_body(&self) -> ValidateBodyResult {
        self.0.length(3, 128).into_validate_body_result("UserName")
    }
}

impl AppTrim for UserName {
    fn app_trim(&mut self) {
        self.0.app_trim();
    }
}

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
    pub name: UserName,
}

impl AppTrim for UpdateUserRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl ValidateBody for UpdateUserRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name.validate_body()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Change password")]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, IntoParams, ts_rs::TS)]
#[ts(export)]
pub struct SearchUserRequest {
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "User list item")]
pub struct UserListItemResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
