use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use shared::{models::Claims, CoreError};

use crate::routes::AppState;

/// Extract JWT token from Authorization header
pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                None
            }
        })
}

/// Verify JWT token and extract claims
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, CoreError> {
    let validation = Validation::default();
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| CoreError::Unauthorized(format!("Invalid token: {}", e)))
}

/// Middleware to require authentication
pub async fn require_auth(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let token = extract_token(&headers).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "Missing or invalid Authorization header",
        )
    })?;

    let claims = verify_token(&token, &state.config.jwt.secret).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Authentication failed: {}", e),
        )
    })?;

    // Store claims in request extensions for handlers to use
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Middleware to require admin role
pub async fn require_admin(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let token = extract_token(&headers).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "Missing or invalid Authorization header",
        )
    })?;

    let claims = verify_token(&token, &state.config.jwt.secret).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Authentication failed: {}", e),
        )
    })?;

    if !claims.is_admin {
        return Err((StatusCode::FORBIDDEN, "Admin access required"));
    }

    // Store claims in request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
