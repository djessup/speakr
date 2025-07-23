//! Settings directory validation and permissions checking.

use speakr_types::AppError;
use std::path::Path;

/// Validates directory permissions for settings operations.
///
/// # Arguments
///
/// * `dir_path` - The directory path to validate
///
/// # Returns
///
/// Returns `Ok(())` if permissions are valid.
///
/// # Errors
///
/// Returns `AppError` if permissions are insufficient.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn validate_settings_directory_permissions(dir_path: &Path) -> Result<(), AppError> {
    // Minimal implementation to pass the test
    // Check if directory exists and is writable
    if !dir_path.exists() {
        return Err(AppError::FileSystem("Directory does not exist".to_string()));
    }

    // Try to create a test file to verify write permissions
    let test_file = dir_path.join(".permission_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            // Clean up test file
            let _ = std::fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => Err(AppError::FileSystem(format!("Directory not writable: {e}"))),
    }
}
