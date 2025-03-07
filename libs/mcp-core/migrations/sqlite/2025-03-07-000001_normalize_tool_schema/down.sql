-- Recreate the original tools table structure
CREATE TABLE old_tools (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL
);

-- Migrate data back to the original format
INSERT INTO old_tools (id, data)
SELECT 
    t.id,
    json_object(
        'name', t.name,
        'description', t.description,
        'enabled', t.enabled,
        'tool_type', t.tool_type,
        'entry_point', t.entry_point,
        'configuration', json_object(
            'command', t.command,
            'args', t.args,
            'env', (
                SELECT json_group_object(
                    env_key, 
                    json_object(
                        'description', env_description,
                        'default', env_value,
                        'required', env_required
                    )
                )
                FROM tool_env
                WHERE tool_env.tool_id = t.id
            )
        ),
        'distribution', CASE 
            WHEN t.distribution_type IS NOT NULL THEN json_object(
                'type', t.distribution_type,
                'package', t.distribution_package
            )
            ELSE NULL
        END
    ) AS data
FROM tools t;

-- Drop the new tables
DROP TABLE tool_env;
DROP TABLE tools;

-- Rename the old table back to tools
ALTER TABLE old_tools RENAME TO tools;
