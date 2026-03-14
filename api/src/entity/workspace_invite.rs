use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    db::workspace_invite::DBWorkspaceInvite,
    protocol::workspace_invite::{UserInviteResponse, WorkspaceInviteResponse},
};

#[derive(Debug, Clone)]
pub struct WorkspaceInviteEntity {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct WorkspaceInviteParamsEntity {
    pub workspace_id: Uuid,
    pub user_id: Uuid,
}

impl From<DBWorkspaceInvite> for WorkspaceInviteEntity {
    fn from(workspace_invite: DBWorkspaceInvite) -> Self {
        Self {
            workspace_id: workspace_invite.workspace_id,
            user_id: workspace_invite.user_id,
            created_at: workspace_invite.created_at,
            updated_at: workspace_invite.updated_at,
            deleted_at: workspace_invite.deleted_at,
        }
    }
}

impl From<WorkspaceInviteEntity> for UserInviteResponse {
    fn from(workspace_invite: WorkspaceInviteEntity) -> Self {
        Self {
            workspace_id: workspace_invite.workspace_id,
            created_at: workspace_invite.created_at,
            updated_at: workspace_invite.updated_at,
        }
    }
}

impl From<WorkspaceInviteEntity> for WorkspaceInviteResponse {
    fn from(workspace_invite: WorkspaceInviteEntity) -> Self {
        Self {
            user_id: workspace_invite.user_id,
            created_at: workspace_invite.created_at,
            updated_at: workspace_invite.updated_at,
        }
    }
}
