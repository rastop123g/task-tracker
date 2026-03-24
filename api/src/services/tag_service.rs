use uuid::Uuid;

use crate::{
    db::tag::{DBNewWorkspaceTag, DBTag, DBUpdateWorkspaceTag},
    entity::tag::{CreateTagEntity, TagEntity, UpdateTagEntity},
    error::{ApiError, ApiResult},
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct TagService {
    ctx: Ctx,
}

impl TagService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn get_list(&self, workspace_id: &Uuid) -> ApiResult<Vec<TagEntity>> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let tags = DBTag::get_by_workspace_id(workspace_id, &mut conn).await?;
        Ok(tags.into_iter().map(Into::into).collect())
    }

    pub async fn get(&self, id: &Uuid) -> ApiResult<TagEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let tag = DBTag::get_by_id(id, &mut conn).await?;
        let res = tag
            .filter(|t| t.deleted_at.is_none())
            .ok_or(ApiError::NotFound("Tag".to_string()))?;
        Ok(res.into())
    }

    pub async fn create(&self, data: CreateTagEntity, workspace_id: &Uuid) -> ApiResult<TagEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let tag = DBNewWorkspaceTag::from(data)
            .create(workspace_id, &mut conn)
            .await?;
        Ok(tag.into())
    }

    pub async fn update(&self, id: &Uuid, data: UpdateTagEntity) -> ApiResult<TagEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let tag = DBUpdateWorkspaceTag::from(data)
            .update(id, &mut conn)
            .await?;
        let res = tag
            .filter(|t| t.deleted_at.is_none())
            .ok_or(ApiError::NotFound("Tag".to_string()))?;
        Ok(res.into())
    }

    pub async fn delete(&self, id: &Uuid) -> ApiResult<TagEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let tag = DBTag::delete(id, &mut conn).await?;
        tag.map(Into::into)
            .ok_or(ApiError::NotFound("Tag".to_string()))
    }
}
