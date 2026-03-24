use uuid::Uuid;

use crate::{
    db::status::{DBNewWorkspaceStatus, DBUpdateWorkspaceStatus, DBWorkspaceStatus},
    entity::common::{ColorEntity, StatusCategoryEntity},
    protocol::status::{CreateStatusRequest, StatusResponse, UpdateStatusRequest},
};

#[derive(Debug, Clone)]
pub struct CreateStatusEntity {
    pub name: String,
    pub category: StatusCategoryEntity,
    pub color: ColorEntity,
}

#[derive(Debug, Clone)]
pub struct UpdateStatusEntity {
    pub name: Option<String>,
    pub category: Option<StatusCategoryEntity>,
    pub color: Option<ColorEntity>,
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

impl From<CreateStatusRequest> for CreateStatusEntity {
    fn from(req: CreateStatusRequest) -> Self {
        Self {
            name: req.name.into(),
            category: req.category.into(),
            color: req.color.into(),
        }
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

impl From<DBWorkspaceStatus> for StatusEntity {
    fn from(status: DBWorkspaceStatus) -> Self {
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

impl From<UpdateStatusRequest> for UpdateStatusEntity {
    fn from(req: UpdateStatusRequest) -> Self {
        Self {
            name: req.name.map(|n| n.into()),
            category: req.category.map(|c| c.into()),
            color: req.color.map(|c| c.into()),
        }
    }
}

impl From<UpdateStatusEntity> for DBUpdateWorkspaceStatus {
    fn from(status: UpdateStatusEntity) -> Self {
        Self {
            name: status.name,
            category: status.category.map(|c| c.into()),
            color: status.color.map(|c| c.into()),
        }
    }
}
