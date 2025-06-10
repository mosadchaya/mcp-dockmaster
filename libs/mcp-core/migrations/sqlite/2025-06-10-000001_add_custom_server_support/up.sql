-- Add new columns to servers table for custom server support
ALTER TABLE servers ADD COLUMN server_type TEXT DEFAULT 'package';
ALTER TABLE servers ADD COLUMN working_directory TEXT;
ALTER TABLE servers ADD COLUMN executable_path TEXT;

-- Create enum-like constraint for server_type
-- Options: 'package' (existing), 'local', 'custom'
-- package: Standard npm/pip/docker packages (current behavior)
-- local: Local filesystem servers (clanki, local projects)
-- custom: Fully custom configurations

-- Add index for new server_type field for better querying
CREATE INDEX idx_servers_type ON servers(server_type);

-- Update existing servers to use 'package' type (default already set above)
-- This ensures backward compatibility