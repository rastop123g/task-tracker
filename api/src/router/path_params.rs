use serde::Deserialize;
use uuid::Uuid;

pub trait WorkspaceIdFromPathParams {
    fn workspace_id(self) -> Uuid;
}

pub trait StatusIdFromPathParams {
    fn status_id(self) -> Uuid;
}

pub trait UserIdFromPathParams {
    fn user_id(self) -> Uuid;
}

pub trait TagIdFromPathParams {
    fn tag_id(self) -> Uuid;
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkspacePathParams {
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserPathParams {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusPathParams {
    pub workspace_id: Uuid,
    pub status_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TagPathParams {
    pub workspace_id: Uuid,
    pub tag_id: Uuid,
}

impl WorkspaceIdFromPathParams for TagPathParams {
    fn workspace_id(self) -> Uuid {
        self.workspace_id
    }
}

impl TagIdFromPathParams for TagPathParams {
    fn tag_id(self) -> Uuid {
        self.tag_id
    }
}

impl WorkspaceIdFromPathParams for WorkspacePathParams {
    fn workspace_id(self) -> Uuid {
        self.workspace_id
    }
}

impl WorkspaceIdFromPathParams for StatusPathParams {
    fn workspace_id(self) -> Uuid {
        self.workspace_id
    }
}

impl StatusIdFromPathParams for StatusPathParams {
    fn status_id(self) -> Uuid {
        self.status_id
    }
}

impl UserIdFromPathParams for UserPathParams {
    fn user_id(self) -> Uuid {
        self.user_id
    }
}
