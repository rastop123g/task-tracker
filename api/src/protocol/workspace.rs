use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::{ApiError, ApiResult, validation::ValidationError},
    protocol::{
        status::{CreateStatusRequest, StatusResponse},
        tag::{CreateTagRequest, TagResponse},
    },
    utils::{AppTrim, FieldValidate},
    validation::ValidateStringLength,
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create Workspace Request")]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub statuses: Vec<CreateStatusRequest>,
    pub tags: Vec<CreateTagRequest>,
}

impl AppTrim for CreateWorkspaceRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
        self.statuses.app_trim();
        self.tags.app_trim();
    }
}

impl FieldValidate for CreateWorkspaceRequest {
    fn field_validate(&self) -> ApiResult<()> {
        if let Err(e) = self.name.length(3, 128) {
            return Err(ApiError::Validation(vec![ValidationError(
                "CreateWorkspaceRequest.name",
                e,
            )]));
        }
        self.statuses.field_validate()?;
        self.tags.field_validate()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Workspace Response")]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    pub admin: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Workspace Response")]
pub struct WorkspaceWithStatusesAndTagsResponse {
    pub id: Uuid,
    pub name: String,
    pub admin: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub statuses: Vec<StatusResponse>,
    pub tags: Vec<TagResponse>,
}
