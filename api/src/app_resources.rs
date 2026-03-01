use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppResources {
    pub db: sqlx::PgPool,
    pub nats: crate::nats::NatsClient,
    pub redis: crate::redis::RedisClient,
    pub config: Arc<crate::config::Config>,
}
