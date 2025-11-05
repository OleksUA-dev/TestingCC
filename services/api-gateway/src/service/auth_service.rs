use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use shared::{
    models::{User, UserInfo, RegisterRequest, LoginRequest, LoginResponse, Claims},
    CoreResult, CoreError,
};

use crate::repository::user_repository;

/// Register a new user
pub async fn register(
    pool: &PgPool,
    request: RegisterRequest,
) -> CoreResult<UserInfo> {
    // Check if username already exists
    if user_repository::exists_by_username(pool, &request.username).await? {
        return Err(CoreError::InvalidInput(
            format!("Username '{}' already exists", request.username)
        ));
    }

    // Check if email already exists
    if user_repository::exists_by_email(pool, &request.email).await? {
        return Err(CoreError::InvalidInput(
            format!("Email '{}' already exists", request.email)
        ));
    }

    // Hash password
    let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
        .map_err(|e| CoreError::Internal(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = User {
        id: Uuid::new_v4(),
        username: request.username,
        email: request.email,
        password_hash,
        is_active: true,
        is_admin: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    user_repository::create(pool, &user).await?;

    Ok(user.into())
}

/// Authenticate user and return JWT token
pub async fn login(
    pool: &PgPool,
    request: LoginRequest,
    jwt_secret: &str,
    expiration_hours: i64,
) -> CoreResult<LoginResponse> {
    // Get user by username
    let user = user_repository::get_by_username(pool, &request.username)
        .await?
        .ok_or_else(|| CoreError::Unauthorized("Invalid username or password".to_string()))?;

    // Check if user is active
    if !user.is_active {
        return Err(CoreError::Unauthorized("Account is disabled".to_string()));
    }

    // Verify password
    let valid = bcrypt::verify(&request.password, &user.password_hash)
        .map_err(|e| CoreError::Internal(format!("Failed to verify password: {}", e)))?;

    if !valid {
        return Err(CoreError::Unauthorized("Invalid username or password".to_string()));
    }

    // Generate JWT token
    let now = Utc::now();
    let expiration = now + Duration::hours(expiration_hours);

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        is_admin: user.is_admin,
        iat: now.timestamp(),
        exp: expiration.timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| CoreError::Internal(format!("Failed to generate token: {}", e)))?;

    Ok(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: expiration_hours * 3600,
        user: user.into(),
    })
}

/// Get current user info
pub async fn get_current_user(pool: &PgPool, user_id: Uuid) -> CoreResult<UserInfo> {
    let user = user_repository::get_by_id(pool, user_id)
        .await?
        .ok_or_else(|| CoreError::NotFound("User not found".to_string()))?;

    Ok(user.into())
}
