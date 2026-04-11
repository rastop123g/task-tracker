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
        let members = self.ctx.ws_registry_service()
            .get_workspace_members(workspace_id)
            .await?;
        Ok(members.contains(user_id))
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
        self.ctx.ws_registry_service()
            .remove_workspace_member(workspace_id, user_id)
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
