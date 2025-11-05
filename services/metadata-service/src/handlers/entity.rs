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
    models::{CreateEntityRequest, UpdateEntityRequest, Entity},
    CoreError,
};

use crate::{
    routes::AppState,
    service::entity_service,
};

/// Create a new entity
pub async fn create_entity(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateEntityRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    // Create entity
    let entity = entity_service::create_entity(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(entity)))
}

/// List all entities
pub async fn list_entities(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let entities = entity_service::list_entities(&state.db_pool).await?;

    Ok(Json(entities))
}

/// Get entity by ID
pub async fn get_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let entity = entity_service::get_entity(&state.db_pool, id).await?;

    Ok(Json(entity))
}

/// Update entity
pub async fn update_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEntityRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    let entity = entity_service::update_entity(&state.db_pool, id, payload).await?;

    Ok(Json(entity))
}

/// Delete entity
pub async fn delete_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    entity_service::delete_entity(&state.db_pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

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
