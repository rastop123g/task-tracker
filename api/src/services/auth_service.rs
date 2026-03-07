use std::sync::Arc;

use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};

use crate::{
    cache::RedisCache,
    config::Config,
    db::{
        DbPool,
        user::{DBNewUser, DBUser},
    },
    entity::{
        auth::{LoginnedUserEntity, RegisterUserEntity, TokensEntity},
        user::UserEntity,
    },
    error::{ApiError, ApiResult, bad_request::BadRequestError, unauthotized::UnauthotizedError},
    jwt,
    redis::RedisClient,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    db: DbPool,
    redis: RedisClient,
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(db: DbPool, redis: RedisClient, config: Arc<Config>) -> Self {
        Self { db, redis, config }
    }

    pub async fn register(&self, data: RegisterUserEntity) -> ApiResult<()> {
        let mut conn = self.db.acquire().await?;
        let users = DBUser::get_by_email(&data.email, &mut conn).await?;
        if users.iter().any(|u| u.confirmed == true) {
            return Err(ApiError::BadRequest(BadRequestError::EmailAlreadyUsed));
        }
        let new_user = DBNewUser::from(data);
        let created = new_user.create(&mut conn).await?;
        let confirmation_token_exp = Utc::now() + Duration::hours(3);
        let confirmation_token = jwt::create(&created.id, confirmation_token_exp, &self.config)?;
        let link = format!(
            "{}?token={}",
            self.config.validate_email_prefix, confirmation_token
        );
        tracing::debug!("confirm email link: {}", link);
        // TODO: send email task
        Ok(())
    }

    pub async fn login(&self, email: String, password: String) -> ApiResult<LoginnedUserEntity> {
        let password = Sha256::digest(password.as_bytes());
        let hex_password = hex::encode(password);
        let mut conn = self.db.acquire().await?;
        let users = DBUser::check_credentials(&email, &hex_password, &mut conn).await?;
        if users.len() == 0 {
            return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
        }
        if users.iter().all(|user| user.confirmed == false) {
            return Err(ApiError::Unauthorized(UnauthotizedError::EmailNotConfirmed));
        }
        let user = users.into_iter().find(|user| user.confirmed == true);
        if let Some(user) = user {
            let user = UserEntity::from(user);
            if user.deleted_at.is_some() {
                return Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted));
            }
            let token_exp = Utc::now() + chrono::Duration::days(3);
            let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
            let token = jwt::create(&user.id, token_exp, &self.config)?;
            let refresh_token = jwt::create(&user.id, refresh_token_exp, &self.config)?;
            Ok(LoginnedUserEntity {
                user,
                tokens: TokensEntity {
                    token,
                    refresh_token,
                    token_exp: token_exp,
                    refresh_exp: refresh_token_exp,
                    server_time: Utc::now(),
                },
            })
        } else {
            return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
        }
    }

    pub async fn refresh(&self, refresh_token: &str) -> ApiResult<TokensEntity> {
        let user_id = jwt::verify(refresh_token, &self.config)?;
        let mut conn = self.db.acquire().await?;
        let user = UserEntity::get_by_id(&user_id, &self.redis, &mut conn).await?;
        if let Some(user) = user {
            user.check_user()?;
            let token_exp = Utc::now() + chrono::Duration::days(3);
            let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
            let token = jwt::create(&user.id, token_exp, &self.config)?;
            let refresh_token = jwt::create(&user.id, refresh_token_exp, &self.config)?;
            Ok(TokensEntity {
                token,
                refresh_token,
                token_exp: token_exp,
                refresh_exp: refresh_token_exp,
                server_time: Utc::now(),
            })
        } else {
            return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
        }
    }

    pub async fn verify_email(&self, token: &str) -> ApiResult<()> {
        let user_id = jwt::verify(&token, &self.config)?;
        let mut conn = self.db.acquire().await?;
        let user = UserEntity::get_by_id(&user_id, &self.redis, &mut conn).await?;
        if let Some(mut user) = user {
            if user.deleted_at.is_some() {
                return Err(ApiError::BadRequest(BadRequestError::UserDeleted));
            }
            if user.confirmed == true {
                return Err(ApiError::BadRequest(BadRequestError::UserAlreadyConfirmed));
            }
            DBUser::confirm_email(&user_id, &mut conn).await?;
            user.confirmed = true;
            user.cache(&self.redis).await?;
            Ok(())
        } else {
            return Err(ApiError::NotFound("user".to_string()));
        }
    }
}
