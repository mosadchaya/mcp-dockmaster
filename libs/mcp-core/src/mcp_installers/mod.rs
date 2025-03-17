mod install_claude;
mod install_cursor;
mod install_errors;
mod install_paths;
pub use self::install_claude::get_claude_config;
pub use self::install_claude::install_claude;
pub use self::install_claude::is_claude_installed;
pub use self::install_cursor::get_cursor_config;
pub use self::install_cursor::install_cursor;
pub use self::install_cursor::is_cursor_installed;

pub use self::install_paths::get_generic_config;
