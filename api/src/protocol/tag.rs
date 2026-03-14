use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult, validation::ValidationError},
    protocol::common::ColorSchema,
    utils::{AppTrim, FieldValidate},
    validation::ValidateStringLength,
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create Tag Request")]
pub struct CreateTagRequest {
    pub name: String,
    pub color: ColorSchema,
}

impl AppTrim for CreateTagRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl FieldValidate for CreateTagRequest {
    fn field_validate(&self) -> ApiResult<()> {
        let mut errs = Vec::new();
        if let Err(e) = self.name.length(3, 64) {
            errs.push(ValidationError("CreateTagRequest.name", e));
        }
        if errs.len() > 0 {
            return Err(ApiError::Validation(errs));
        }
        Ok(())
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
