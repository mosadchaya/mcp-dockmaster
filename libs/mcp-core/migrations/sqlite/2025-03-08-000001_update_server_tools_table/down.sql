-- Drop the new server_tools table
DROP TABLE IF EXISTS server_tools;

-- Recreate the original server_tools table
CREATE TABLE server_tools (
    server_id TEXT PRIMARY KEY,
    tool_data TEXT NOT NULL
);
