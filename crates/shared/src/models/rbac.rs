use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Role defines a set of permissions with support for inheritance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_system: bool,
    /// Parent role for inheritance (optional)
    pub parent_role_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Permission types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PermissionType {
    Create,
    Read,
    Update,
    Delete,
}

/// Entity permission - grants access to an entity for a role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityPermission {
    pub id: Uuid,
    pub role_id: Uuid,
    pub entity_id: Uuid,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub created_at: DateTime<Utc>,
}

/// User-Role assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub assigned_at: DateTime<Utc>,
}

/// Request to create a role
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    /// Parent role for inheritance
    pub parent_role_id: Option<Uuid>,
}

/// Request to set entity permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetEntityPermissionRequest {
    pub role_id: Uuid,
    pub entity_id: Uuid,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
}

/// Request to assign role to user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

/// Permission check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheck {
    pub has_permission: bool,
    pub permission_type: PermissionType,
    pub entity_id: Uuid,
}
