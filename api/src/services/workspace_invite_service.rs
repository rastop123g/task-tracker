use std::sync::Arc;

use uuid::Uuid;

use crate::{
    config::Config,
    db::{
        DbPool, user::DBUserListItem, workspace_invite::DBWorkspaceInvite,
        workspace_member::DBWorkspaceMember,
    },
    entity::{user::UserListItemEntity, workspace_invite::WorkspaceInviteEntity},
    error::{ApiError, ApiResult, bad_request::BadRequestError},
    redis::RedisClient,
};

#[derive(Debug, Clone)]
pub struct WorkspaceInviteService {
    db: DbPool,
    redis: RedisClient,
    config: Arc<Config>,
}

impl WorkspaceInviteService {
    pub fn new(db: DbPool, redis: RedisClient, config: Arc<Config>) -> Self {
        Self { db, redis, config }
    }

    pub async fn get_user_invitations(
        &self,
        user_id: &Uuid,
    ) -> ApiResult<Vec<WorkspaceInviteEntity>> {
        let mut conn = self.db.acquire().await?;
        let workspace_invites = DBWorkspaceInvite::get_by_user_id(user_id, &mut conn).await?;
        Ok(workspace_invites.into_iter().map(Into::into).collect())
    }

    pub async fn get_workspace_invitations(
        &self,
        workspace_id: &Uuid,
    ) -> ApiResult<Vec<WorkspaceInviteEntity>> {
        let mut conn = self.db.acquire().await?;
        let workspace_invites =
            DBWorkspaceInvite::get_by_workspace_id(workspace_id, &mut conn).await?;
        Ok(workspace_invites.into_iter().map(Into::into).collect())
    }

    pub async fn create(
        &self,
        workspace_id: &Uuid,
        user_id: &Uuid,
    ) -> ApiResult<WorkspaceInviteEntity> {
        let mut conn = self.db.acquire().await?;
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
        Ok(invite.into())
    }

    pub async fn delete(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.db.acquire().await?;
        let invite = DBWorkspaceInvite::get(workspace_id, user_id, &mut conn).await?;
        let has_invite = invite
            .map(|invite| invite.deleted_at.is_none())
            .unwrap_or(false);
        if !has_invite {
            return Err(ApiError::NotFound("workspace_invite".to_string()));
        }
        DBWorkspaceInvite::delete(workspace_id, user_id, &mut conn).await?;
        Ok(())
    }

    pub async fn users_for_invite_search(
        &self,
        search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
        workspace_id: &Uuid,
    ) -> ApiResult<Vec<UserListItemEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let mut conn = self.db.acquire().await?;
        let users =
            DBUserListItem::list_for_invite(search, limit, offset, workspace_id, &mut conn).await?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    pub async fn accept_invite(&self, workspace_id: &Uuid, user_id: &Uuid) -> ApiResult<()> {
        let mut conn = self.db.begin().await?;
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
            DBWorkspaceMember::create(user_id, workspace_id, &mut conn).await?;
            conn.commit().await?;
            Ok(())
        } else {
            return Err(ApiError::NotFound("workspace_invite".to_string()));
        }
    }
}
