use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    protocol::{
        status::{CreateStatusRequest, StatusResponse},
        tag::{CreateTagRequest, TagResponse},
    },
    utils::AppTrim,
    validation::{ValidateBody, ValidateBodyResult, ValidateStringLength},
};

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Workspace name")]
pub struct WorkspaceName(pub String);

impl From<WorkspaceName> for String {
    fn from(name: WorkspaceName) -> Self {
        name.0
    }
}

impl AppTrim for WorkspaceName {
    fn app_trim(&mut self) {
        self.0.app_trim();
    }
}

impl ValidateBody for WorkspaceName {
    fn validate_body(&self) -> ValidateBodyResult {
        self.0
            .length(3, 128)
            .into_validate_body_result("WorkspaceName")
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create Workspace Request")]
pub struct CreateWorkspaceRequest {
    pub name: WorkspaceName,
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

impl ValidateBody for CreateWorkspaceRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name
            .validate_body()
            .and(self.statuses.validate_body())
            .and(self.tags.validate_body())
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

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Update Workspace Request")]
pub struct UpdateWorkspaceRequest {
    pub name: WorkspaceName,
}

impl AppTrim for UpdateWorkspaceRequest {
    fn app_trim(&mut self) {
        self.name.app_trim();
    }
}

impl ValidateBody for UpdateWorkspaceRequest {
    fn validate_body(&self) -> ValidateBodyResult {
        self.name.validate_body()
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Change Admin Request")]
pub struct ChangeAdminRequest {
    pub admin: Uuid,
}

impl AppTrim for ChangeAdminRequest {}

impl ValidateBody for ChangeAdminRequest {}
