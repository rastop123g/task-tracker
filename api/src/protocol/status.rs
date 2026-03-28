use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    protocol::common::{ColorSchema, StatusCategorySchema},
    utils::AppTrim,
    validation::{ValidateBody, ValidateBodyResult, ValidateStringLength, ValidationErrorKind},
};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Status name")]
pub struct StatusName(pub String);

impl From<StatusName> for String {
    fn from(name: StatusName) -> Self {
        name.0
    }
}

impl AppTrim for StatusName {
    fn app_trim(&mut self) {
        self.0.app_trim();
    }
}

impl ValidateBody for StatusName {
    fn validate_body(&self) -> ValidateBodyResult {
        self.0.length(3, 64).into_validate_body_result("StatusName")
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create Status Request")]
pub struct CreateStatusRequest {
    pub name: StatusName,
    pub category: StatusCategorySchema,
    pub color: ColorSchema,
}

impl AppTrim for CreateStatusRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl ValidateBody for CreateStatusRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name.validate_body()
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Status Response")]
pub struct StatusResponse {
    pub id: Uuid,
    pub name: String,
    pub category: StatusCategorySchema,
    pub color: ColorSchema,
    pub workspace: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Update Status Request")]
pub struct UpdateStatusRequest {
    pub name: Option<StatusName>,
    pub category: Option<StatusCategorySchema>,
    pub color: Option<ColorSchema>,
}

impl AppTrim for UpdateStatusRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl ValidateBody for UpdateStatusRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        if self.name.is_none() && self.category.is_none() && self.color.is_none() {
            return ValidateBodyResult::One(ValidationErrorKind::Body("Empty update".to_string()));
        }
        self.name.validate_body()
    }
}
