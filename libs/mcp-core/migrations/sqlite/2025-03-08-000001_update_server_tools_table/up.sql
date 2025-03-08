-- Drop the existing server_tools table
DROP TABLE IF EXISTS server_tools;

-- Create the new server_tools table with proper columns
CREATE TABLE server_tools (
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    input_schema TEXT,
    server_id TEXT NOT NULL,
    proxy_id TEXT,
    PRIMARY KEY (id, server_id),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

-- Create an index for faster lookups by server_id
CREATE INDEX idx_server_tools_server_id ON server_tools(server_id);
