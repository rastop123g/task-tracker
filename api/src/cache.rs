use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    error::{ApiError, ApiResult},
    redis::RedisClient,
};

pub trait RedisCache<T>: Clone + std::fmt::Debug + Serialize + DeserializeOwned {
    fn cache_key(&self) -> String;
    fn cache_exp(&self) -> u64;
    fn key_from(id: &T) -> String;

    async fn cached(key: &T, client: &RedisClient) -> ApiResult<Option<Self>> {
        let key = Self::key_from(key);
        let mut r = client.get().await?;
        let value: Option<String> = r.get(key).await?;
        if let Some(value) = value {
            let value: Self =
                serde_json::from_str(&value).map_err(|_| ApiError::InternalServerError)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
    async fn cache(&self, client: &RedisClient) -> ApiResult<()> {
        let value = serde_json::to_string(&self).map_err(|_| ApiError::InternalServerError)?;
        let mut r = client.get().await?;
        let key = self.cache_key();
        let _: () = r.set_ex(key, value, self.cache_exp()).await?;
        Ok(())
    }

    async fn clear(&self, client: &RedisClient) -> ApiResult<()> {
        let mut r = client.get().await?;
        let key = self.cache_key();
        let _: () = r.del(key).await?;
        Ok(())
    }

    async fn clear_id(id: &T, client: &RedisClient) -> ApiResult<()> {
        let key = Self::key_from(id);
        let mut r = client.get().await?;
        let _: () = r.del(key).await?;
        Ok(())
    }
}
