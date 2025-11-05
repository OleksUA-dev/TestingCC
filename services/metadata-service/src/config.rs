use serde::Deserialize;
use config::{Config as ConfigLoader, ConfigError, Environment, File};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
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

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let config = ConfigLoader::builder()
            // Default values
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8001)?
            .set_default("database.max_connections", 10)?
            // Load from config file if present
            .add_source(File::with_name("config").required(false))
            // Override with environment variables (with prefix METADATA_)
            .add_source(Environment::with_prefix("METADATA").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
