-- Create data_type enum
CREATE TYPE data_type AS ENUM (
    'string',
    'text',
    'integer',
    'float',
    'decimal',
    'boolean',
    'date_time',
    'date',
    'lookup',
    'enum'
);
