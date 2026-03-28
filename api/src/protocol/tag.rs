use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    protocol::common::ColorSchema,
    utils::AppTrim,
    validation::{ValidateBody, ValidateBodyResult, ValidateStringLength},
};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Tag name")]
pub struct TagName(pub String);

impl From<TagName> for String {
    fn from(name: TagName) -> Self {
        name.0
    }
}

impl AppTrim for TagName {
    fn app_trim(&mut self) {
        self.0.app_trim();
    }
}

impl ValidateBody for TagName {
    fn validate_body(&self) -> ValidateBodyResult {
        self.0.length(3, 64).into_validate_body_result("TagName")
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create Tag Request")]
pub struct CreateTagRequest {
    pub name: TagName,
    pub color: ColorSchema,
}

impl AppTrim for CreateTagRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl ValidateBody for CreateTagRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name.validate_body()
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Tag Response")]
pub struct TagResponse {
    pub id: Uuid,
    pub name: String,
    pub color: ColorSchema,
    pub workspace: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Update Tag Request")]
pub struct UpdateTagRequest {
    pub name: Option<TagName>,
    pub color: Option<ColorSchema>,
}

impl AppTrim for UpdateTagRequest {
    fn app_trim(&mut self) {
        if let Some(name) = &mut self.name {
            name.app_trim();
        }
    }
}

impl ValidateBody for UpdateTagRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name.validate_body()
    }
}
