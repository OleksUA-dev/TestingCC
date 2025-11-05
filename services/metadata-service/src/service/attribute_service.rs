use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use shared::{
    models::{Attribute, CreateAttributeRequest, UpdateAttributeRequest, DataType},
    CoreResult, CoreError,
};

use crate::repository::{attribute_repository, entity_repository};

/// Create a new attribute
pub async fn create_attribute(
    pool: &PgPool,
    request: CreateAttributeRequest,
) -> CoreResult<Attribute> {
    // Check if entity exists
    let entity = entity_repository::get_by_id(pool, request.entity_id)
        .await?
        .ok_or_else(|| CoreError::NotFound(
            format!("Entity with id {} not found", request.entity_id)
        ))?;

    // Check if attribute with same system_name already exists for this entity
    if attribute_repository::exists_by_system_name(
        pool,
        request.entity_id,
        &request.system_name
    ).await? {
        return Err(CoreError::InvalidInput(
            format!("Attribute with system_name '{}' already exists for entity '{}'",
                request.system_name, entity.system_name)
        ));
    }

    // Validate data type specific requirements
    if request.data_type == DataType::Lookup && request.lookup_entity_id.is_none() {
        return Err(CoreError::InvalidInput(
            "lookup_entity_id is required for Lookup data type".to_string()
        ));
    }

    if request.data_type == DataType::Enum && request.enum_values.is_none() {
        return Err(CoreError::InvalidInput(
            "enum_values is required for Enum data type".to_string()
        ));
    }

    // Get max display_order for this entity
    let max_order = attribute_repository::get_max_display_order(pool, request.entity_id).await?;

    // Create attribute
    let attribute = Attribute {
        id: Uuid::new_v4(),
        entity_id: request.entity_id,
        system_name: request.system_name,
        display_name: request.display_name,
        description: request.description,
        data_type: request.data_type,
        is_required: request.is_required.unwrap_or(false),
        is_unique: request.is_unique.unwrap_or(false),
        is_system: false,
        is_active: true,
        default_value: request.default_value,
        lookup_entity_id: request.lookup_entity_id,
        enum_values: request.enum_values,
        validation_rules: request.validation_rules,
        display_order: request.display_order.unwrap_or(max_order + 1),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: None, // TODO: Get from auth context
        updated_by: None,
    };

    attribute_repository::create(pool, &attribute).await?;

    Ok(attribute)
}

/// List all attributes for an entity
pub async fn list_attributes(pool: &PgPool, entity_id: Uuid) -> CoreResult<Vec<Attribute>> {
    attribute_repository::list_by_entity(pool, entity_id).await
}

/// Get attribute by ID
pub async fn get_attribute(pool: &PgPool, id: Uuid) -> CoreResult<Attribute> {
    attribute_repository::get_by_id(pool, id)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Attribute with id {} not found", id)))
}

/// Update attribute
pub async fn update_attribute(
    pool: &PgPool,
    id: Uuid,
    request: UpdateAttributeRequest,
) -> CoreResult<Attribute> {
    // Get existing attribute
    let mut attribute = get_attribute(pool, id).await?;

    // Check if it's a system attribute
    if attribute.is_system {
        return Err(CoreError::Forbidden(
            "Cannot modify system attributes".to_string()
        ));
    }

    // Update fields
    if let Some(display_name) = request.display_name {
        attribute.display_name = display_name;
    }
    if let Some(description) = request.description {
        attribute.description = Some(description);
    }
    if let Some(is_required) = request.is_required {
        attribute.is_required = is_required;
    }
    if let Some(is_unique) = request.is_unique {
        attribute.is_unique = is_unique;
    }
    if let Some(is_active) = request.is_active {
        attribute.is_active = is_active;
    }
    if let Some(default_value) = request.default_value {
        attribute.default_value = Some(default_value);
    }
    if let Some(enum_values) = request.enum_values {
        attribute.enum_values = Some(enum_values);
    }
    if let Some(validation_rules) = request.validation_rules {
        attribute.validation_rules = Some(validation_rules);
    }
    if let Some(display_order) = request.display_order {
        attribute.display_order = display_order;
    }

    attribute.updated_at = Utc::now();

    attribute_repository::update(pool, &attribute).await?;

    Ok(attribute)
}

/// Delete attribute
pub async fn delete_attribute(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    // Get attribute to check if it's a system attribute
    let attribute = get_attribute(pool, id).await?;

    if attribute.is_system {
        return Err(CoreError::Forbidden(
            "Cannot delete system attributes".to_string()
        ));
    }

    attribute_repository::delete(pool, id).await?;

    Ok(())
}
