#[derive(Debug)]
pub enum ClaudeError {
    ConfigNotFound(String),
    InvalidJson(String),
    NoMcpServers,
    NoDockmaster,
    UnsupportedOS(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for ClaudeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaudeError::ConfigNotFound(path) => {
                write!(f, "Configuration file not found at {path}")
            }
            ClaudeError::InvalidJson(err) => write!(f, "Invalid JSON configuration: {err}"),
            ClaudeError::NoMcpServers => write!(f, "No mcpServers found in configuration"),
            ClaudeError::NoDockmaster => write!(f, "mcp-dockmaster not found in mcpServers"),
            ClaudeError::UnsupportedOS(os) => write!(f, "Unsupported operating system: {os}"),
            ClaudeError::IoError(err) => write!(f, "IO Error: {err}"),
        }
    }
}

impl std::error::Error for ClaudeError {}

impl From<std::io::Error> for ClaudeError {
    fn from(err: std::io::Error) -> Self {
        ClaudeError::IoError(err)
    }
}

impl From<serde_json::Error> for ClaudeError {
    fn from(err: serde_json::Error) -> Self {
        ClaudeError::InvalidJson(err.to_string())
    }
}

#[derive(Debug)]
pub enum CursorError {
    ConfigNotFound(String),
    DatabaseNotFound(String),
    DatabaseCorrupt(String),
    TableNotFound(String),
    KeyNotFound(String),
    InvalidJson(String),
    NoMcpServers,
    NoDockmaster,
    IoError(std::io::Error),
}

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorError::ConfigNotFound(path) => {
                write!(f, "Configuration file not found at {path}")
            }
            CursorError::DatabaseNotFound(path) => write!(f, "Database not found at {path}"),
            CursorError::DatabaseCorrupt(err) => write!(f, "Database is corrupt: {err}"),
            CursorError::TableNotFound(table) => write!(f, "Required table not found: {table}"),
            CursorError::KeyNotFound(key) => write!(f, "Required key not found: {key}"),
            CursorError::InvalidJson(err) => write!(f, "Invalid JSON configuration: {err}"),
            CursorError::IoError(err) => write!(f, "IO Error: {err}"),
            CursorError::NoMcpServers => write!(f, "No mcpServers found in configuration"),
            CursorError::NoDockmaster => write!(f, "mcp-dockmaster not found in mcpServers"),
        }
    }
}

impl std::error::Error for CursorError {}

impl From<std::io::Error> for CursorError {
    fn from(err: std::io::Error) -> Self {
        CursorError::IoError(err)
    }
}

impl From<diesel::result::Error> for CursorError {
    fn from(err: diesel::result::Error) -> Self {
        CursorError::DatabaseCorrupt(err.to_string())
    }
}

impl From<serde_json::Error> for CursorError {
    fn from(err: serde_json::Error) -> Self {
        CursorError::InvalidJson(err.to_string())
    }
}
