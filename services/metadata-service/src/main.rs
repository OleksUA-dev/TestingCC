mod config;
mod handlers;
mod repository;
mod routes;
mod service;
mod workflow;
mod openapi;

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "metadata_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Create database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    tracing::info!("Database connection pool created");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await?;

    tracing::info!("Database migrations completed");

    // Create shared state
    let state = Arc::new(crate::routes::AppState {
        db_pool: db_pool.clone(),
        config: config.clone(),
    });

    // Build router
    let app = routes::create_router(state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Metadata Service listening on {}", addr);

    axum::serve(listener, app)
        .await?;

    Ok(())
}
