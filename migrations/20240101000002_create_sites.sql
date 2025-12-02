-- Create sites table
CREATE TABLE IF NOT EXISTS sites (
    program_address TEXT PRIMARY KEY,
    owner_pubkey TEXT NOT NULL,
    storage_cid TEXT NOT NULL,
    name TEXT,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for search
CREATE INDEX IF NOT EXISTS idx_sites_owner ON sites(owner_pubkey);
CREATE INDEX IF NOT EXISTS idx_sites_name ON sites(name);
CREATE INDEX IF NOT EXISTS idx_sites_created_at ON sites(created_at DESC);

-- Create updated_at trigger
CREATE TRIGGER update_sites_updated_at BEFORE UPDATE ON sites
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

