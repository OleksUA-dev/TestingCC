use sqlx::{PgPool, Row};
use uuid::Uuid;
use serde_json::Value;

use shared::{
    models::{Entity, Attribute, DataType},
    CoreResult, CoreError,
};

use super::metadata_client::MetadataClient;

/// Dynamic CRUD service for custom entities
pub struct DynamicCrudService {
    metadata_client: MetadataClient,
}

impl DynamicCrudService {
    pub fn new(metadata_service_url: String) -> Self {
        Self {
            metadata_client: MetadataClient::new(metadata_service_url),
        }
    }

    /// Create a new record in a dynamic entity table
    pub async fn create_record(
        &self,
        pool: &PgPool,
        entity_name: &str,
        data: Value,
        user_id: Option<Uuid>,
    ) -> CoreResult<Value> {
        // Get entity metadata
        let entity = self.metadata_client.get_entity_by_name(entity_name).await?;
        let attributes = self.metadata_client.get_attributes(entity.id).await?;

        // Validate required fields
        self.validate_required_fields(&attributes, &data)?;

        // Build INSERT query
        let table_name = format!("entity_{}", entity.system_name);
        let mut columns = vec!["id".to_string()];
        let mut placeholders = vec!["$1".to_string()];
        let mut param_index = 2;

        for attr in &attributes {
            if data.get(&attr.system_name).is_some() {
                columns.push(attr.system_name.clone());
                placeholders.push(format!("${}", param_index));
                param_index += 1;
            }
        }

        // Add audit fields
        columns.extend(["created_at".to_string(), "updated_at".to_string()]);
        placeholders.extend([format!("${}", param_index), format!("${}", param_index + 1)]);

        if user_id.is_some() {
            columns.extend(["created_by".to_string(), "updated_by".to_string()]);
            placeholders.extend([format!("${}", param_index + 2), format!("${}", param_index + 3)]);
        }

        let query_str = format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );

        // Build query with parameters
        let new_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let mut query = sqlx::query(&query_str).bind(new_id);

        for attr in &attributes {
            if let Some(value) = data.get(&attr.system_name) {
                query = self.bind_value(query, &attr.data_type, value)?;
            }
        }

        query = query.bind(now).bind(now);

        if let Some(uid) = user_id {
            query = query.bind(uid).bind(uid);
        }

        // Execute query
        let row = query
            .fetch_one(pool)
            .await
            .map_err(|e| CoreError::Database(e))?;

        // Convert row to JSON
        let result = self.row_to_json(&row, &attributes)?;

