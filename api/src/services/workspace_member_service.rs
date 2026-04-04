use uuid::Uuid;

use crate::{
    db::workspace_member::{DBWorkspaceMember, DBWorkspaceMemberWithUser},
    entity::workspace_member::WorkspaceMemberEntity,
    error::{ApiError, ApiResult},
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct WorkspaceMemberService {
    ctx: Ctx,
}

impl WorkspaceMemberService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn check_member(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<bool> {
        //TODO: cache in redis
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let member = DBWorkspaceMember::get(user_id, workspace_id, &mut conn).await?;
        let res = member.map(|m| m.deleted_at.is_none()).unwrap_or(false);
        Ok(res)
    }

    pub async fn get(
        &self,
        workspace_id: &Uuid,
        user_id: &Uuid,
    ) -> ApiResult<WorkspaceMemberEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        DBWorkspaceMemberWithUser::get(user_id, workspace_id, &mut conn)
            .await?
            .filter(|v| v.deleted_at.is_none() && v.user_deleted_at.is_none())
            .map(|v| v.into())
            .ok_or_else(|| ApiError::NotFound("workspace_member".to_string()))
    }

    pub async fn delete(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        DBWorkspaceMember::delete(user_id, workspace_id, &mut conn)
            .await?;
        Ok(())
    }

    pub async fn get_list(&self, workspace_id: &Uuid) -> ApiResult<Vec<WorkspaceMemberEntity>> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        DBWorkspaceMemberWithUser::get_list(workspace_id, &mut conn)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }
}
