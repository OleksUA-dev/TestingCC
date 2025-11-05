use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::Config, handlers, openapi::ApiDoc};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub config: Config,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))

        // Swagger UI - доступний на http://localhost:8001/swagger-ui/
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))

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

        // Workflow management routes
        .route("/api/v1/workflows", post(handlers::workflow::create_workflow))
        .route("/api/v1/workflows", get(handlers::workflow::list_workflows))
        .route("/api/v1/workflows/:id", get(handlers::workflow::get_workflow))
        .route("/api/v1/workflows/:id/full", get(handlers::workflow::get_workflow_full))
        .route("/api/v1/workflows/:id", delete(handlers::workflow::delete_workflow))

        // Workflow node routes
        .route("/api/v1/workflows/nodes", post(handlers::workflow::create_node))
        .route("/api/v1/workflows/:workflow_id/nodes", get(handlers::workflow::get_workflow_nodes))
        .route("/api/v1/workflows/nodes/:node_id", delete(handlers::workflow::delete_node))

        // Workflow edge routes
        .route("/api/v1/workflows/edges", post(handlers::workflow::create_edge))
        .route("/api/v1/workflows/:workflow_id/edges", get(handlers::workflow::get_workflow_edges))
        .route("/api/v1/workflows/edges/:edge_id", delete(handlers::workflow::delete_edge))

        // Workflow variable routes
        .route("/api/v1/workflows/:workflow_id/variables", get(handlers::workflow::get_workflow_variables))

        // Workflow validation
        .route("/api/v1/workflows/:workflow_id/validate", post(handlers::workflow::validate_workflow))

        // Workflow execution routes
        .route("/api/v1/workflows/:workflow_id/execute", post(handlers::workflow::execute_workflow))
        .route("/api/v1/workflows/:workflow_id/executions", get(handlers::workflow::get_workflow_executions))
        .route("/api/v1/executions/:execution_id", get(handlers::workflow::get_execution))

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
