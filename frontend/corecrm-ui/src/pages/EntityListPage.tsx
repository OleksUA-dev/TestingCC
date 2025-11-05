import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { metadataApi } from '../api/metadata';
import { Entity } from '../types';

export const EntityListPage: React.FC = () => {
  const [entities, setEntities] = useState<Entity[]>([]);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    loadEntities();
  }, []);

  const loadEntities = async () => {
    try {
      const data = await metadataApi.getEntities();
      setEntities(data.filter((e) => e.is_active));
    } catch (err) {
      console.error('Failed to load entities:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-gray-500">Loading...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Entities</h1>
        <p className="mt-1 text-sm text-gray-600">
          Select an entity to view and manage its records
        </p>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {entities.map((entity) => (
          <div
            key={entity.id}
            onClick={() => navigate(`/entities/${entity.system_name}`)}
            className="relative rounded-lg border border-gray-300 bg-white px-6 py-5 shadow-sm hover:border-gray-400 cursor-pointer transition-colors"
          >
            <div className="flex items-center">
              {entity.icon && (
                <span className="mr-4 text-2xl">{entity.icon}</span>
              )}
              <div className="flex-1 min-w-0">
                <h3 className="text-sm font-medium text-gray-900 truncate">
                  {entity.display_name}
                </h3>
                <p className="text-sm text-gray-500 truncate">
                  {entity.display_name_plural}
                </p>
                {entity.description && (
                  <p className="mt-1 text-xs text-gray-400 line-clamp-2">
                    {entity.description}
                  </p>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>

      {entities.length === 0 && (
        <div className="text-center py-12">
          <p className="text-gray-500">No entities found</p>
          <p className="text-sm text-gray-400 mt-2">
            Create entities using the Metadata Service API
          </p>
        </div>
      )}
    </div>
  );
};
