use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub mod common;
pub mod status;
pub mod tag;
pub mod user;
pub mod workspace;
pub mod workspace_invite;
pub mod workspace_member;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .idle_timeout(Duration::from_secs(900))
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("migrate error")?;
    Ok(())
}
