use crate::{
    db::status::{DBNewWorkspaceStatus, DBUpdateWorkspaceStatus, DBWorkspaceStatus},
    entity::status::{CreateStatusEntity, StatusEntity, UpdateStatusEntity},
    error::{ApiError, ApiResult},
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct WorkspaceStatusesService {
    pub ctx: Ctx,
}

//NOTE: В списке мы всегда возвращаем все статусы даже удаленные (для отображения)
//Но ничего не даем с ними делать
impl WorkspaceStatusesService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn get_statuses(&self, workspace_id: &uuid::Uuid) -> ApiResult<Vec<StatusEntity>> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let statuses = DBWorkspaceStatus::get_by_workspace_id(workspace_id, &mut conn).await?;
        Ok(statuses.into_iter().map(Into::into).collect())
    }

    pub async fn get_status(&self, id: &uuid::Uuid) -> ApiResult<StatusEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let status = DBWorkspaceStatus::get_by_id(id, &mut conn).await?;
        if let Some(status) = status {
            let entity = StatusEntity::from(status);
            if entity.deleted_at.is_some() {
                return Err(ApiError::NotFound("status".to_string()));
            }
            Ok(entity)
        } else {
            Err(ApiError::NotFound("status".to_string()))
        }
    }

    pub async fn create_status(
        &self,
        workspace_id: &uuid::Uuid,
        req: CreateStatusEntity,
    ) -> ApiResult<StatusEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let res = DBNewWorkspaceStatus::from(req)
            .create(workspace_id, &mut conn)
            .await?;
        Ok(res.into())
    }

    pub async fn update_status(
        &self,
        id: &uuid::Uuid,
        req: UpdateStatusEntity,
    ) -> ApiResult<StatusEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let res = DBUpdateWorkspaceStatus::from(req)
            .update(id, &mut conn)
            .await?;
        if let Some(res) = res {
            Ok(res.into())
        } else {
            Err(ApiError::NotFound("status".to_string()))
        }
    }

    pub async fn delete_status(&self, id: &uuid::Uuid) -> ApiResult<StatusEntity> {
        let mut conn = self.ctx.app.db.acquire().await?;
        let res = DBWorkspaceStatus::delete(id, &mut conn).await?;
        if let Some(res) = res {
            Ok(res.into())
        } else {
            Err(ApiError::NotFound("status".to_string()))
        }
    }
}
