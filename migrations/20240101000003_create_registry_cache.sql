-- Create registry cache table for faster lookups
CREATE TABLE IF NOT EXISTS registry_cache (
    program_address TEXT PRIMARY KEY,
    metadata JSONB,
    last_synced TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create index for metadata search
CREATE INDEX IF NOT EXISTS idx_registry_metadata ON registry_cache USING GIN (metadata);

