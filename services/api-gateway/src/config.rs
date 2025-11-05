use serde::Deserialize;
use config::{Config as ConfigLoader, ConfigError, Environment, File};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub metadata_service: MetadataServiceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MetadataServiceConfig {
    pub url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let config = ConfigLoader::builder()
            // Default values
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8000)?
            .set_default("database.max_connections", 10)?
            .set_default("jwt.secret", "your-secret-key-change-in-production")?
            .set_default("jwt.expiration_hours", 24)?
            .set_default("metadata_service.url", "http://localhost:8001")?
            // Load from config file if present
            .add_source(File::with_name("config").required(false))
            // Override with environment variables (with prefix GATEWAY_)
            .add_source(Environment::with_prefix("GATEWAY").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
