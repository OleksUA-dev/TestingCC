use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::DataType;

/// Attribute represents a field/column in an entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Attribute {
    /// Unique identifier
    pub id: Uuid,

    /// Reference to parent entity
    pub entity_id: Uuid,

    /// System name (e.g., "first_name", "email")
    #[validate(length(min = 1, max = 100))]
    #[validate(regex(path = "RE_SYSTEM_NAME"))]
    pub system_name: String,

    /// Display name for UI (e.g., "Ім'я", "Email")
    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    /// Optional description
    pub description: Option<String>,

    /// Data type of this attribute
    pub data_type: DataType,

    /// Is this attribute required?
    pub is_required: bool,

    /// Is this attribute unique?
    pub is_unique: bool,

    /// Is this attribute a system attribute (immutable)?
    pub is_system: bool,

    /// Is this attribute active?
    pub is_active: bool,

    /// Default value (stored as JSON)
    pub default_value: Option<serde_json::Value>,

    /// For Lookup type: referenced entity ID
    pub lookup_entity_id: Option<Uuid>,

    /// For Enum type: possible values (stored as JSON array)
    pub enum_values: Option<serde_json::Value>,

    /// Validation rules (stored as JSON)
    pub validation_rules: Option<serde_json::Value>,

    /// Display order in forms
    pub display_order: i32,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,

    /// Created by user ID
    pub created_by: Option<Uuid>,

    /// Updated by user ID
    pub updated_by: Option<Uuid>,
}

/// Request to create a new attribute
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAttributeRequest {
    pub entity_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    #[validate(regex(path = "RE_SYSTEM_NAME"))]
    pub system_name: String,

    #[validate(length(min = 1, max = 255))]
    pub display_name: String,

    pub description: Option<String>,

    pub data_type: DataType,

    pub is_required: Option<bool>,

    pub is_unique: Option<bool>,

    pub default_value: Option<serde_json::Value>,

    /// Required for Lookup data type
    pub lookup_entity_id: Option<Uuid>,

    /// Required for Enum data type
    pub enum_values: Option<serde_json::Value>,

    pub validation_rules: Option<serde_json::Value>,

    pub display_order: Option<i32>,
}

/// Request to update an existing attribute
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateAttributeRequest {
    #[validate(length(min = 1, max = 255))]
    pub display_name: Option<String>,

    pub description: Option<String>,

    pub is_required: Option<bool>,

    pub is_unique: Option<bool>,

    pub is_active: Option<bool>,

    pub default_value: Option<serde_json::Value>,

    pub enum_values: Option<serde_json::Value>,

    pub validation_rules: Option<serde_json::Value>,

    pub display_order: Option<i32>,
}

/// Enum value definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub key: String,
    pub value: String,
    pub display_order: i32,
}

// Regex for validating system names
lazy_static::lazy_static! {
    static ref RE_SYSTEM_NAME: regex::Regex = regex::Regex::new(r"^[a-z][a-z0-9_]*$").unwrap();
}
