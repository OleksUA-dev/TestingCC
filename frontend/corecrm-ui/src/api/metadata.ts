import { apiClient } from './client';
import { Entity, Attribute, EntityMetadata } from '../types';

const METADATA_BASE_URL = 'http://localhost:8001';

export const metadataApi = {
  // Entities
  getEntities: () =>
    apiClient.get<Entity[]>(`${METADATA_BASE_URL}/api/v1/entities`),

  getEntity: (id: string) =>
    apiClient.get<Entity>(`${METADATA_BASE_URL}/api/v1/entities/${id}`),

  // Attributes
  getAttributes: (entityId: string) =>
    apiClient.get<Attribute[]>(`${METADATA_BASE_URL}/api/v1/entities/${entityId}/attributes`),

  // Complete metadata (for future use)
  getEntityMetadata: (entityId: string) =>
    apiClient.get<EntityMetadata>(`${METADATA_BASE_URL}/api/v1/entities/${entityId}/metadata`),
};
