use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ts_rs::TS)]
pub enum ForbiddenError {
    OwnerOnly,
    WorkspaceAdminOnly,
}
