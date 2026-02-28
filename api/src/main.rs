use std::sync::Arc;

use api::{app_resources::AppResources, cli, config::Config, db::init_pool, router::app_router};
use axum::Router;
use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Serve => {
            serve().await?;
        }
    }
    Ok(())
}

async fn serve() -> anyhow::Result<()> {
    let config = Config::parse();
    let env_filter = {
        let first = EnvFilter::try_from_default_env();
        if let Ok(first) = first {
            first
        } else {
            EnvFilter::try_from(&config.env_filter).unwrap_or_else(|_| {
                format!("{}={}", env!("CARGO_CRATE_NAME"), config.log_level).into()
            })
        }
    };
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer())
        .init();

    let db = init_pool(&config.database_url).await?;
    let nats = api::nats::init_nats_client(&config).await?;
    let redis = api::redis::create_redis_pool(&config.redis_url).await?;
    let config = Arc::new(config);

    let res = AppResources { db, nats, redis, config: config.clone() };

    let router = app_router(res);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
