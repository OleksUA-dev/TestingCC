use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use shared::{
    models::{Entity, CreateEntityRequest, UpdateEntityRequest},
    CoreResult, CoreError,
};

use crate::repository::entity_repository;

/// Create a new entity
pub async fn create_entity(
    pool: &PgPool,
    request: CreateEntityRequest,
) -> CoreResult<Entity> {
    // Check if entity with same system_name already exists
    if entity_repository::exists_by_system_name(pool, &request.system_name).await? {
        return Err(CoreError::InvalidInput(
            format!("Entity with system_name '{}' already exists", request.system_name)
        ));
    }

    // Create entity
    let entity = Entity {
        id: Uuid::new_v4(),
        system_name: request.system_name,
        display_name: request.display_name,
        description: request.description,
        display_name_plural: request.display_name_plural,
        is_system: false,
        is_active: true,
        icon: request.icon,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: None, // TODO: Get from auth context
        updated_by: None,
    };

    entity_repository::create(pool, &entity).await?;

    Ok(entity)
}

/// List all entities
pub async fn list_entities(pool: &PgPool) -> CoreResult<Vec<Entity>> {
    entity_repository::list(pool).await
}

/// Get entity by ID
pub async fn get_entity(pool: &PgPool, id: Uuid) -> CoreResult<Entity> {
    entity_repository::get_by_id(pool, id)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Entity with id {} not found", id)))
}

/// Update entity
pub async fn update_entity(
    pool: &PgPool,
    id: Uuid,
    request: UpdateEntityRequest,
) -> CoreResult<Entity> {
    // Get existing entity
    let mut entity = get_entity(pool, id).await?;

    // Check if it's a system entity
    if entity.is_system {
        return Err(CoreError::Forbidden(
            "Cannot modify system entities".to_string()
        ));
    }

    // Update fields
    if let Some(display_name) = request.display_name {
        entity.display_name = display_name;
    }
    if let Some(description) = request.description {
        entity.description = Some(description);
    }
    if let Some(display_name_plural) = request.display_name_plural {
        entity.display_name_plural = display_name_plural;
    }
    if let Some(is_active) = request.is_active {
        entity.is_active = is_active;
    }
    if let Some(icon) = request.icon {
        entity.icon = Some(icon);
    }

    entity.updated_at = Utc::now();

    entity_repository::update(pool, &entity).await?;

    Ok(entity)
}

/// Delete entity
pub async fn delete_entity(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    // Get entity to check if it's a system entity
    let entity = get_entity(pool, id).await?;

    if entity.is_system {
        return Err(CoreError::Forbidden(
            "Cannot delete system entities".to_string()
        ));
    }

    entity_repository::delete(pool, id).await?;

    Ok(())
}
