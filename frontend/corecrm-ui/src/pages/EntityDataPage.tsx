import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { metadataApi } from '../api/metadata';
import { dataApi } from '../api/data';
import { DynamicGrid } from '../components/DynamicGrid';
import { DynamicForm } from '../components/DynamicForm';
import { Attribute, Record } from '../types';

export const EntityDataPage: React.FC = () => {
  const { entityName } = useParams<{ entityName: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [showForm, setShowForm] = useState(false);
  const [editingRecord, setEditingRecord] = useState<Record | null>(null);

  // Load entity metadata
  const { data: entities } = useQuery({
    queryKey: ['entities'],
    queryFn: metadataApi.getEntities,
  });

  const entity = entities?.find((e) => e.system_name === entityName);

  // Load attributes
  const { data: attributes = [] } = useQuery({
    queryKey: ['attributes', entity?.id],
    queryFn: () => metadataApi.getAttributes(entity!.id),
    enabled: !!entity,
  });

  // Load records
  const { data: records = [], isLoading } = useQuery({
    queryKey: ['records', entityName],
    queryFn: () => dataApi.list(entityName!),
    enabled: !!entityName,
  });

  // Create mutation
  const createMutation = useMutation({
    mutationFn: (data: any) => dataApi.create(entityName!, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['records', entityName] });
      setShowForm(false);
    },
  });

  // Update mutation
  const updateMutation = useMutation({
    mutationFn: ({ id, data }: { id: string; data: any }) =>
      dataApi.update(entityName!, id, data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['records', entityName] });
      setShowForm(false);
      setEditingRecord(null);
    },
  });

  // Delete mutation
  const deleteMutation = useMutation({
    mutationFn: (id: string) => dataApi.delete(entityName!, id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['records', entityName] });
    },
  });

  const handleSubmit = (data: any) => {
    if (editingRecord) {
      updateMutation.mutate({ id: editingRecord.id, data });
    } else {
      createMutation.mutate(data);
    }
  };

  const handleEdit = (record: Record) => {
    setEditingRecord(record);
    setShowForm(true);
  };

  const handleDelete = (record: Record) => {
    deleteMutation.mutate(record.id);
  };

  if (!entity) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-gray-500">Entity not found</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-gray-500">Loading...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <button
            onClick={() => navigate('/')}
            className="text-sm text-blue-600 hover:text-blue-800 mb-2"
          >
            ← Back to entities
          </button>
          <h1 className="text-2xl font-bold text-gray-900">
            {entity.display_name_plural}
          </h1>
          {entity.description && (
            <p className="mt-1 text-sm text-gray-600">{entity.description}</p>
          )}
        </div>
        <button
          onClick={() => {
            setEditingRecord(null);
            setShowForm(true);
          }}
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          + New {entity.display_name}
        </button>
      </div>

      {/* Form modal */}
      {showForm && (
        <div className="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 max-w-4xl w-full max-h-[90vh] overflow-y-auto">
            <h2 className="text-lg font-medium text-gray-900 mb-4">
              {editingRecord ? 'Edit' : 'New'} {entity.display_name}
            </h2>
            <DynamicForm
              attributes={attributes}
              initialData={editingRecord || {}}
              onSubmit={handleSubmit}
              onCancel={() => {
                setShowForm(false);
                setEditingRecord(null);
              }}
            />
          </div>
        </div>
      )}

      {/* Grid */}
      <div className="bg-white shadow overflow-hidden sm:rounded-lg">
        <DynamicGrid
          attributes={attributes}
          data={records}
          onEdit={handleEdit}
          onDelete={handleDelete}
        />
      </div>
    </div>
  );
};
