use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "User invite")]
pub struct UserInviteResponse {
    pub workspace_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Workspace invite")]
pub struct WorkspaceInviteResponse {
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create invite")]
pub struct CreateInviteRequest {
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ts_rs::TS)]
#[ts(export)]
#[schema(description = "Create invite")]
pub struct DeleteInviteRequest {
    pub user_id: Uuid,
}
