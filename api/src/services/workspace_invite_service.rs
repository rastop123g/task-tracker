use uuid::Uuid;

use crate::{
    db::{
        user::{DBUserListItem}, workspace_invite::DBWorkspaceInvite,
        workspace_member::DBWorkspaceMember,
    }, entity::{user::{UserEntity, UserListItemEntity}, workspace_invite::WorkspaceInviteEntity}, error::{ApiError, ApiResult, bad_request::BadRequestError}, protocol::websocket::ws_outgoing::user::{AddMemberEvent, RevokeInviteEvent, UserInviteEvent}, router::extractors::req_ctx::Ctx, websocket::global_fan_out::GlobalFanOutSender
};

#[derive(Debug, Clone)]
pub struct WorkspaceInviteService {
    ctx: Ctx,
}

impl WorkspaceInviteService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn get_user_invitations(
        &self,
        user_id: &Uuid,
    ) -> ApiResult<Vec<WorkspaceInviteEntity>> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let workspace_invites = DBWorkspaceInvite::get_by_user_id(user_id, &mut conn).await?;
        Ok(workspace_invites.into_iter().map(Into::into).collect())
    }

    pub async fn get_workspace_invitations(
        &self,
        workspace_id: &Uuid,
    ) -> ApiResult<Vec<WorkspaceInviteEntity>> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let workspace_invites =
            DBWorkspaceInvite::get_by_workspace_id(workspace_id, &mut conn).await?;
        Ok(workspace_invites.into_iter().map(Into::into).collect())
    }

    pub async fn create(
        &self,
        workspace_id: &Uuid,
        user_id: &Uuid,
    ) -> ApiResult<WorkspaceInviteEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let member = DBWorkspaceMember::get(user_id, workspace_id, &mut conn).await?;
        if let Some(member) = member
            && member.deleted_at.is_none()
        {
            return Err(ApiError::BadRequest(BadRequestError::UserIsMember));
        }
        let invite = DBWorkspaceInvite::get(workspace_id, user_id, &mut conn).await?;
        if let Some(invite) = invite
            && invite.deleted_at.is_none()
        {
            return Err(ApiError::BadRequest(BadRequestError::UserAlreadyInvited));
        }
        let invite = DBWorkspaceInvite::create(workspace_id, user_id, &mut conn).await?;
        let ev = UserInviteEvent {
            user_id: user_id.clone(),
            workspace_id: workspace_id.clone(),
        };
        ev.send_event(&app.nats.js).await;
        Ok(invite.into())
    }

    pub async fn delete(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let invite = DBWorkspaceInvite::get(workspace_id, user_id, &mut conn).await?;
        let has_invite = invite
            .map(|invite| invite.deleted_at.is_none())
            .unwrap_or(false);
        if !has_invite {
            return Err(ApiError::NotFound("workspace_invite".to_string()));
        }
        DBWorkspaceInvite::delete(workspace_id, user_id, &mut conn).await?;
        let ev = RevokeInviteEvent {
            user_id: user_id.clone(),
            workspace_id: workspace_id.clone(),
        };
        ev.send_event(&app.nats.js).await;
        Ok(())
    }

    pub async fn users_for_invite_search(
        &self,
        search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
        workspace_id: &Uuid,
    ) -> ApiResult<Vec<UserListItemEntity>> {
        let app = &self.ctx.app;
        let limit = limit.unwrap_or(50).clamp(10, 200);
        let offset = offset.unwrap_or(0).max(0);
        let mut conn = app.db.acquire().await?;
        let users =
            DBUserListItem::list_for_invite(search, limit, offset, workspace_id, &mut conn).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn accept_invite(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let app = &self.ctx.app;
        let mut conn = app.db.begin().await?;
        let user = UserEntity::get_by_id(user_id, &app.redis, &mut conn).await?;
        let Some(user) = user else {
            return Err(ApiError::NotFound("user".to_string()));
        };
        if !user.confirmed || user.deleted_at.is_some() {
            return Err(ApiError::NotFound("user".to_string()));
        }
        let invite = DBWorkspaceInvite::get(workspace_id, user_id, &mut conn).await?;
        let member = DBWorkspaceMember::get(user_id, workspace_id, &mut conn).await?;
        if let Some(member) = member
            && member.deleted_at.is_none()
        {
            return Err(ApiError::BadRequest(BadRequestError::UserIsMember));
        }
        if let Some(invite) = invite {
            if invite.deleted_at.is_some() {
                return Err(ApiError::NotFound("workspace_invite".to_string()));
            }
            DBWorkspaceInvite::delete(workspace_id, user_id, &mut conn).await?;
            let created = DBWorkspaceMember::create(user_id, workspace_id, &mut conn).await?;
            conn.commit().await?;

            self.ctx.ws_registry_service()
                .add_workspace_member(workspace_id, user_id)
                .await?;
            let ev = AddMemberEvent {
                user_id: user.id,
                name: user.name,
                email: user.email,
                avatar: user.avatar.is_some(),
                member_since: created.created_at.max(created.updated_at),
                workspace_id: workspace_id.clone(),
            };
            ev.send_event(&app.nats.js).await;

            Ok(())
        } else {
            Err(ApiError::NotFound("workspace_invite".to_string()))
        }
    }
}
