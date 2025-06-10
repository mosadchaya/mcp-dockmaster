-- Remove the new columns added for custom server support
ALTER TABLE servers DROP COLUMN server_type;
ALTER TABLE servers DROP COLUMN working_directory; 
ALTER TABLE servers DROP COLUMN executable_path;

-- Drop the index
DROP INDEX IF EXISTS idx_servers_type;