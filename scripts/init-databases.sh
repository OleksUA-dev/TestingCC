#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    -- Create metadata database
    CREATE DATABASE corecrm_metadata;
    GRANT ALL PRIVILEGES ON DATABASE corecrm_metadata TO $POSTGRES_USER;

    -- Create gateway database
    CREATE DATABASE corecrm_gateway;
    GRANT ALL PRIVILEGES ON DATABASE corecrm_gateway TO $POSTGRES_USER;

    -- Print success message
    SELECT 'CoreCRM-R databases created successfully!' AS message;
EOSQL
