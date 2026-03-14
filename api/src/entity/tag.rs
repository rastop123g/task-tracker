use uuid::Uuid;

use crate::{
    db::tag::{DBNewWorkspaceTag, DBTag},
    entity::common::ColorEntity,
    error::ApiError,
    protocol::tag::TagResponse,
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

impl TryFrom<crate::protocol::tag::CreateTagRequest> for CreateTagEntity {
    type Error = ApiError;
    fn try_from(req: crate::protocol::tag::CreateTagRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: req.name,
            color: req.color.into(),
        })
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
