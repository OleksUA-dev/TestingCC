use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use shared::{
    models::DataType,
    CoreResult, CoreError,
};

use crate::repository::{entity_repository, attribute_repository};

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaGenerationResult {
    pub table_name: String,
    pub sql: String,
    pub success: bool,
    pub message: String,
}

/// Generate and apply database schema for entity
pub async fn generate_and_apply_schema(
    pool: &PgPool,
    entity_id: Uuid,
) -> CoreResult<SchemaGenerationResult> {
    // Get entity
    let entity = entity_repository::get_by_id(pool, entity_id)
        .await?
        .ok_or_else(|| CoreError::NotFound(
            format!("Entity with id {} not found", entity_id)
        ))?;

    // Get attributes
    let attributes = attribute_repository::list_by_entity(pool, entity_id).await?;

    if attributes.is_empty() {
        return Err(CoreError::InvalidInput(
            "Cannot generate schema for entity without attributes".to_string()
        ));
    }

    // Generate table name (prefix with "entity_" to avoid conflicts)
    let table_name = format!("entity_{}", entity.system_name);

    // Generate CREATE TABLE SQL
    let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (\n", table_name);
    sql.push_str("    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),\n");

    // Add columns for each attribute
    for (idx, attr) in attributes.iter().enumerate() {
        let column_name = &attr.system_name;
        let data_type = attr.data_type.to_pg_type();

        sql.push_str(&format!("    {} {}", column_name, data_type));

        // Add constraints
        if attr.is_required {
            sql.push_str(" NOT NULL");
        }
        if attr.is_unique {
            sql.push_str(" UNIQUE");
        }

        // Add default value if present
        if let Some(default_val) = &attr.default_value {
            if let Some(default_str) = default_val.as_str() {
                sql.push_str(&format!(" DEFAULT '{}'", default_str));
            } else if let Some(default_num) = default_val.as_i64() {
                sql.push_str(&format!(" DEFAULT {}", default_num));
            } else if let Some(default_bool) = default_val.as_bool() {
                sql.push_str(&format!(" DEFAULT {}", default_bool));
            }
        }

        // Add foreign key constraint for Lookup type
        if attr.data_type == DataType::Lookup {
            if let Some(lookup_entity_id) = attr.lookup_entity_id {
                if let Ok(Some(lookup_entity)) = entity_repository::get_by_id(pool, lookup_entity_id).await {
                    let lookup_table = format!("entity_{}", lookup_entity.system_name);
                    sql.push_str(&format!(" REFERENCES {}(id)", lookup_table));
                }
            }
        }

        if idx < attributes.len() - 1 {
            sql.push_str(",\n");
        } else {
            sql.push_str(",\n");
        }
    }

    // Add audit fields
    sql.push_str("    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),\n");
    sql.push_str("    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),\n");
    sql.push_str("    created_by UUID,\n");
    sql.push_str("    updated_by UUID\n");
    sql.push_str(");");

    // Create indexes for lookup fields and unique fields
    let mut index_sql = String::new();
    for attr in &attributes {
        if attr.data_type == DataType::Lookup || attr.is_unique {
            index_sql.push_str(&format!(
                "\nCREATE INDEX IF NOT EXISTS idx_{}_{} ON {} ({});",
                table_name, attr.system_name, table_name, attr.system_name
            ));
        }
    }

    let full_sql = format!("{}\n{}", sql, index_sql);

    // Execute SQL
    match sqlx::raw_sql(&full_sql).execute(pool).await {
        Ok(_) => Ok(SchemaGenerationResult {
            table_name: table_name.clone(),
            sql: full_sql,
            success: true,
            message: format!("Table '{}' created successfully", table_name),
        }),
        Err(e) => Ok(SchemaGenerationResult {
            table_name: table_name.clone(),
            sql: full_sql,
            success: false,
            message: format!("Failed to create table: {}", e),
        }),
    }
}