        Ok(result)
    }

    /// Get a record by ID
    pub async fn get_record(
        &self,
        pool: &PgPool,
        entity_name: &str,
        record_id: Uuid,
    ) -> CoreResult<Value> {
        let entity = self.metadata_client.get_entity_by_name(entity_name).await?;
        let attributes = self.metadata_client.get_attributes(entity.id).await?;

        let table_name = format!("entity_{}", entity.system_name);
        let query_str = format!("SELECT * FROM {} WHERE id = $1", table_name);

        let row = sqlx::query(&query_str)
            .bind(record_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| CoreError::Database(e))?
            .ok_or_else(|| CoreError::NotFound(format!("Record {} not found", record_id)))?;

        let result = self.row_to_json(&row, &attributes)?;

        Ok(result)
    }

    /// List records with optional filtering
    pub async fn list_records(
        &self,
        pool: &PgPool,
        entity_name: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> CoreResult<Vec<Value>> {
        let entity = self.metadata_client.get_entity_by_name(entity_name).await?;
        let attributes = self.metadata_client.get_attributes(entity.id).await?;

        let table_name = format!("entity_{}", entity.system_name);
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let query_str = format!(
            "SELECT * FROM {} ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            table_name
        );

        let rows = sqlx::query(&query_str)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(|e| CoreError::Database(e))?;

        let results = rows
            .iter()
            .map(|row| self.row_to_json(row, &attributes))
            .collect::<CoreResult<Vec<_>>>()?;

        Ok(results)
    }

    /// Update a record
    pub async fn update_record(
        &self,
        pool: &PgPool,
        entity_name: &str,
        record_id: Uuid,
        data: Value,
        user_id: Option<Uuid>,
    ) -> CoreResult<Value> {
        let entity = self.metadata_client.get_entity_by_name(entity_name).await?;
        let attributes = self.metadata_client.get_attributes(entity.id).await?;

        // Build UPDATE query
        let table_name = format!("entity_{}", entity.system_name);
        let mut set_clauses = vec![];
        let mut param_index = 1;

        for attr in &attributes {
            if data.get(&attr.system_name).is_some() {
                set_clauses.push(format!("{} = ${}", attr.system_name, param_index));
                param_index += 1;
            }
        }

        set_clauses.push(format!("updated_at = ${}", param_index));
        param_index += 1;

        if user_id.is_some() {
            set_clauses.push(format!("updated_by = ${}", param_index));
            param_index += 1;
        }

        let query_str = format!(
            "UPDATE {} SET {} WHERE id = ${} RETURNING *",
            table_name,
            set_clauses.join(", "),
            param_index
        );

        let mut query = sqlx::query(&query_str);

        for attr in &attributes {
            if let Some(value) = data.get(&attr.system_name) {
                query = self.bind_value(query, &attr.data_type, value)?;
            }
        }

        let now = chrono::Utc::now();
        query = query.bind(now);

        if let Some(uid) = user_id {
            query = query.bind(uid);
        }

        query = query.bind(record_id);

        let row = query
            .fetch_optional(pool)
            .await
            .map_err(|e| CoreError::Database(e))?
            .ok_or_else(|| CoreError::NotFound(format!("Record {} not found", record_id)))?;

        let result = self.row_to_json(&row, &attributes)?;

        Ok(result)
    }

    /// Delete a record
    pub async fn delete_record(
        &self,
        pool: &PgPool,
        entity_name: &str,
        record_id: Uuid,
    ) -> CoreResult<()> {
        let entity = self.metadata_client.get_entity_by_name(entity_name).await?;

        let table_name = format!("entity_{}", entity.system_name);
        let query_str = format!("DELETE FROM {} WHERE id = $1", table_name);

        let result = sqlx::query(&query_str)
            .bind(record_id)
            .execute(pool)
            .await
            .map_err(|e| CoreError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(CoreError::NotFound(format!("Record {} not found", record_id)));
        }

        Ok(())
    }

    /// Validate required fields
    fn validate_required_fields(&self, attributes: &[Attribute], data: &Value) -> CoreResult<()> {
        for attr in attributes {
            if attr.is_required && !data.get(&attr.system_name).is_some() {
                return Err(CoreError::Validation(format!(
                    "Required field '{}' is missing",
                    attr.system_name
                )));
            }
        }
        Ok(())
    }

    /// Bind value to query based on data type
    fn bind_value<'q>(
        &self,
        query: sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>,
        data_type: &DataType,
        value: &Value,
    ) -> CoreResult<sqlx::query::Query<'q, sqlx::Postgres, sqlx::postgres::PgArguments>> {
        let query = match data_type {
            DataType::String | DataType::Text | DataType::Enum => {
                query.bind(value.as_str().ok_or_else(|| {
                    CoreError::InvalidInput("Expected string value".to_string())
                })?)
            }
            DataType::Integer => {
                query.bind(value.as_i64().ok_or_else(|| {
                    CoreError::InvalidInput("Expected integer value".to_string())
                })?)
            }
            DataType::Float => {
                query.bind(value.as_f64().ok_or_else(|| {
                    CoreError::InvalidInput("Expected float value".to_string())
                })?)
            }
            DataType::Decimal => {
                // For decimal, we accept string or number
                if let Some(s) = value.as_str() {
                    query.bind(s)
                } else if let Some(n) = value.as_f64() {
                    query.bind(n.to_string())
                } else {
                    return Err(CoreError::InvalidInput("Expected decimal value".to_string()));
                }
            }
            DataType::Boolean => {
                query.bind(value.as_bool().ok_or_else(|| {
                    CoreError::InvalidInput("Expected boolean value".to_string())
                })?)
            }
            DataType::Date | DataType::DateTime => {
                query.bind(value.as_str().ok_or_else(|| {
                    CoreError::InvalidInput("Expected date/datetime string".to_string())
                })?)
            }
            DataType::Lookup => {
                let uuid_str = value.as_str().ok_or_else(|| {
                    CoreError::InvalidInput("Expected UUID string for lookup".to_string())
                })?;
                let uuid = Uuid::parse_str(uuid_str).map_err(|_| {
                    CoreError::InvalidInput("Invalid UUID format".to_string())
                })?;
                query.bind(uuid)
            }
        };
        Ok(query)
    }

    /// Convert database row to JSON
    fn row_to_json(&self, row: &sqlx::postgres::PgRow, attributes: &[Attribute]) -> CoreResult<Value> {
        let mut map = serde_json::Map::new();

        // Add ID
        if let Ok(id) = row.try_get::<Uuid, _>("id") {
            map.insert("id".to_string(), Value::String(id.to_string()));
        }

        // Add attribute values
        for attr in attributes {
            let value = match attr.data_type {
                DataType::String | DataType::Text | DataType::Enum => {
                    row.try_get::<Option<String>, _>(attr.system_name.as_str())
                        .ok()
                        .flatten()
                        .map(Value::String)
                }
                DataType::Integer => {
                    row.try_get::<Option<i64>, _>(attr.system_name.as_str())
                        .ok()
                        .flatten()
                        .map(|v| Value::Number(v.into()))
                }
                DataType::Float => {
                    row.try_get::<Option<f64>, _>(attr.system_name.as_str())
                        .ok()
                        .flatten()
                        .and_then(|v| serde_json::Number::from_f64(v))
                        .map(Value::Number)
                }
                DataType::Boolean => {
                    row.try_get::<Option<bool>, _>(attr.system_name.as_str())
                        .ok()
                        .flatten()
                        .map(Value::Bool)
                }
                DataType::Lookup => {
                    row.try_get::<Option<Uuid>, _>(attr.system_name.as_str())
                        .ok()
                        .flatten()
                        .map(|v| Value::String(v.to_string()))
                }
                _ => None,
            };

            if let Some(v) = value {
                map.insert(attr.system_name.clone(), v);
            } else {
                map.insert(attr.system_name.clone(), Value::Null);
            }
        }

        // Add audit fields
        if let Ok(created_at) = row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at") {
            map.insert("created_at".to_string(), Value::String(created_at.to_rfc3339()));
        }
        if let Ok(updated_at) = row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at") {
            map.insert("updated_at".to_string(), Value::String(updated_at.to_rfc3339()));
        }

        Ok(Value::Object(map))
    }
}
