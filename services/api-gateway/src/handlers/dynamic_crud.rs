use axum::{
    extract::{Path, Query, Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use shared::{models::Claims, CoreError};

use crate::routes::AppState;

use super::auth::AppError;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Create a new record in a dynamic entity
pub async fn create_record(
    State(state): State<Arc<AppState>>,
    Path(entity_name): Path<String>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    // Extract user from claims if authenticated
    let user_id = request
        .extensions()
        .get::<Claims>()
        .and_then(|c| c.user_id());

    // Parse request body
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| CoreError::InvalidInput("Failed to read request body".to_string()))?;
    let data: Value = serde_json::from_slice(&bytes)
        .map_err(|e| CoreError::InvalidInput(format!("Invalid JSON: {}", e)))?;

    // Create record
    let result = state
        .crud_service
        .create_record(&state.db_pool, &entity_name, data, user_id)
        .await?;

    Ok((StatusCode::CREATED, Json(result)))
}

/// Get a record by ID
pub async fn get_record(
    State(state): State<Arc<AppState>>,
    Path((entity_name, record_id)): Path<(String, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    let result = state
        .crud_service
        .get_record(&state.db_pool, &entity_name, record_id)
        .await?;

    Ok(Json(result))
}

/// List records
pub async fn list_records(
    State(state): State<Arc<AppState>>,
    Path(entity_name): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let results = state
        .crud_service
        .list_records(&state.db_pool, &entity_name, query.limit, query.offset)
        .await?;

    Ok(Json(results))
}

/// Update a record
pub async fn update_record(
    State(state): State<Arc<AppState>>,
    Path((entity_name, record_id)): Path<(String, Uuid)>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    // Extract user from claims if authenticated
    let user_id = request
        .extensions()
        .get::<Claims>()
        .and_then(|c| c.user_id());

    // Parse request body
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| CoreError::InvalidInput("Failed to read request body".to_string()))?;
    let data: Value = serde_json::from_slice(&bytes)
        .map_err(|e| CoreError::InvalidInput(format!("Invalid JSON: {}", e)))?;

    // Update record
    let result = state
        .crud_service
        .update_record(&state.db_pool, &entity_name, record_id, data, user_id)
        .await?;

    Ok(Json(result))
}

/// Delete a record
pub async fn delete_record(
    State(state): State<Arc<AppState>>,
    Path((entity_name, record_id)): Path<(String, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    state
        .crud_service
        .delete_record(&state.db_pool, &entity_name, record_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
