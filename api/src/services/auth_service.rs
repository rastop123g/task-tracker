use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};

use crate::{
    cache::RedisCache,
    db::user::{DBNewUser, DBUser},
    entity::{
        auth::{LoginnedUserEntity, RegisterUserEntity, TokensEntity},
        user::UserEntity,
    },
    error::{ApiError, ApiResult, bad_request::BadRequestError, unauthotized::UnauthotizedError},
    jwt,
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    ctx: Ctx,
}

impl AuthService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn register(&self, data: RegisterUserEntity) -> ApiResult<()> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let users = DBUser::get_by_email(&data.email, &mut conn).await?;
        if users.iter().any(|u| u.confirmed) {
            return Err(ApiError::BadRequest(BadRequestError::EmailAlreadyUsed));
        }
        let sha = Sha256::digest(data.password.as_bytes());
        let hex_password = hex::encode(sha);
        let new_user = DBNewUser {
            name: data.name,
            email: data.email,
            password: hex_password,
            avatar: None,
            avatar_preview: None,
        };
        let created = new_user.create(&mut conn).await?;
        let confirmation_token_exp = Utc::now() + Duration::hours(3);
        let confirmation_token = jwt::create(&created.id, confirmation_token_exp, &app.config)?;
        let link = format!(
            "{}?token={}",
            app.config.validate_email_prefix, confirmation_token
        );
        tracing::debug!("confirm email link: {}", link);
        UserEntity::from(created).cache(&app.redis).await?;
        // TODO: send email task
        Ok(())
    }

    pub async fn login(&self, email: String, password: String) -> ApiResult<LoginnedUserEntity> {
        let app = &self.ctx.app;
        let password = Sha256::digest(password.as_bytes());
        let hex_password = hex::encode(password);
        let mut conn = app.db.acquire().await?;
        let users = DBUser::check_credentials(&email, &hex_password, &mut conn).await?;
        if users.is_empty() {
            return Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials));
        }
        if users.iter().all(|user| !user.confirmed) {
            return Err(ApiError::Unauthorized(UnauthotizedError::EmailNotConfirmed));
        }
        let user = users.into_iter().find(|user| user.confirmed);
        if let Some(user) = user {
            let user = UserEntity::from(user);
            if user.deleted_at.is_some() {
                return Err(ApiError::Unauthorized(UnauthotizedError::UserDeleted));
            }
            let token_exp = Utc::now() + chrono::Duration::days(3);
            let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
            let token = jwt::create(&user.id, token_exp, &app.config)?;
            let refresh_token = jwt::create(&user.id, refresh_token_exp, &app.config)?;
            Ok(LoginnedUserEntity {
                user,
                tokens: TokensEntity {
                    token,
                    refresh_token,
                    token_exp,
                    refresh_exp: refresh_token_exp,
                    server_time: Utc::now(),
                },
            })
        } else {
            Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials))
        }
    }

    pub async fn refresh(&self, refresh_token: &str) -> ApiResult<TokensEntity> {
        let app = &self.ctx.app;
        let user_id = jwt::verify(refresh_token, &app.config)?;
        let mut conn = app.db.acquire().await?;
        let user = UserEntity::get_by_id(&user_id, &app.redis, &mut conn).await?;
        if let Some(user) = user {
            user.check_user()?;
            let token_exp = Utc::now() + chrono::Duration::days(3);
            let refresh_token_exp = Utc::now() + chrono::Duration::days(365);
            let token = jwt::create(&user.id, token_exp, &app.config)?;
            let refresh_token = jwt::create(&user.id, refresh_token_exp, &app.config)?;
            Ok(TokensEntity {
                token,
                refresh_token,
                token_exp,
                refresh_exp: refresh_token_exp,
                server_time: Utc::now(),
            })
        } else {
            Err(ApiError::Unauthorized(UnauthotizedError::BadCredentials))
        }
    }

    pub async fn verify_email(&self, token: &str) -> ApiResult<()> {
        let app = &self.ctx.app;
        let user_id = jwt::verify(token, &app.config)?;
        let mut conn = app.db.acquire().await?;
        let user = UserEntity::get_by_id(&user_id, &app.redis, &mut conn).await?;
        if let Some(mut user) = user {
            if user.deleted_at.is_some() {
                return Err(ApiError::BadRequest(BadRequestError::UserDeleted));
            }
            if user.confirmed {
                return Err(ApiError::BadRequest(BadRequestError::UserAlreadyConfirmed));
            }
            DBUser::confirm_email(&user_id, &mut conn).await?;
            user.confirmed = true;
            user.cache(&app.redis).await?;
            Ok(())
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }
}
