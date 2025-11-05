use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use validator::Validate;

use shared::{
    models::{RegisterRequest, LoginRequest, LoginResponse, UserInfo, Claims},
    CoreError,
};

use crate::{routes::AppState, service::auth_service};

/// Register a new user
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    // Register user
    let user = auth_service::register(&state.db_pool, payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// Login and get JWT token
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    payload.validate()?;

    // Authenticate user
    let response = auth_service::login(
        &state.db_pool,
        payload,
        &state.config.jwt.secret,
        state.config.jwt.expiration_hours,
    )
    .await?;

    Ok(Json(response))
}

/// Get current user information
pub async fn me(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    // Extract claims from request extensions (set by auth middleware)
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| CoreError::Unauthorized("Not authenticated".to_string()))?;

    let user_id = claims
        .user_id()
        .ok_or_else(|| CoreError::Internal("Invalid user ID in token".to_string()))?;

    // Get user info
    let user = auth_service::get_current_user(&state.db_pool, user_id).await?;

    Ok(Json(user))
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
