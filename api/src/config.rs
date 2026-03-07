#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
    pub env_filter: String,
    pub nats_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub validate_email_prefix: String,
    pub s3: S3Config,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct S3Config {
    pub endpoint: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
    pub region: String,
    pub force_path_style: bool,
}

impl Config {
    pub fn parse() -> Self {
        Config {
            host: std::env::var("HTTP_HOST").unwrap_or(String::from("0.0.0.0")),
            port: std::env::var("HTTP_PORT")
                .unwrap_or(String::from("8045"))
                .parse()
                .unwrap(),
            database_url: std::env::var("DATABASE_URL").unwrap_or(String::from(
                "postgres://postgres:postgres@localhost:5432/task_tracker",
            )),
            log_level: std::env::var("APP_LOG").unwrap_or(String::from("debug")),
            env_filter: std::env::var("APP_FILTER").unwrap_or(String::from("debug")),
            nats_url: std::env::var("NATS_URL").unwrap_or(String::from("nats://localhost:4222")),
            redis_url: std::env::var("REDIS_URL").unwrap_or(String::from("redis://localhost:6379")),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or(String::from("secret")),
            validate_email_prefix: std::env::var("VALIDATE_EMAIL_PREFIX")
                .unwrap_or(String::from("http://localhost:8045/api/v1/auth/verify")),
            s3: S3Config {
                endpoint: std::env::var("S3_ENDPOINT")
                    .unwrap_or(String::from("http://localhost:9000")),
                access_key_id: std::env::var("S3_ACCESS_KEY_ID")
                    .unwrap_or(String::from("minioadmin")),
                secret_access_key: std::env::var("S3_SECRET_ACCESS_KEY")
                    .unwrap_or(String::from("minioadmin")),
                bucket: std::env::var("S3_BUCKET").unwrap_or(String::from("tasktrackerfiles")),
                region: std::env::var("S3_REGION").unwrap_or(String::from("us-east-1")),
                force_path_style: std::env::var("S3_FORCE_PATH_STYLE")
                    .unwrap_or(String::from("false"))
                    .parse()
                    .unwrap(),
            },
        }
    }
}
