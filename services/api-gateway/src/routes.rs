use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};

use crate::{config::Config, handlers, middleware::auth, service::dynamic_crud_service::DynamicCrudService};

pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
    pub crud_service: DynamicCrudService,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route("/api/v1/data/:entity_name", post(handlers::dynamic_crud::create_record))
        .route("/api/v1/data/:entity_name", get(handlers::dynamic_crud::list_records))
        .route("/api/v1/data/:entity_name/:record_id", get(handlers::dynamic_crud::get_record))
        .route("/api/v1/data/:entity_name/:record_id", put(handlers::dynamic_crud::update_record))
        .route("/api/v1/data/:entity_name/:record_id", delete(handlers::dynamic_crud::delete_record))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::require_auth,
        ));

    // Combine routes
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
}
