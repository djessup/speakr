// ============================================================================
//! Legacy Compatibility Commands
// ============================================================================
//!
//! This module provides **shim** implementations that keep historical
//! frontend code working while the Speakr backend APIs continue to
//! evolve. The functionality here should be considered **deprecated**
//! and will be removed once the migration to the new command set is
//! complete.
//!
//! # Provided Functionality
//! • `greet_internal` – Synchronous helper that returns a greeting string.
//!   Useful only for verifying the Tauri invoke bridge during development.
//! • `register_hot_key_internal` – Validates (but does **not** register)
//!   a global hot-key.  Real registration happens on the frontend; new
//!   code should call `register_global_hotkey` instead.
//!
//! # Deprecation Notice
//! These commands are maintained strictly for **backwards compatibility**.
//! Do **not** build new features that depend on them.

// ============================================================================
// External Imports
// ============================================================================

use speakr_types::AppError;
use tracing::{debug, warn};

// ============================================================================
// Internal Imports
// ============================================================================

use crate::commands::validation::validate_hot_key_internal;

// ============================================================================
// Legacy Command Implementations
// ============================================================================

// --------------------------------------------------------------------------
/// Simple greeting command for testing Tauri communication.
///
/// # Arguments
///
/// * `name` - The name to include in the greeting
///
/// # Returns
///
/// Returns a formatted greeting string.
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::legacy::greet_internal;
///
/// let greeting = greet_internal("Alice");
/// assert_eq!(greeting, "Hello, Alice! You've been greeted from Rust!");
/// ```
///
/// # Deprecation Notice
///
/// This command is primarily for testing and demonstration purposes.
/// Production applications should use more specific commands.
pub fn greet_internal(name: &str) -> String {
    debug!(name = %name, "Generating greeting message");
    format!("Hello, {name}! You've been greeted from Rust!")
}

// --------------------------------------------------------------------------
/// Registers a global hot-key with the system (legacy interface).
///
/// # Arguments
///
/// * `hot_key` - The hot-key combination to register
///
/// # Returns
///
/// Returns `Ok(())` if validation succeeds.
///
/// # Errors
///
/// Returns `AppError::HotKey` if the hot-key format is invalid.
///
/// # Deprecation Notice
///
/// This command provides a simple interface for hot-key registration
/// but delegates actual registration to the frontend. New implementations
/// should use `register_global_hotkey` and `unregister_global_hotkey`
/// commands which provide proper backend hot-key management.
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::legacy::register_hot_key_internal;
///
/// # #[tokio::main]
/// # async fn main() {
/// let result = register_hot_key_internal("CmdOrCtrl+Alt+Space".to_string()).await;
/// assert!(result.is_ok());
/// # }
/// ```
pub async fn register_hot_key_internal(hot_key: String) -> Result<(), AppError> {
    debug!(hot_key = %hot_key, "Legacy hot-key registration requested");

    // Validate the hot-key first using the validation module
    validate_hot_key_internal(hot_key.clone()).await?;

    warn!(
        hot_key = %hot_key,
        "Using legacy hot-key registration - consider migrating to register_global_hotkey"
    );

    // For backward compatibility, we just validate the hot-key format
    // The actual registration is handled by the frontend or newer commands
    debug!("Legacy hot-key validation successful, delegation to frontend");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_internal() {
        let result = greet_internal("Alice");
        assert_eq!(result, "Hello, Alice! You've been greeted from Rust!");
    }

    #[test]
    fn test_greet_internal_empty_name() {
        let result = greet_internal("");
        assert_eq!(result, "Hello, ! You've been greeted from Rust!");
    }

    #[test]
    fn test_greet_internal_special_characters() {
        let result = greet_internal("Test User 123!@#");
        assert_eq!(
            result,
            "Hello, Test User 123!@#! You've been greeted from Rust!"
        );
    }

    #[tokio::test]
    async fn test_register_hot_key_internal_valid() {
        let result = register_hot_key_internal("CmdOrCtrl+Alt+Space".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_hot_key_internal_invalid() {
        // Should fail validation
        let result = register_hot_key_internal("InvalidKey".to_string()).await;
        assert!(result.is_err());

        if let Err(AppError::HotKey(_)) = result {
            // Expected error type
        } else {
            panic!("Expected AppError::HotKey");
        }
    }

    #[tokio::test]
    async fn test_register_hot_key_internal_empty() {
        let result = register_hot_key_internal("".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_hot_key_internal_various_formats() {
        // Test various valid formats
        let valid_hotkeys = vec![
            "Cmd+Space",
            "CmdOrCtrl+Alt+V",
            "Shift+F1",
            "Ctrl+Alt+Delete",
            "Alt+Tab",
        ];

        for hotkey in valid_hotkeys {
            let result = register_hot_key_internal(hotkey.to_string()).await;
            assert!(result.is_ok(), "Failed for hotkey: {hotkey}");
        }
    }
}

// ============================================================================
