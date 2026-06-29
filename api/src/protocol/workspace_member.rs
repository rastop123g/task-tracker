use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Workspace member")]
pub struct WorkspaceMemberResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    /// Превью аватарки
    pub avatar: bool,
    /// Дата приглашения (вступления)
    pub member_since: chrono::DateTime<chrono::Utc>,
    /// Дата исключения (выхода)
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
