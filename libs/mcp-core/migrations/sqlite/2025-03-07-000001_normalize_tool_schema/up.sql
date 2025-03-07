-- Create a new normalized tools table
CREATE TABLE new_tools (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    tool_type TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    entry_point TEXT NULL,
    command TEXT NULL,
    args TEXT NULL, -- stored as JSON array
    distribution_type TEXT NULL,
    distribution_package TEXT NULL
);

-- Create a table for environment variables
CREATE TABLE tool_env (
    tool_id TEXT NOT NULL,
    env_key TEXT NOT NULL,
    env_value TEXT NOT NULL,
    env_description TEXT NOT NULL DEFAULT '',
    env_required BOOLEAN NOT NULL DEFAULT 0,
    PRIMARY KEY (tool_id, env_key),
    FOREIGN KEY(tool_id) REFERENCES new_tools(id) ON DELETE CASCADE
);

-- Migrate data from the old tools table to the new tables
INSERT INTO new_tools (id, name, description, tool_type, enabled, entry_point, command, args, distribution_type, distribution_package)
SELECT 
    tools.id,
    json_extract(data, '$.name'),
    json_extract(data, '$.description'),
    json_extract(data, '$.tool_type'),
    json_extract(data, '$.enabled'),
    json_extract(data, '$.entry_point'),
    json_extract(data, '$.configuration.command'),
    json_extract(data, '$.configuration.args'),
    json_extract(data, '$.distribution.type'),
    json_extract(data, '$.distribution.package')
FROM tools;

-- Insert environment variables from the old tools table
INSERT INTO tool_env (tool_id, env_key, env_value, env_description, env_required)
WITH env_keys AS (
    SELECT 
        tools.id AS tool_id,
        json_each.key AS env_key,
        json_extract(json_each.value, '$.default') AS env_value,
        json_extract(json_each.value, '$.description') AS env_description,
        json_extract(json_each.value, '$.required') AS env_required
    FROM tools, 
    json_each(json_extract(data, '$.configuration.env'))
)
SELECT tool_id, env_key, COALESCE(env_value, ''), COALESCE(env_description, ''), COALESCE(env_required, 0)
FROM env_keys;

-- Drop the old tools table and rename the new one
DROP TABLE tools;
ALTER TABLE new_tools RENAME TO tools;
