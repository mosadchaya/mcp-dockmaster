use std::fmt;

#[derive(Debug)]
pub enum DomainError {
    ToolNotFound(String),
    ToolAlreadyExists(String),
    InvalidToolConfiguration(String),
    ConfigurationError(String),
    ProcessError(String),
    RepositoryError(String),
    ServerNotFound(String),
    ServerClosedConnection,
    StdinWriteError(String),
    StdinFlushError(String),
    StdoutReadError(String),
    TimeoutError(String),
    NoResponse,
    JsonParseError(String),
    ToolExecutionError(String),
    NoResultField,
    SerializationError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::ToolNotFound(id) => write!(f, "Tool not found: {}", id),
            DomainError::ToolAlreadyExists(id) => write!(f, "Tool already exists: {}", id),
            DomainError::InvalidToolConfiguration(msg) => write!(f, "Invalid tool configuration: {}", msg),
            DomainError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            DomainError::ProcessError(msg) => write!(f, "Process error: {}", msg),
            DomainError::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            DomainError::ServerNotFound(id) => write!(f, "Server not found: {}", id),
            DomainError::ServerClosedConnection => write!(f, "Server closed connection"),
            DomainError::StdinWriteError(msg) => write!(f, "Failed to write to process stdin: {}", msg),
            DomainError::StdinFlushError(msg) => write!(f, "Failed to flush stdin: {}", msg),
            DomainError::StdoutReadError(msg) => write!(f, "Failed to read from process stdout: {}", msg),
            DomainError::TimeoutError(id) => write!(f, "Timeout waiting for response from server {}", id),
            DomainError::NoResponse => write!(f, "No response from process"),
            DomainError::JsonParseError(msg) => write!(f, "Failed to parse response as JSON: {}", msg),
            DomainError::ToolExecutionError(msg) => write!(f, "Tool execution error: {}", msg),
            DomainError::NoResultField => write!(f, "Response missing result field"),
            DomainError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for DomainError {}

// Conversion from String to DomainError
impl From<String> for DomainError {
    fn from(error: String) -> Self {
        DomainError::RepositoryError(error)
    }
}
