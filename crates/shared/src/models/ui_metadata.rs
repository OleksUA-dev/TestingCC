use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Form layout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormLayout {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub name: String,
    pub is_default: bool,
    pub sections: Vec<FormSection>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Form section (grouping of fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormSection {
    pub title: String,
    pub display_order: i32,
    pub columns: i32,
    pub fields: Vec<FormField>,
}

/// Form field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub attribute_id: Uuid,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub display_order: i32,
    pub is_readonly: bool,
    pub is_visible: bool,
    pub column_span: i32,
}

/// Grid/List view configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridLayout {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub name: String,
    pub is_default: bool,
    pub columns: Vec<GridColumn>,
    pub default_sort_by: Option<String>,
    pub default_sort_order: Option<SortOrder>,
    pub page_size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Grid column configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridColumn {
    pub attribute_id: Uuid,
    pub header: Option<String>,
    pub width: Option<i32>,
    pub display_order: i32,
    pub is_sortable: bool,
    pub is_filterable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Request to create form layout
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFormLayoutRequest {
    pub entity_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub is_default: Option<bool>,
    pub sections: Vec<FormSection>,
}

/// Request to create grid layout
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateGridLayoutRequest {
    pub entity_id: Uuid,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub is_default: Option<bool>,
    pub columns: Vec<GridColumn>,
    pub default_sort_by: Option<String>,
    pub default_sort_order: Option<SortOrder>,

    #[validate(range(min = 10, max = 1000))]
    pub page_size: Option<i32>,
}

/// Complete entity metadata including UI layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMetadata {
    pub entity: crate::models::Entity,
    pub attributes: Vec<crate::models::Attribute>,
    pub form_layouts: Vec<FormLayout>,
    pub grid_layouts: Vec<GridLayout>,
}
