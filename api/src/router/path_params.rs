use serde::Deserialize;
use uuid::Uuid;

pub trait WorkspaceIdFromPathParams {
    fn workspace_id(self) -> Uuid;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkspacePathParams {
    pub workspace_id: Uuid,
}

impl WorkspaceIdFromPathParams for WorkspacePathParams {
    fn workspace_id(self) -> Uuid {
        self.workspace_id
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserPathParams {
    pub user_id: Uuid,
}
