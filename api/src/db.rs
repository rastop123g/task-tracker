use std::time::Duration;
use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod user;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
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
