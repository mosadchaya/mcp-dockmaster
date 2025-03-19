pub mod handlers;
pub mod mcp_client_manager;
pub mod tools;
pub mod mcp_router;

// Re-export public items
pub use self::handlers::{ClientManager, ClientManagerTrait};
pub use self::handlers::start_mcp_server;
pub use self::mcp_client_manager::MCPClientManager;
pub use self::tools::{TOOL_REGISTER_SERVER, get_register_server_tool};
pub use self::mcp_router::MCPDockmasterRouter; 