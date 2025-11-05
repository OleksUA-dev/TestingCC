import React from 'react';
import { useForm } from 'react-hook-form';
import { Attribute, DataType } from '../types';

interface DynamicFormProps {
  attributes: Attribute[];
  initialData?: any;
  onSubmit: (data: any) => void;
  onCancel?: () => void;
}

export const DynamicForm: React.FC<DynamicFormProps> = ({
  attributes,
  initialData = {},
  onSubmit,
  onCancel,
}) => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    defaultValues: initialData,
  });

  const renderField = (attr: Attribute) => {
    const fieldProps = {
      ...register(attr.system_name, {
        required: attr.is_required && `${attr.display_name} is required`,
      }),
      id: attr.system_name,
      placeholder: attr.display_name,
    };

    switch (attr.data_type) {
      case 'boolean':
        return (
          <input
            type="checkbox"
            {...fieldProps}
            className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          />
        );

      case 'text':
        return (
          <textarea
            {...fieldProps}
            rows={4}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        );

      case 'integer':
      case 'float':
      case 'decimal':
        return (
          <input
            type="number"
            step={attr.data_type === 'integer' ? '1' : 'any'}
            {...fieldProps}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        );

      case 'date':
        return (
          <input
            type="date"
            {...fieldProps}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        );

      case 'date_time':
        return (
          <input
            type="datetime-local"
            {...fieldProps}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        );

      case 'enum':
        return (
          <select
            {...fieldProps}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="">Select {attr.display_name}</option>
            {attr.enum_values?.map((ev) => (
              <option key={ev.key} value={ev.key}>
                {ev.value}
              </option>
            ))}
          </select>
        );

      case 'string':
      default:
        return (
          <input
            type="text"
            {...fieldProps}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        );
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
      {/* Form fields */}
      <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
        {attributes
          .filter((attr) => attr.is_active)
          .sort((a, b) => a.display_order - b.display_order)
          .map((attr) => (
            <div key={attr.id} className="space-y-1">
              <label
                htmlFor={attr.system_name}
                className="block text-sm font-medium text-gray-700"
              >
                {attr.display_name}
                {attr.is_required && <span className="text-red-500 ml-1">*</span>}
              </label>

              {renderField(attr)}

              {attr.description && (
                <p className="text-xs text-gray-500">{attr.description}</p>
              )}

              {errors[attr.system_name] && (
                <p className="text-xs text-red-600">
                  {errors[attr.system_name]?.message as string}
                </p>
              )}
            </div>
          ))}
      </div>

      {/* Action buttons */}
      <div className="flex justify-end space-x-3 pt-4 border-t">
        {onCancel && (
          <button
            type="button"
            onClick={onCancel}
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            Cancel
          </button>
        )}
        <button
          type="submit"
          className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          Save
        </button>
      </div>
    </form>
  );
};
