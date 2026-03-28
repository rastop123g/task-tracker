use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    cache::RedisCache,
    db::workspace::DBWorkspace,
    entity::{status::CreateStatusEntity, tag::CreateTagEntity},
    error::{ApiError, bad_request::BadRequestError},
    protocol::workspace::{CreateWorkspaceRequest, WorkspaceResponse},
};

#[derive(Debug, Clone)]
pub struct CreateWorkspaceEntity {
    pub name: String,
    pub statuses: Vec<CreateStatusEntity>,
    pub tags: Vec<CreateTagEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEntity {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub admin: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl RedisCache<Uuid> for WorkspaceEntity {
    fn cache_key(&self) -> String {
        Self::key_from(&self.id)
    }

    fn key_from(id: &Uuid) -> String {
        format!("workspace-entity:{}", id)
    }

    fn cache_exp(&self) -> u64 {
        60 * 60 * 24 * 7 // 7 days
    }
}

impl TryFrom<CreateWorkspaceRequest> for CreateWorkspaceEntity {
    type Error = ApiError;
    fn try_from(req: CreateWorkspaceRequest) -> Result<Self, Self::Error> {
        if req.statuses.is_empty() {
            return Err(ApiError::BadRequest(
                BadRequestError::MissingStatusOnCreateWorkspace,
            ));
        }
        Ok(Self {
            name: req.name.into(),
            statuses: req.statuses.into_iter().map(Into::into).collect(),
            tags: req.tags.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<WorkspaceEntity> for WorkspaceResponse {
    fn from(workspace: WorkspaceEntity) -> Self {
        Self {
            id: workspace.id,
            name: workspace.name,
            admin: workspace.admin,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
            deleted_at: workspace.deleted_at,
        }
    }
}

impl From<DBWorkspace> for WorkspaceEntity {
    fn from(workspace: DBWorkspace) -> Self {
        Self {
            id: workspace.id,
            name: workspace.name,
            avatar: workspace.avatar,
            admin: workspace.admin_id,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
            deleted_at: workspace.deleted_at,
        }
    }
}

impl From<WorkspaceEntity> for DBWorkspace {
    fn from(workspace: WorkspaceEntity) -> Self {
        Self {
            id: workspace.id,
            name: workspace.name,
            avatar: workspace.avatar,
            admin_id: workspace.admin,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
            deleted_at: workspace.deleted_at,
        }
    }
}
