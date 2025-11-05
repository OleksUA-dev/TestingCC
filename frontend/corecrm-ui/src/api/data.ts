import { apiClient } from './client';
import { Record } from '../types';

export const dataApi = {
  // Create record
  create: (entityName: string, data: any) =>
    apiClient.post<Record>(`/api/v1/data/${entityName}`, data),

  // List records
  list: (entityName: string, params?: { limit?: number; offset?: number }) =>
    apiClient.get<Record[]>(`/api/v1/data/${entityName}`, params),

  // Get single record
  get: (entityName: string, id: string) =>
    apiClient.get<Record>(`/api/v1/data/${entityName}/${id}`),

  // Update record
  update: (entityName: string, id: string, data: any) =>
    apiClient.put<Record>(`/api/v1/data/${entityName}/${id}`, data),

  // Delete record
  delete: (entityName: string, id: string) =>
    apiClient.delete(`/api/v1/data/${entityName}/${id}`),
};
