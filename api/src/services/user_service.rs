use uuid::Uuid;

use crate::{
    cache::RedisCache,
    db::user::{DBUpdateUser, DBUser, DBUserListItem},
    entity::user::{UserEntity, UserListItemEntity},
    error::{ApiError, ApiResult},
    redis::RedisClient,
    router::extractors::req_ctx::Ctx,
};

#[derive(Debug, Clone)]
pub struct UserService {
    ctx: Ctx,
}

impl UserService {
    pub fn new(ctx: Ctx) -> Self {
        Self { ctx }
    }

    pub async fn get(&self, id: &Uuid) -> ApiResult<UserEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let user = UserEntity::get_by_id(id, &app.redis, &mut conn).await?;
        if let Some(user) = user {
            if user.deleted_at.is_some() {
                return Err(ApiError::NotFound("user".to_string()));
            }
            if !user.confirmed {
                return Err(ApiError::NotFound("user".to_string()));
            }
            Ok(user)
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }

    pub async fn update(&self, id: &Uuid, name: String) -> ApiResult<UserEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let data = DBUpdateUser {
            name: Some(name),
            ..Default::default()
        };
        let updated = data.update(id, &mut conn).await?;
        if let Some(updated) = updated {
            let user = UserEntity::from(updated);
            user.cache(&app.redis).await?;
            Ok(user)
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }

    pub async fn change_password(&self, id: &Uuid, new_password: String) -> ApiResult<UserEntity> {
        let app = &self.ctx.app;
        let mut conn = app.db.acquire().await?;
        let data = DBUpdateUser {
            password: Some(new_password),
            ..Default::default()
        };
        let updated = data.update(id, &mut conn).await?;
        if let Some(updated) = updated {
            let user = UserEntity::from(updated);
            user.cache(&app.redis).await?;
            Ok(user)
        } else {
            Err(ApiError::NotFound("user".to_string()))
        }
    }

    pub async fn users_search(
        &self,
        search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ApiResult<Vec<UserListItemEntity>> {
        let app = &self.ctx.app;
        let limit = limit.unwrap_or(50).clamp(10, 200);
        let offset = offset.unwrap_or(0).max(0);
        let mut conn = app.db.acquire().await?;
        let users = DBUserListItem::list(search, limit, offset, &mut conn).await?;
        Ok(users.into_iter().map(Into::into).collect())
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
