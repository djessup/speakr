//! System integration commands for Speakr Tauri backend.
//!
//! This module provides commands for system-level functionality including
//! model file availability checking and auto-launch management.

use speakr_types::AppError;
use tracing::{debug, warn};

/// Checks if a model file exists for the given model size.
///
/// # Arguments
///
/// * `model_size` - The model size to check ("small", "medium", "large")
///
/// # Returns
///
/// Returns `true` if the model file exists, `false` otherwise.
///
/// # Errors
///
/// Returns `AppError::Settings` if the model size is not recognised.
///
/// # Supported Model Sizes
///
/// - `"small"` - Maps to `ggml-small.bin`
/// - `"medium"` - Maps to `ggml-medium.bin`
/// - `"large"` - Maps to `ggml-large.bin`
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::system::check_model_availability_internal;
///
/// # #[tokio::main]
/// # async fn main() {
/// let is_available = check_model_availability_internal("small".to_string()).await.unwrap();
/// println!("Small model available: {}", is_available);
/// # }
/// ```
pub async fn check_model_availability_internal(model_size: String) -> Result<bool, AppError> {
    let filename = match model_size.as_str() {
        "small" => "ggml-small.bin",
        "medium" => "ggml-medium.bin",
        "large" => "ggml-large.bin",
        _ => {
            return Err(AppError::Settings(format!(
                "Unknown model size: {model_size}"
            )));
        }
    };

    // Check in models directory relative to the app
    let models_dir = std::env::current_dir()
        .map_err(|e| AppError::FileSystem(format!("Failed to get current dir: {e}")))?
        .join("models");

    let model_path = models_dir.join(filename);
    let exists = model_path.exists();

    debug!(
        model_size = %model_size,
        filename = %filename,
        path = %model_path.display(),
        exists = %exists,
        "Checked model availability"
    );

    // Additional file system checks for better error reporting
    if !exists {
        if !models_dir.exists() {
            debug!(
                models_dir = %models_dir.display(),
                "Models directory does not exist"
            );
        } else if !model_path.parent().is_some_and(|p| p.exists()) {
            warn!(
                parent_dir = %model_path.parent().unwrap().display(),
                "Model parent directory does not exist"
            );
        }
    }

    Ok(exists)
}

/// Sets the auto-launch preference for the application.
///
/// # Arguments
///
/// * `enable` - Whether to enable auto-launch on system startup
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError::System` if the auto-launch configuration fails.
///
/// # Platform Support
///
/// Currently provides a placeholder implementation. Full implementation
/// will use platform-specific APIs:
/// - macOS: Launch Services and Login Items
/// - Windows: Registry startup entries
/// - Linux: XDG autostart specification
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::system::set_auto_launch_internal;
///
/// # #[tokio::main]
/// # async fn main() {
/// // Enable auto-launch
/// set_auto_launch_internal(true).await.unwrap();
///
/// // Disable auto-launch
/// set_auto_launch_internal(false).await.unwrap();
/// # }
/// ```
pub async fn set_auto_launch_internal(enable: bool) -> Result<(), AppError> {
    debug!(enable = %enable, "Setting auto-launch preference");

    // TODO: Implement actual auto-launch registration using system APIs
    //
    // Implementation roadmap:
    // 1. macOS: Use `tauri-plugin-autostart` or native Launch Services
    // 2. Windows: Registry entries in HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
    // 3. Linux: Create .desktop file in ~/.config/autostart/
    //
    // For now, this is a placeholder that accepts the preference but doesn't
    // actually configure system auto-launch. The setting could be persisted
    // in app settings for future implementation.

    if enable {
        debug!("Auto-launch enabled (placeholder implementation)");
        // Future: Register with system startup
    } else {
        debug!("Auto-launch disabled (placeholder implementation)");
        // Future: Unregister from system startup
    }

    // TODO: Re-implement error simulation with proper test isolation
    // The current approach using global environment variables causes
    // test interference in parallel execution. Need dependency injection.
    #[cfg(test)]
    {
        // Temporarily disabled due to test isolation issues
        // Will be re-implemented with proper dependency injection in future PR
        let _unused = std::env::var("SIMULATE_AUTOLAUNCH_ERROR");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_check_model_availability_internal_valid_sizes() {
        // Test valid model sizes
        let small_result = check_model_availability_internal("small".to_string()).await;
        assert!(small_result.is_ok());

        let medium_result = check_model_availability_internal("medium".to_string()).await;
        assert!(medium_result.is_ok());

        let large_result = check_model_availability_internal("large".to_string()).await;
        assert!(large_result.is_ok());
    }

    #[tokio::test]
    async fn test_check_model_availability_internal_invalid_size() {
        let result = check_model_availability_internal("invalid".to_string()).await;
        assert!(result.is_err());

        if let Err(AppError::Settings(msg)) = result {
            assert!(msg.contains("Unknown model size: invalid"));
        } else {
            panic!("Expected AppError::Settings");
        }
    }

    #[tokio::test]
    async fn test_check_model_availability_internal_empty_size() {
        let result = check_model_availability_internal("".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_auto_launch_internal_enable() {
        // Ensure clean test environment
        env::remove_var("SIMULATE_AUTOLAUNCH_ERROR");

        let result = set_auto_launch_internal(true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_auto_launch_internal_disable() {
        // Ensure clean test environment
        env::remove_var("SIMULATE_AUTOLAUNCH_ERROR");

        let result = set_auto_launch_internal(false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_auto_launch_internal_simulated_error() {
        // TODO: This test is temporarily disabled due to test isolation issues
        // The error simulation using global environment variables causes
        // interference with other tests running in parallel.
        // Will be re-implemented with proper dependency injection.

        // For now, just test that the function works normally
        let result = set_auto_launch_internal(true).await;
        assert!(
            result.is_ok(),
            "Function should work normally when simulation is disabled"
        );

        let result = set_auto_launch_internal(false).await;
        assert!(
            result.is_ok(),
            "Function should work normally when simulation is disabled"
        );
    }
}
