use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use shared::{
    models::workflow::{CreateWorkflowRequest, CreateNodeRequest, CreateEdgeRequest},
    CoreError,
};

use crate::{
    routes::AppState,
    service::workflow_service,
};

// ============================================================================
// Workflow Endpoints
// ============================================================================

/// Create a new workflow
pub async fn create_workflow(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkflowRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let workflow = workflow_service::create_workflow(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(workflow)))
}

/// List all workflows
pub async fn list_workflows(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let workflows = workflow_service::list_workflows(&state.db_pool).await?;

    Ok(Json(workflows))
}

/// Get workflow by ID
pub async fn get_workflow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let workflow = workflow_service::get_workflow(&state.db_pool, id).await?;

    Ok(Json(workflow))
}

/// Get full workflow definition with all components
pub async fn get_workflow_full(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let full_def = workflow_service::get_workflow_full(&state.db_pool, id).await?;

    Ok(Json(full_def))
}

/// Delete workflow
pub async fn delete_workflow(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    workflow_service::delete_workflow(&state.db_pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Node Endpoints
// ============================================================================

/// Create a workflow node
pub async fn create_node(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let node = workflow_service::create_node(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(node)))
}

/// Get workflow nodes
pub async fn get_workflow_nodes(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let nodes = workflow_service::get_workflow_nodes(&state.db_pool, workflow_id).await?;

    Ok(Json(nodes))
}

/// Delete workflow node
pub async fn delete_node(
    State(state): State<Arc<AppState>>,
    Path(node_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    workflow_service::delete_node(&state.db_pool, node_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Edge Endpoints
// ============================================================================

/// Create a workflow edge
pub async fn create_edge(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEdgeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let edge = workflow_service::create_edge(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(edge)))
}

/// Get workflow edges
pub async fn get_workflow_edges(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let edges = workflow_service::get_workflow_edges(&state.db_pool, workflow_id).await?;

    Ok(Json(edges))
}

/// Delete workflow edge
pub async fn delete_edge(
    State(state): State<Arc<AppState>>,
    Path(edge_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    workflow_service::delete_edge(&state.db_pool, edge_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Variable Endpoints
// ============================================================================

/// Get workflow variables
pub async fn get_workflow_variables(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let variables = workflow_service::get_workflow_variables(&state.db_pool, workflow_id).await?;

    Ok(Json(variables))
}

// ============================================================================
// Validation Endpoint
// ============================================================================

/// Validate workflow configuration
pub async fn validate_workflow(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = workflow_service::validate_workflow(&state.db_pool, workflow_id).await?;

    Ok(Json(result))
}

// ============================================================================
// Execution Endpoints
// ============================================================================

/// Execute a workflow
pub async fn execute_workflow(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
    Json(initial_context): Json<Option<std::collections::HashMap<String, serde_json::Value>>>,
) -> Result<impl IntoResponse, AppError> {
    let execution = workflow_service::execute_workflow(
        &state.db_pool,
        workflow_id,
        initial_context,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(execution)))
}

/// Get execution by ID
pub async fn get_execution(
    State(state): State<Arc<AppState>>,
    Path(execution_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let execution = workflow_service::get_execution(&state.db_pool, execution_id).await?;

    Ok(Json(execution))
}

/// Get workflow executions
pub async fn get_workflow_executions(
    State(state): State<Arc<AppState>>,
    Path(workflow_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let executions = workflow_service::get_workflow_executions(&state.db_pool, workflow_id).await?;

    Ok(Json(executions))
}

// ============================================================================
// Error Handling
// ============================================================================

/// Error wrapper for proper HTTP responses
pub struct AppError(CoreError);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.0.status_code()).unwrap();
        let body = Json(serde_json::json!({
            "error": self.0.to_string()
        }));

        (status_code, body).into_response()
    }
}

impl From<CoreError> for AppError {
    fn from(err: CoreError) -> Self {
        AppError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError(CoreError::Validation(err.to_string()))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError(CoreError::Internal(err.to_string()))
    }
}
