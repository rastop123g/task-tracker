use std::sync::Arc;

use crate::{router::extractors::req_ctx::Ctx, websocket::WsSessionMap};

#[derive(Clone, Debug)]
pub struct AppResources {
    pub db: sqlx::PgPool,
    pub nats: Arc<crate::nats::NatsClient>,
    pub redis: crate::redis::RedisClient,
    pub config: Arc<crate::config::Config>,
    pub s3: aws_sdk_s3::Client,
    pub ws_sessions: Arc<WsSessionMap>,
}

impl AppResources {
    pub fn new(
        db: sqlx::PgPool,
        nats: Arc<crate::nats::NatsClient>,
        redis: crate::redis::RedisClient,
        config: Arc<crate::config::Config>,
        s3: aws_sdk_s3::Client,
    ) -> Self {
        Self {
            ws_sessions: WsSessionMap::new(),
            db,
            nats,
            redis,
            config,
            s3,
        }
    }

    pub fn ctx(self) -> Ctx {
        Ctx::new(self)
    }
}
