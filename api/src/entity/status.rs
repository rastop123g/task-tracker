use uuid::Uuid;

use crate::{
    db::status::{DBNewWorkspaceStatus, DBStatus},
    entity::common::{ColorEntity, StatusCategoryEntity},
    protocol::status::{CreateStatusRequest, StatusResponse},
};

#[derive(Debug, Clone)]
pub struct CreateStatusEntity {
    pub name: String,
    pub category: StatusCategoryEntity,
    pub color: ColorEntity,
}

#[derive(Debug, Clone)]
pub struct StatusEntity {
    pub id: Uuid,
    pub name: String,
    pub category: StatusCategoryEntity,
    pub color: ColorEntity,
    pub workspace: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl TryFrom<CreateStatusRequest> for CreateStatusEntity {
    type Error = crate::error::ApiError;
    fn try_from(req: CreateStatusRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: req.name,
            category: req.category.into(),
            color: req.color.into(),
        })
    }
}

impl From<CreateStatusEntity> for DBNewWorkspaceStatus {
    fn from(status: CreateStatusEntity) -> Self {
        Self {
            name: status.name,
            category: status.category.into(),
            color: status.color.into(),
        }
    }
}

impl From<DBStatus> for StatusEntity {
    fn from(status: DBStatus) -> Self {
        Self {
            id: status.id,
            name: status.name,
            category: status.category.into(),
            color: status.color.into(),
            workspace: status.workspace_id,
            created_at: status.created_at,
            updated_at: status.updated_at,
            deleted_at: status.deleted_at,
        }
    }
}

impl From<StatusEntity> for StatusResponse {
    fn from(status: StatusEntity) -> Self {
        Self {
            id: status.id,
            name: status.name,
            category: status.category.into(),
            color: status.color.into(),
            workspace: status.workspace,
            created_at: status.created_at,
            updated_at: status.updated_at,
            deleted_at: status.deleted_at,
        }
    }
}
