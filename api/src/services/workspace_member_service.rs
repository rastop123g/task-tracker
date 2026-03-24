use uuid::Uuid;

use crate::{
    db::workspace_member::DBWorkspaceMember, error::ApiResult, router::extractors::req_ctx::Ctx,
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
}
