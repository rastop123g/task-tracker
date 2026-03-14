use std::sync::Arc;

use uuid::Uuid;

use crate::{
    config::Config,
    db::{
        DbPool,
        status::DBNewWorkspaceStatus,
        tag::DBNewWorkspaceTag,
        workspace::{DBNewWorkspace, DBWorkspace},
        workspace_member::DBWorkspaceMember,
    },
    entity::{
        status::StatusEntity,
        tag::TagEntity,
        user::UserEntity,
        workspace::{CreateWorkspaceEntity, WorkspaceEntity},
    },
    error::{ApiError, ApiResult},
    redis::RedisClient,
};

#[derive(Debug, Clone)]
pub struct WorkspaceService {
    db: DbPool,
    redis: RedisClient,
    config: Arc<Config>,
}

impl WorkspaceService {
    pub fn new(db: DbPool, redis: RedisClient, config: Arc<Config>) -> Self {
        Self { db, redis, config }
    }

    pub async fn get(&self, id: &Uuid) -> ApiResult<WorkspaceEntity> {
        let mut conn = self.db.acquire().await?;
        let workspace = DBWorkspace::get(id, &mut conn).await?;
        if let Some(workspace) = workspace
            && workspace.deleted_at.is_none()
        {
            Ok(workspace.into())
        } else {
            Err(ApiError::NotFound("workspace".to_string()))
        }
    }

    pub async fn create(
        &self,
        user: &UserEntity,
        req: CreateWorkspaceEntity,
    ) -> ApiResult<(WorkspaceEntity, Vec<StatusEntity>, Vec<TagEntity>)> {
        let mut tx = self.db.begin().await?;
        let workspace = DBNewWorkspace {
            name: req.name,
            admin_id: user.id,
        };
        let workspace = workspace.create(&mut tx).await?;
        DBWorkspaceMember::create(&user.id, &workspace.id, &mut tx).await?;
        let tags: Vec<DBNewWorkspaceTag> = req.tags.into_iter().map(Into::into).collect();
        let tags = DBNewWorkspaceTag::create_many(&tags, &workspace.id, &mut tx).await?;
        let statuses: Vec<DBNewWorkspaceStatus> =
            req.statuses.into_iter().map(Into::into).collect();
        let statuses = DBNewWorkspaceStatus::create_many(&statuses, &workspace.id, &mut tx).await?;
        tx.commit().await?;
        Ok((
            workspace.into(),
            statuses.into_iter().map(Into::into).collect(),
            tags.into_iter().map(Into::into).collect(),
        ))
    }
}
