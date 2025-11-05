-- Create form_layouts table
CREATE TABLE form_layouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    sections JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(entity_id, name)
);

-- Create grid_layouts table
CREATE TABLE grid_layouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    columns JSONB NOT NULL,
    default_sort_by VARCHAR(100),
    default_sort_order VARCHAR(10),
    page_size INTEGER NOT NULL DEFAULT 50,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(entity_id, name)
);

-- Create indexes
CREATE INDEX idx_form_layouts_entity ON form_layouts(entity_id);
CREATE INDEX idx_form_layouts_default ON form_layouts(entity_id, is_default);
CREATE INDEX idx_grid_layouts_entity ON grid_layouts(entity_id);
CREATE INDEX idx_grid_layouts_default ON grid_layouts(entity_id, is_default);

-- Add comments
COMMENT ON TABLE form_layouts IS 'Form UI configurations for entities';
COMMENT ON TABLE grid_layouts IS 'Grid/List view configurations for entities';
COMMENT ON COLUMN form_layouts.sections IS 'JSON array of form sections with fields';
COMMENT ON COLUMN grid_layouts.columns IS 'JSON array of grid columns configuration';
