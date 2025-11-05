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
    models::{CreateAttributeRequest, UpdateAttributeRequest},
    CoreError,
};

use crate::{
    routes::AppState,
    service::attribute_service,
};

use super::entity::AppError;

/// Create a new attribute
pub async fn create_attribute(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAttributeRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    // Create attribute
    let attribute = attribute_service::create_attribute(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(attribute)))
}

/// List attributes for an entity
pub async fn list_attributes(
    State(state): State<Arc<AppState>>,
    Path(entity_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let attributes = attribute_service::list_attributes(&state.db_pool, entity_id).await?;

    Ok(Json(attributes))
}

/// Get attribute by ID
pub async fn get_attribute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let attribute = attribute_service::get_attribute(&state.db_pool, id).await?;

    Ok(Json(attribute))
}

/// Update attribute
pub async fn update_attribute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAttributeRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    let attribute = attribute_service::update_attribute(&state.db_pool, id, payload).await?;

    Ok(Json(attribute))
}

/// Delete attribute
pub async fn delete_attribute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    attribute_service::delete_attribute(&state.db_pool, id).await?;

    Ok(StatusCode::NO_CONTENT)
}
