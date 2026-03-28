use uuid::Uuid;

use crate::{
    db::tag::{DBNewWorkspaceTag, DBTag, DBUpdateWorkspaceTag},
    entity::common::ColorEntity,
    protocol::tag::{CreateTagRequest, TagResponse, UpdateTagRequest},
};

#[derive(Debug, Clone)]
pub struct CreateTagEntity {
    pub name: String,
    pub color: ColorEntity,
}

#[derive(Debug, Clone)]
pub struct TagEntity {
    pub id: Uuid,
    pub name: String,
    pub color: ColorEntity,
    pub workspace: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct UpdateTagEntity {
    pub name: Option<String>,
    pub color: Option<ColorEntity>,
}

impl From<CreateTagRequest> for CreateTagEntity {
    fn from(req: CreateTagRequest) -> Self {
        Self {
            name: req.name.into(),
            color: req.color.into(),
        }
    }
}

impl From<CreateTagEntity> for DBNewWorkspaceTag {
    fn from(tag: CreateTagEntity) -> Self {
        Self {
            name: tag.name,
            color: tag.color.into(),
        }
    }
}

impl From<DBTag> for TagEntity {
    fn from(tag: DBTag) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            color: tag.color.into(),
            workspace: tag.workspace_id,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
            deleted_at: tag.deleted_at,
        }
    }
}

impl From<TagEntity> for TagResponse {
    fn from(tag: TagEntity) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
            color: tag.color.into(),
            workspace: tag.workspace,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
            deleted_at: tag.deleted_at,
        }
    }
}

impl From<UpdateTagRequest> for UpdateTagEntity {
    fn from(req: UpdateTagRequest) -> Self {
        Self {
            name: req.name.map(|n| n.into()),
            color: req.color.map(|c| c.into()),
        }
    }
}

impl From<UpdateTagEntity> for DBUpdateWorkspaceTag {
    fn from(tag: UpdateTagEntity) -> Self {
        Self {
            name: tag.name,
            color: tag.color.map(|c| c.into()),
        }
    }
}
