use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};

use crate::{config::Config, handlers};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))

        // Entity management routes
        .route("/api/v1/entities", post(handlers::entity::create_entity))
        .route("/api/v1/entities", get(handlers::entity::list_entities))
        .route("/api/v1/entities/:id", get(handlers::entity::get_entity))
        .route("/api/v1/entities/:id", put(handlers::entity::update_entity))
        .route("/api/v1/entities/:id", delete(handlers::entity::delete_entity))

        // Attribute management routes
        .route("/api/v1/attributes", post(handlers::attribute::create_attribute))
        .route("/api/v1/entities/:entity_id/attributes", get(handlers::attribute::list_attributes))
        .route("/api/v1/attributes/:id", get(handlers::attribute::get_attribute))
        .route("/api/v1/attributes/:id", put(handlers::attribute::update_attribute))
        .route("/api/v1/attributes/:id", delete(handlers::attribute::delete_attribute))

        // Schema generation
        .route("/api/v1/entities/:id/schema", post(handlers::schema::generate_schema))

        // Add state
        .with_state(state)

        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
}
