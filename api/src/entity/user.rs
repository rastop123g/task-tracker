use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    cache::RedisCache,
    db::user::{DBUser, DBUserListItem},
    error::{ApiError, ApiResult, bad_request::BadRequestError, unauthotized::UnauthotizedError},
    protocol::user::{UserListItemResponse, UserResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirmed: bool,
    pub avatar: Option<String>,
    pub avatar_preview: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserEntity {
    pub fn check_user(&self) -> ApiResult<()> {
        if self.deleted_at.is_some() {
            return Err(ApiError::BadRequest(BadRequestError::UserDeleted));
        }
        if self.confirmed == false {
            return Err(ApiError::BadRequest(BadRequestError::UserNotConfirmed));
        }
        Ok(())
    }

    pub fn check_self(&self) -> ApiResult<()> {
        if self.deleted_at.is_some() {
            return Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted));
        }
        if self.confirmed == false {
            return Err(ApiError::Unauthorized(UnauthotizedError::EmailNotConfirmed));
        }
        Ok(())
    }
}

impl From<DBUser> for UserEntity {
    fn from(user: DBUser) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password,
            confirmed: user.confirmed,
            avatar: user.avatar,
            avatar_preview: user.avatar_preview,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

impl RedisCache<Uuid> for UserEntity {
    fn cache_key(&self) -> String {
        Self::key_from(&self.id)
    }

    fn key_from(id: &Uuid) -> String {
        format!("user-entity:{}", id)
    }

    fn cache_exp(&self) -> u64 {
        60 * 60 * 24 * 7 // 7 days
    }
}

impl From<UserEntity> for UserResponse {
    fn from(user: UserEntity) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            avatar: user.avatar,
            avatar_preview: user.avatar_preview,
            created_at: user.created_at,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserListItemEntity {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl From<DBUserListItem> for UserListItemEntity {
    fn from(user: DBUserListItem) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

impl From<UserListItemEntity> for UserListItemResponse {
    fn from(user: UserListItemEntity) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
