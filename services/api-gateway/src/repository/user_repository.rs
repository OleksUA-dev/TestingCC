use sqlx::PgPool;
use uuid::Uuid;

use shared::{models::User, CoreResult};

/// Create a new user
pub async fn create(pool: &PgPool, user: &User) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password_hash, is_active, is_admin, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        user.id,
        user.username,
        user.email,
        user.password_hash,
        user.is_active,
        user.is_admin,
        user.created_at,
        user.updated_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get user by ID
pub async fn get_by_id(pool: &PgPool, id: Uuid) -> CoreResult<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Get user by username
pub async fn get_by_username(pool: &PgPool, username: &str) -> CoreResult<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Get user by email
pub async fn get_by_email(pool: &PgPool, email: &str) -> CoreResult<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Check if username exists
pub async fn exists_by_username(pool: &PgPool, username: &str) -> CoreResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM users WHERE username = $1) as "exists!"
        "#,
        username
    )
    .fetch_one(pool)
    .await?
    .exists;

    Ok(exists)
}

/// Check if email exists
pub async fn exists_by_email(pool: &PgPool, email: &str) -> CoreResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!"
        "#,
        email
    )
    .fetch_one(pool)
    .await?
    .exists;

    Ok(exists)
}

/// List all users
pub async fn list(pool: &PgPool) -> CoreResult<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}
