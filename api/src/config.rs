#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
    pub env_filter: String,
    pub nats_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn parse() -> Self {
        Config {
            host: std::env::var("HTTP_HOST").unwrap_or(String::from("0.0.0.0")),
            port: std::env::var("HTTP_PORT").unwrap_or(String::from("8045")).parse().unwrap(),
            database_url: std::env::var("DATABASE_URL").unwrap_or(String::from("postgres://postgres:postgres@localhost:5432/task_tracker")),
            log_level: std::env::var("APP_LOG").unwrap_or(String::from("debug")),
            env_filter: std::env::var("APP_FILTER").unwrap_or(String::from("debug")),
            nats_url: std::env::var("NATS_URL").unwrap_or(String::from("nats://localhost:4222")),
            redis_url: std::env::var("REDIS_URL").unwrap_or(String::from("redis://localhost:6379")),
        }
    }
}
