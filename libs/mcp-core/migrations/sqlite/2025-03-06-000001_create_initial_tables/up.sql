CREATE TABLE tools (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL
);

CREATE TABLE server_tools (
    server_id TEXT,
    tool_data TEXT NOT NULL,
    PRIMARY KEY (server_id)
);
