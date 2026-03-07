use std::sync::Arc;

use uuid::Uuid;

use crate::{
    cache::RedisCache,
    config::Config,
    db::{DbPool, user::{DBUpdateUser, DBUser}},
    entity::user::UserEntity,
    error::{ApiError, ApiResult},
    redis::RedisClient,
};

#[derive(Debug, Clone)]
pub struct UserService {
    db: DbPool,
    redis: RedisClient,
    config: Arc<Config>,
}

impl UserService {
    pub fn new(db: DbPool, redis: RedisClient, config: Arc<Config>) -> Self {
        Self { db, redis, config }
    }

    pub async fn get(&self, id: &Uuid) -> ApiResult<Option<UserEntity>> {
        let mut conn = self.db.acquire().await?;
        UserEntity::get_by_id(id, &self.redis, &mut conn).await
    }

    pub async fn update(&self, id: &Uuid, name: String) -> ApiResult<UserEntity> {
        let mut conn = self.db.acquire().await?;
        let data = DBUpdateUser {
            name: Some(name),
            ..Default::default()
        };
        let updated = data.update(id, &mut conn).await?;
        if let Some(updated) = updated {
            let user = UserEntity::from(updated);
            user.cache(&self.redis).await?;
            Ok(user)
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }

    pub async fn change_password(&self, id: &Uuid, new_password: String) -> ApiResult<UserEntity> {
        let mut conn = self.db.acquire().await?;
        let data = DBUpdateUser {
            password: Some(new_password),
            ..Default::default()
        };
        let updated = data.update(id, &mut conn).await?;
        if let Some(updated) = updated {
            let user = UserEntity::from(updated);
            user.cache(&self.redis).await?;
            Ok(user)
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }
}

impl UserEntity {
    pub async fn get_by_id(
        id: &Uuid,
        redis: &RedisClient,
        db: &mut sqlx::PgConnection,
    ) -> ApiResult<Option<Self>> {
        let cached = UserEntity::cached(id, redis).await?;
        if let Some(cached) = cached {
            Ok(Some(cached))
        } else {
            let user = DBUser::get(id, db).await?;
            if let Some(user) = user {
                let entity = Self::from(user);
                entity.cache(redis).await?;
                Ok(Some(entity))
            } else {
                Ok(None)
            }
        }
    }
}
