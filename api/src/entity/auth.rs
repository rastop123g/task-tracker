use chrono::Utc;

use crate::{
    db,
    entity::user::UserEntity,
    protocol::auth::{LoginResponse, RefreshTokenResponse},
};

#[derive(Debug, Clone)]
pub struct RegisterUserEntity {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct TokensEntity {
    pub token: String,
    pub refresh_token: String,
    pub token_exp: chrono::DateTime<chrono::Utc>,
    pub refresh_exp: chrono::DateTime<chrono::Utc>,
    pub server_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct LoginnedUserEntity {
    pub user: UserEntity,
    pub tokens: TokensEntity,
}

impl From<crate::protocol::auth::RegisterRequest> for RegisterUserEntity {
    fn from(req: crate::protocol::auth::RegisterRequest) -> Self {
        Self {
            name: req.name.into(),
            email: req.email,
            password: req.password,
        }
    }
}

impl From<RegisterUserEntity> for db::user::DBNewUser {
    fn from(req: RegisterUserEntity) -> Self {
        Self {
            name: req.name,
            email: req.email,
            password: req.password,
            avatar: None,
            avatar_preview: None,
        }
    }
}

impl From<LoginnedUserEntity> for LoginResponse {
    fn from(user: LoginnedUserEntity) -> Self {
        Self {
            user_id: user.user.id,
            name: user.user.name,
            email: user.user.email,
            created_at: user.user.created_at,
            updated_at: user.user.updated_at,
            deleted_at: user.user.deleted_at,
            token: user.tokens.token,
            refresh_token: user.tokens.refresh_token,
            token_exp: user.tokens.token_exp,
            refresh_exp: user.tokens.refresh_exp,
            server_time: Utc::now(),
        }
    }
}

impl From<TokensEntity> for RefreshTokenResponse {
    fn from(tokens: TokensEntity) -> Self {
        Self {
            token: tokens.token,
            refresh_token: tokens.refresh_token,
            token_exp: tokens.token_exp,
            refresh_exp: tokens.refresh_exp,
            server_time: tokens.server_time,
        }
    }
}
