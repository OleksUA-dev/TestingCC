import React from 'react';
import { useReactTable, getCoreRowModel, flexRender, ColumnDef } from '@tanstack/react-table';
import { Attribute, Record } from '../types';

interface DynamicGridProps {
  attributes: Attribute[];
  data: Record[];
  onRowClick?: (record: Record) => void;
  onEdit?: (record: Record) => void;
  onDelete?: (record: Record) => void;
}

export const DynamicGrid: React.FC<DynamicGridProps> = ({
  attributes,
  data,
  onRowClick,
  onEdit,
  onDelete,
}) => {
  const columns: ColumnDef<Record>[] = React.useMemo(() => {
    const attrColumns: ColumnDef<Record>[] = attributes
      .filter((attr) => attr.is_active)
      .sort((a, b) => a.display_order - b.display_order)
      .map((attr) => ({
        id: attr.system_name,
        accessorKey: attr.system_name,
        header: attr.display_name,
        cell: (info) => {
          const value = info.getValue();

          // Format based on data type
          switch (attr.data_type) {
            case 'boolean':
              return value ? '✓' : '✗';
            case 'date':
            case 'date_time':
              return value ? new Date(value as string).toLocaleDateString() : '-';
            case 'decimal':
            case 'float':
              return typeof value === 'number' ? value.toFixed(2) : value;
            default:
              return value || '-';
          }
        },
      }));

    // Add actions column
    if (onEdit || onDelete) {
      attrColumns.push({
        id: 'actions',
        header: 'Actions',
        cell: ({ row }) => (
          <div className="flex space-x-2">
            {onEdit && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onEdit(row.original);
                }}
                className="text-blue-600 hover:text-blue-900"
              >
                Edit
              </button>
            )}
            {onDelete && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  if (confirm('Are you sure you want to delete this record?')) {
                    onDelete(row.original);
                  }
                }}
                className="text-red-600 hover:text-red-900"
              >
                Delete
              </button>
            )}
          </div>
        ),
      });
    }

    return attrColumns;
  }, [attributes, onEdit, onDelete]);

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  if (data.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500">No records found</p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th
                  key={header.id}
                  className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                >
                  {flexRender(header.column.columnDef.header, header.getContext())}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {table.getRowModel().rows.map((row) => (
            <tr
              key={row.id}
              onClick={() => onRowClick?.(row.original)}
              className={onRowClick ? 'cursor-pointer hover:bg-gray-50' : ''}
            >
              {row.getVisibleCells().map((cell) => (
                <td key={cell.id} className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};
