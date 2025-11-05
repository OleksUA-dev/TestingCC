-- Create entities table
CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_name VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    display_name_plural VARCHAR(255) NOT NULL,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    icon VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID,
    updated_by UUID
);

-- Create indexes
CREATE INDEX idx_entities_system_name ON entities(system_name);
CREATE INDEX idx_entities_is_active ON entities(is_active);
CREATE INDEX idx_entities_created_at ON entities(created_at DESC);

-- Add comments
COMMENT ON TABLE entities IS 'Metadata definition for business entities (e.g., Contact, Account, Invoice)';
COMMENT ON COLUMN entities.system_name IS 'Unique system identifier used in API and database (e.g., "contact", "invoice")';
COMMENT ON COLUMN entities.display_name IS 'User-friendly name displayed in UI';
COMMENT ON COLUMN entities.is_system IS 'If true, this is a system entity and cannot be deleted';
