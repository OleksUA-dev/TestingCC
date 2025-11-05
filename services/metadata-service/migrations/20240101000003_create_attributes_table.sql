-- Create attributes table
CREATE TABLE attributes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    system_name VARCHAR(100) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    data_type data_type NOT NULL,
    is_required BOOLEAN NOT NULL DEFAULT FALSE,
    is_unique BOOLEAN NOT NULL DEFAULT FALSE,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    default_value JSONB,
    lookup_entity_id UUID REFERENCES entities(id),
    enum_values JSONB,
    validation_rules JSONB,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID,

    -- Ensure unique system_name per entity
    CONSTRAINT uq_attribute_system_name UNIQUE (entity_id, system_name)
);

-- Create indexes
CREATE INDEX idx_attributes_entity_id ON attributes(entity_id);
CREATE INDEX idx_attributes_system_name ON attributes(system_name);
CREATE INDEX idx_attributes_display_order ON attributes(entity_id, display_order);
CREATE INDEX idx_attributes_data_type ON attributes(data_type);
CREATE INDEX idx_attributes_is_active ON attributes(is_active);

-- Add comments
COMMENT ON TABLE attributes IS 'Metadata definition for entity attributes (fields/columns)';
COMMENT ON COLUMN attributes.system_name IS 'Unique system identifier within entity (e.g., "first_name", "email")';
COMMENT ON COLUMN attributes.data_type IS 'Data type of the attribute';
COMMENT ON COLUMN attributes.lookup_entity_id IS 'For Lookup data type: reference to target entity';
COMMENT ON COLUMN attributes.enum_values IS 'For Enum data type: JSON array of possible values';
COMMENT ON COLUMN attributes.validation_rules IS 'JSON object with validation rules (e.g., regex, min/max length)';
COMMENT ON COLUMN attributes.display_order IS 'Order in which attribute appears in forms';
