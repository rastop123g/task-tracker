use uuid::Uuid;

use crate::{
    cache::RedisCache,
    db::{
        status::DBNewWorkspaceStatus,
        tag::DBNewWorkspaceTag,
        workspace::{DBNewWorkspace, DBUpdateWorkspace, DBWorkspace},
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
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct WorkspaceService {
    ctx: Ctx,
}

impl WorkspaceService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn get(&self, id: &Uuid) -> ApiResult<WorkspaceEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let workspace = WorkspaceEntity::get_by_id(id, &app.redis, &mut conn).await?;
        if let Some(workspace) = workspace
            && workspace.deleted_at.is_none()
        {
            Ok(workspace)
        } else {
            Err(ApiError::NotFound("workspace".to_string()))
        }
    }

    pub async fn create(
        &self,
        user: &UserEntity,
        req: CreateWorkspaceEntity,
    ) -> ApiResult<(WorkspaceEntity, Vec<StatusEntity>, Vec<TagEntity>)> {
        let app = &self.ctx.app;
        let mut tx = app.db.begin().await?;
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
        let workspace = WorkspaceEntity::from(workspace);
        workspace.cache(&app.redis).await?;
        tx.commit().await?;
        Ok((
            workspace,
            statuses.into_iter().map(Into::into).collect(),
            tags.into_iter().map(Into::into).collect(),
        ))
    }

    pub async fn update(&self, workspace_id: &Uuid, name: String) -> ApiResult<WorkspaceEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let res = DBUpdateWorkspace {
            name: Some(name),
            ..Default::default()
        }
        .update(workspace_id, &mut conn)
        .await?
        .ok_or(ApiError::NotFound("workspace".to_string()))?;
        let w = WorkspaceEntity::from(res);
        w.cache(&app.redis).await?;
        Ok(w)
    }

    pub async fn change_admin(
        &self,
        workspace_id: &Uuid,
        admin_id: &Uuid,
    ) -> ApiResult<WorkspaceEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let res = DBUpdateWorkspace {
            admin_id: Some(*admin_id),
            ..Default::default()
        }
        .update(workspace_id, &mut conn)
        .await?
        .ok_or(ApiError::NotFound("workspace".to_string()))?;
        let w = WorkspaceEntity::from(res);
        w.cache(&app.redis).await?;
        Ok(w)
    }
}

impl WorkspaceEntity {
    pub async fn get_by_id(
        id: &Uuid,
        redis: &RedisClient,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<Self>> {
        let cached = WorkspaceEntity::cached(id, redis).await?;
        if let Some(cached) = cached {
            Ok(Some(cached))
        } else {
            let workspace = DBWorkspace::get(id, db).await?;
            if let Some(workspace) = workspace {
                let entity = Self::from(workspace);
                entity.cache(redis).await?;
                Ok(Some(entity))
            } else {
                Ok(None)
            }
        }
    }
}
