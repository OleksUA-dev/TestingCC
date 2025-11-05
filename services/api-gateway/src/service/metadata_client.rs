use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

use shared::{models::{Entity, Attribute}, CoreResult, CoreError};

/// Client for communicating with metadata-service
pub struct MetadataClient {
    base_url: String,
    client: Client,
}

impl MetadataClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    /// Get entity by ID
    pub async fn get_entity(&self, entity_id: Uuid) -> CoreResult<Entity> {
        let url = format!("{}/api/v1/entities/{}", self.base_url, entity_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to fetch entity: {}", e)))?;

        if !response.status().is_success() {
            return Err(CoreError::NotFound(format!("Entity {} not found", entity_id)));
        }

        let entity = response
            .json::<Entity>()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to parse entity: {}", e)))?;

        Ok(entity)
    }

    /// Get entity by system_name
    pub async fn get_entity_by_name(&self, system_name: &str) -> CoreResult<Entity> {
        let url = format!("{}/api/v1/entities", self.base_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to fetch entities: {}", e)))?;

        let entities = response
            .json::<Vec<Entity>>()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to parse entities: {}", e)))?;

        entities
            .into_iter()
            .find(|e| e.system_name == system_name)
            .ok_or_else(|| CoreError::NotFound(format!("Entity '{}' not found", system_name)))
    }

    /// List attributes for entity
    pub async fn get_attributes(&self, entity_id: Uuid) -> CoreResult<Vec<Attribute>> {
        let url = format!("{}/api/v1/entities/{}/attributes", self.base_url, entity_id);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to fetch attributes: {}", e)))?;

        if !response.status().is_success() {
            return Err(CoreError::NotFound(format!(
                "Attributes for entity {} not found",
                entity_id
            )));
        }

        let attributes = response
            .json::<Vec<Attribute>>()
            .await
            .map_err(|e| CoreError::Internal(format!("Failed to parse attributes: {}", e)))?;

        Ok(attributes)
    }
}
