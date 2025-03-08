CREATE TABLE servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    tool_type TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    entry_point TEXT,
    command TEXT,
    args TEXT,
    distribution_type TEXT,
    distribution_package TEXT
);

CREATE INDEX idx_servers_id_desc ON servers(id DESC);

CREATE TABLE server_env (
    server_id TEXT NOT NULL,
    env_key TEXT NOT NULL,
    env_value TEXT NOT NULL,
    env_description TEXT NOT NULL,
    env_required BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (server_id, env_key),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE TABLE server_tools (
    server_id TEXT PRIMARY KEY,
    tool_data TEXT NOT NULL
);
