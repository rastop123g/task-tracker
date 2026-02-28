use std::time::Duration;

use anyhow::Result;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub type RedisClient = Pool<RedisConnectionManager>;

pub async fn create_redis_pool(redis_url: &str) -> Result<RedisClient> {
    let manager = RedisConnectionManager::new(redis_url)?;
    let pool = Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .build(manager)
        .await?;
    Ok(pool)
}
