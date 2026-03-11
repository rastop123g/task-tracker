use chrono::Utc;

use crate::{
    db,
    entity::user::UserEntity,
    error::{ApiError, validation::ValidationError},
    protocol::auth::{LoginResponse, RefreshTokenResponse},
    validation::{AppValidateEmail, ValidateStringLength},
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

impl TryFrom<crate::protocol::auth::RegisterRequest> for RegisterUserEntity {
    type Error = crate::error::ApiError;
    fn try_from(req: crate::protocol::auth::RegisterRequest) -> Result<Self, Self::Error> {
        let mut errs = Vec::new();
        if let Err(e) = req.name.length(3, 128) {
            errs.push(ValidationError("name", e));
        }
        if let Err(e) = req.email.validate_email() {
            errs.push(ValidationError("email", e));
        }
        if let Err(e) = req.password.length(8, 128) {
            errs.push(ValidationError("password", e));
        }
        if errs.len() > 0 {
            return Err(ApiError::Validation(errs));
        }
        Ok(Self {
            name: req.name,
            email: req.email,
            password: req.password,
        })
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
            avatar: user.user.avatar,
            avatar_preview: user.user.avatar_preview,
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
