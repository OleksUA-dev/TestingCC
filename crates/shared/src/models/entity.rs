use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Entity represents a business object (e.g., Contact, Account, Invoice)
/// This is the core of the metadata-driven architecture
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Entity {
    /// Unique identifier
    pub id: Uuid,

    /// System name (e.g., "contact", "invoice") - used in API and database
    #[validate(length(min = 1, max = 100))]
    #[validate(regex(path = "RE_SYSTEM_NAME"))]
    pub system_name: String,

    /// Display name for UI (e.g., "Контакт", "Рахунок")
    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    /// Optional description
    pub description: Option<String>,

    /// Plural display name (e.g., "Контакти", "Рахунки")
    #[validate(length(min = 1, max = 255))]
    pub display_name_plural: String,

    /// Is this a system entity (immutable core objects)?
    pub is_system: bool,

    /// Is this entity active?
    pub is_active: bool,

    /// Icon name for UI
    pub icon: Option<String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,

    /// Created by user ID
    pub created_by: Option<Uuid>,

    /// Updated by user ID
    pub updated_by: Option<Uuid>,
}

/// Request to create a new entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateEntityRequest {
    #[validate(length(min = 1, max = 100))]
    #[validate(regex(path = "RE_SYSTEM_NAME"))]
    pub system_name: String,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    pub description: Option<String>,

    #[validate(length(min = 1, max = 255))]
    pub display_name_plural: String,

    pub icon: Option<String>,
}

/// Request to update an existing entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateEntityRequest {
    #[validate(length(min = 1, max = 255))]
    pub display_name: Option<String>,

    pub description: Option<String>,

    #[validate(length(min = 1, max = 255))]
    pub display_name_plural: Option<String>,

    pub is_active: Option<bool>,

    pub icon: Option<String>,
}

// Regex for validating system names (lowercase, alphanumeric, underscores only)
lazy_static::lazy_static! {
    static ref RE_SYSTEM_NAME: regex::Regex = regex::Regex::new(r"^[a-z][a-z0-9_]*$").unwrap();
}
