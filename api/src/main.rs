use std::sync::Arc;

use api::{
    app_resources::AppResources,
    cli,
    config::Config,
    db::{init_pool, run_migrations},
    router::app_router,
    swagger::ApiDoc,
};
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::config::{Credentials, SharedCredentialsProvider};
use axum::Router;
use clap::Parser;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
    run_migrations(&db).await?;
    let nats = api::nats::init_nats_client(&config).await?;
    let redis = api::redis::create_redis_pool(&config.redis_url).await?;
    let config = Arc::new(config);
    let creds = Credentials::builder()
        .access_key_id(config.s3.access_key_id.clone())
        .secret_access_key(config.s3.secret_access_key.clone())
        .provider_name("local_provider")
        .build();
    let provider = SharedCredentialsProvider::new(creds);
    let s3_config = SdkConfig::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(config.s3.region.clone()))
        .endpoint_url(config.s3.endpoint.clone())
        .credentials_provider(provider)
        .build();
    let s3_config = aws_sdk_s3::config::Builder::from(&s3_config)
        .force_path_style(config.s3.force_path_style)
        .build();
    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    let res = AppResources::new(db, nats, redis, config.clone(), s3);

    let router = Router::new()
        .merge(
            app_router(res.clone()).layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            ),
        )
        .with_state(res)
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()));
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;
    tracing::info!("Listening on {}:{}", config.host, config.port);
    axum::serve(listener, router).await?;

    Ok(())
}
