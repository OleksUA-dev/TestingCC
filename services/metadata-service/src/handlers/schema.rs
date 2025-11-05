use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    routes::AppState,
    service::schema_service,
};

use super::entity::AppError;

/// Generate database schema for entity
pub async fn generate_schema(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = schema_service::generate_and_apply_schema(&state.db_pool, id).await?;

    Ok((StatusCode::OK, Json(result)))
}
