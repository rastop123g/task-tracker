use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub struct UpdateUserEvent {
    pub user_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub struct AddMemberEvent {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    /// Превью аватарки
    pub avatar: bool,
    /// Дата приглашения (вступления)
    pub member_since: chrono::DateTime<chrono::Utc>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub struct RemoveMemberEvent {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub struct UserInviteEvent {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub struct RevokeInviteEvent {
    pub user_id: Uuid,
    pub workspace_id: Uuid,
}
