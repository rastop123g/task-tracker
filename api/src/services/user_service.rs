use std::sync::Arc;

use uuid::Uuid;

use crate::{
    cache::RedisCache,
    config::Config,
    db::{DbPool, user::DBUser},
    entity::user::UserEntity,
    error::ApiResult,
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
