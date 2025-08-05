use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub host: String,
    pub upload_dir: String,
    pub max_file_size: usize,
    pub stripe_secret_key: Option<String>,
    pub stripe_webhook_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            upload_dir: std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string()),
            max_file_size: std::env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "10485760".to_string()) // 10MB
                .parse()?,
            stripe_secret_key: std::env::var("STRIPE_SECRET_KEY").ok(),
            stripe_webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").ok(),
        })
    }
}
