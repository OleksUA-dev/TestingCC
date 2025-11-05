use sqlx::PgPool;
use uuid::Uuid;

use shared::{
    models::Entity,
    CoreResult,
};

/// Create a new entity
pub async fn create(pool: &PgPool, entity: &Entity) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO entities (
            id, system_name, display_name, description, display_name_plural,
            is_system, is_active, icon, created_at, updated_at, created_by, updated_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        entity.id,
        entity.system_name,
        entity.display_name,
        entity.description,
        entity.display_name_plural,
        entity.is_system,
        entity.is_active,
        entity.icon,
        entity.created_at,
        entity.updated_at,
        entity.created_by,
        entity.updated_by,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get entity by ID
pub async fn get_by_id(pool: &PgPool, id: Uuid) -> CoreResult<Option<Entity>> {
    let entity = sqlx::query_as!(
        Entity,
        r#"
        SELECT * FROM entities WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(entity)
}

/// List all entities
pub async fn list(pool: &PgPool) -> CoreResult<Vec<Entity>> {
    let entities = sqlx::query_as!(
        Entity,
        r#"
        SELECT * FROM entities ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(entities)
}

/// Check if entity exists by system_name
pub async fn exists_by_system_name(pool: &PgPool, system_name: &str) -> CoreResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT EXISTS(SELECT 1 FROM entities WHERE system_name = $1) as "exists!"
        "#,
        system_name
    )
    .fetch_one(pool)
    .await?
    .exists;

    Ok(exists)
}

/// Update entity
pub async fn update(pool: &PgPool, entity: &Entity) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE entities
        SET display_name = $2,
            description = $3,
            display_name_plural = $4,
            is_active = $5,
            icon = $6,
            updated_at = $7,
            updated_by = $8
        WHERE id = $1
        "#,
        entity.id,
        entity.display_name,
        entity.description,
        entity.display_name_plural,
        entity.is_active,
        entity.icon,
        entity.updated_at,
        entity.updated_by,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete entity
pub async fn delete(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM entities WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
