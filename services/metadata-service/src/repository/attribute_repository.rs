use sqlx::PgPool;
use uuid::Uuid;

use shared::{
    models::Attribute,
    CoreResult,
};

/// Create a new attribute
pub async fn create(pool: &PgPool, attribute: &Attribute) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO attributes (
            id, entity_id, system_name, display_name, description, data_type,
            is_required, is_unique, is_system, is_active, default_value,
            lookup_entity_id, enum_values, validation_rules, display_order,
            created_at, updated_at, created_by, updated_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        "#,
        attribute.id,
        attribute.entity_id,
        attribute.system_name,
        attribute.display_name,
        attribute.description,
        attribute.data_type as _,
        attribute.is_required,
        attribute.is_unique,
        attribute.is_system,
        attribute.is_active,
        attribute.default_value,
        attribute.lookup_entity_id,
        attribute.enum_values,
        attribute.validation_rules,
        attribute.display_order,
        attribute.created_at,
        attribute.updated_at,
        attribute.created_by,
        attribute.updated_by,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get attribute by ID
pub async fn get_by_id(pool: &PgPool, id: Uuid) -> CoreResult<Option<Attribute>> {
    let attribute = sqlx::query_as!(
        Attribute,
        r#"
        SELECT
            id, entity_id, system_name, display_name, description,
            data_type as "data_type: _",
            is_required, is_unique, is_system, is_active, default_value,
            lookup_entity_id, enum_values, validation_rules, display_order,
            created_at, updated_at, created_by, updated_by
        FROM attributes WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(attribute)
}

/// List all attributes for an entity
pub async fn list_by_entity(pool: &PgPool, entity_id: Uuid) -> CoreResult<Vec<Attribute>> {
    let attributes = sqlx::query_as!(
        Attribute,
        r#"
        SELECT
            id, entity_id, system_name, display_name, description,
            data_type as "data_type: _",
            is_required, is_unique, is_system, is_active, default_value,
            lookup_entity_id, enum_values, validation_rules, display_order,
            created_at, updated_at, created_by, updated_by
        FROM attributes
        WHERE entity_id = $1
        ORDER BY display_order ASC
        "#,
        entity_id
    )
    .fetch_all(pool)
    .await?;

    Ok(attributes)
}

/// Check if attribute exists by system_name for an entity
pub async fn exists_by_system_name(
    pool: &PgPool,
    entity_id: Uuid,
    system_name: &str,
) -> CoreResult<bool> {
    let exists = sqlx::query!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM attributes
            WHERE entity_id = $1 AND system_name = $2
        ) as "exists!"
        "#,
        entity_id,
        system_name
    )
    .fetch_one(pool)
    .await?
    .exists;

    Ok(exists)
}

/// Get max display_order for an entity
pub async fn get_max_display_order(pool: &PgPool, entity_id: Uuid) -> CoreResult<i32> {
    let result = sqlx::query!(
        r#"
        SELECT COALESCE(MAX(display_order), 0) as "max_order!"
        FROM attributes
        WHERE entity_id = $1
        "#,
        entity_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.max_order)
}

/// Update attribute
pub async fn update(pool: &PgPool, attribute: &Attribute) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE attributes
        SET display_name = $2,
            description = $3,
            is_required = $4,
            is_unique = $5,
            is_active = $6,
            default_value = $7,
            enum_values = $8,
            validation_rules = $9,
            display_order = $10,
            updated_at = $11,
            updated_by = $12
        WHERE id = $1
        "#,
        attribute.id,
        attribute.display_name,
        attribute.description,
        attribute.is_required,
        attribute.is_unique,
        attribute.is_active,
        attribute.default_value,
        attribute.enum_values,
        attribute.validation_rules,
        attribute.display_order,
        attribute.updated_at,
        attribute.updated_by,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete attribute
pub async fn delete(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM attributes WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
