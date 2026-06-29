use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{db::workspace_member::DBWorkspaceMemberWithUser, entity::{user::UserEntity, workspace::WorkspaceEntity}, protocol::workspace_member::WorkspaceMemberResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMemberEntity {
    pub user: UserEntity,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<DBWorkspaceMemberWithUser> for WorkspaceMemberEntity {
    fn from(wm: DBWorkspaceMemberWithUser) -> Self {
        Self {
            user: UserEntity {
                id: wm.user_id,
                name: wm.user_name,
                email: wm.user_email,
                avatar: wm.user_avatar,
                avatar_preview: wm.user_avatar_preview,
                password: "hidden".to_string(),
                confirmed: wm.user_confirmed,
                created_at: wm.user_created_at,
                updated_at: wm.user_updated_at,
                deleted_at: wm.user_deleted_at,
            },
            created_at: wm.created_at,
            updated_at: wm.updated_at,
            deleted_at: wm.deleted_at,
        }
    }
}

impl From<WorkspaceMemberEntity> for WorkspaceMemberResponse {
    fn from(wm: WorkspaceMemberEntity) -> Self {
        Self {
            id: wm.user.id,
            name: wm.user.name,
            email: wm.user.email,
            avatar: wm.user.avatar.is_some(),
            member_since: wm.created_at.max(wm.updated_at),
            deleted_at: wm.deleted_at,
        }
    }
}
