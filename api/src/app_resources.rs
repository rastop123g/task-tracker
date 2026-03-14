use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppResources {
    pub db: sqlx::PgPool,
    pub nats: crate::nats::NatsClient,
    pub redis: crate::redis::RedisClient,
    pub config: Arc<crate::config::Config>,
    pub s3: aws_sdk_s3::Client,

    pub auth_service: crate::services::auth_service::AuthService,
    pub user_service: crate::services::user_service::UserService,
    pub workspace_service: crate::services::workspace_service::WorkspaceService,
    pub workspace_invite_service: crate::services::workspace_invite_service::WorkspaceInviteService,
}

impl AppResources {
    pub fn new(
        db: sqlx::PgPool,
        nats: crate::nats::NatsClient,
        redis: crate::redis::RedisClient,
        config: Arc<crate::config::Config>,
        s3: aws_sdk_s3::Client,
    ) -> Self {
        Self {
            // DI
            auth_service: crate::services::auth_service::AuthService::new(
                db.clone(),
                redis.clone(),
                config.clone(),
            ),
            user_service: crate::services::user_service::UserService::new(
                db.clone(),
                redis.clone(),
                config.clone(),
            ),
            workspace_service: crate::services::workspace_service::WorkspaceService::new(
                db.clone(),
                redis.clone(),
                config.clone(),
            ),
            workspace_invite_service:
                crate::services::workspace_invite_service::WorkspaceInviteService::new(
                    db.clone(),
                    redis.clone(),
                    config.clone(),
                ),
            db,
            nats,
            redis,
            config,
            s3,
        }
    }
}
