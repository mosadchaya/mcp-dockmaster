use super::{install_cursor_after_0470, install_cursor_before_0470, install_errors::CursorError};

pub fn is_cursor_installed(app_name: &str, after_0470: bool) -> Result<bool, CursorError> {
    if after_0470 {
        install_cursor_after_0470::is_installed(app_name)
    } else {
        let result = install_cursor_before_0470::is_cursor_installed(app_name);
        if result.is_err() {
            Err(result.err().unwrap())
        } else {
            Ok(true)
        }
    }
}

pub fn install_cursor(
    app_name: &str,
    binary_path: &str,
    after_0470: bool,
) -> Result<(), CursorError> {
    if after_0470 {
        install_cursor_after_0470::install_cursor(app_name, binary_path)
    } else {
        install_cursor_before_0470::install_cursor(app_name, binary_path)
    }
}

pub fn get_cursor_config(
    app_name: &str,
    binary_path: &str,
    after_0470: bool,
) -> Result<String, CursorError> {
    if after_0470 {
        install_cursor_after_0470::get_cursor_config(app_name, binary_path)
    } else {
        install_cursor_before_0470::get_cursor_config(app_name, binary_path)
    }
}
