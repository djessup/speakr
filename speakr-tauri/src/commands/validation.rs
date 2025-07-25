// ============================================================================
//! Input Validation Commands
// ============================================================================
//!
//! This module provides validation functions for user input, particularly
//! for hotkey format validation and input sanitisation.

use speakr_types::AppError;
use tracing::warn;

/// Validates that a hot-key string is in the correct format.
///
/// # Arguments
///
/// * `hot_key` - The hot-key combination string to validate
///
/// # Returns
///
/// Returns `Ok(())` if the hot-key format is valid.
///
/// # Errors
///
/// Returns `AppError::HotKey` if:
/// - The hot-key format is invalid
/// - Required modifiers are missing
/// - Unsupported key combinations are used
///
/// # Supported Formats
///
/// - `Cmd+Key` (macOS Command key)
/// - `CmdOrCtrl+Key` (Command on macOS, Ctrl on other platforms)
/// - `Alt+Key` (Alt/Option key)
/// - `Shift+Key` (Shift key)
/// - Multiple modifiers: `Cmd+Alt+Key`, `CmdOrCtrl+Alt+Key`, etc.
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::validation::validate_hot_key_internal;
///
/// # #[tokio::main]
/// # async fn main() {
/// // Valid hotkeys
/// assert!(validate_hot_key_internal("CmdOrCtrl+Alt+Space".to_string()).await.is_ok());
/// assert!(validate_hot_key_internal("Cmd+Shift+V".to_string()).await.is_ok());
///
/// // Invalid hotkeys
/// assert!(validate_hot_key_internal("InvalidKey".to_string()).await.is_err());
/// assert!(validate_hot_key_internal("".to_string()).await.is_err());
/// # }
/// ```
pub async fn validate_hot_key_internal(hot_key: String) -> Result<(), AppError> {
    // Input sanitisation
    let hot_key = hot_key.trim();

    if hot_key.is_empty() {
        return Err(AppError::HotKey("Hot-key cannot be empty".to_string()));
    }

    // Basic format validation - check for modifiers (case-insensitive for legacy support)
    let hot_key_upper = hot_key.to_uppercase();
    let has_modifier = hot_key_upper.contains("CMD")
        || hot_key_upper.contains("CMDORCTRL")
        || hot_key_upper.contains("ALT")
        || hot_key_upper.contains("SHIFT")
        || hot_key_upper.contains("CTRL");

    if !has_modifier {
        return Err(AppError::HotKey(
            "Hot-key must include at least one modifier (Cmd, CmdOrCtrl, Alt, Shift, or Ctrl)"
                .to_string(),
        ));
    }

    // Check for valid separator
    if !hot_key.contains('+') {
        return Err(AppError::HotKey(
            "Hot-key must use '+' to separate modifiers and keys".to_string(),
        ));
    }

    // Split by '+' and validate parts
    let parts: Vec<&str> = hot_key.split('+').collect();
    if parts.len() < 2 {
        return Err(AppError::HotKey(
            "Hot-key must have at least one modifier and one key".to_string(),
        ));
    }

    // Check that the last part is a valid key (not another modifier)
    if let Some(key_part) = parts.last() {
        let key_part = key_part.trim();
        if key_part.is_empty() {
            return Err(AppError::HotKey(
                "Hot-key must end with a valid key".to_string(),
            ));
        }

        // Ensure the last part isn't a modifier (would indicate malformed input)
        let key_part_upper = key_part.to_uppercase();
        if ["CMD", "CMDORCTRL", "ALT", "SHIFT", "CTRL"].contains(&key_part_upper.as_str()) {
            return Err(AppError::HotKey(
                "Hot-key must end with a key, not a modifier".to_string(),
            ));
        }
    }

    // Warn about potentially problematic combinations
    if hot_key.contains("Cmd+") && hot_key.contains("CmdOrCtrl+") {
        warn!(
            "Hot-key contains both Cmd and CmdOrCtrl modifiers: {}",
            hot_key
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_validate_hot_key_internal_valid_combinations() {
        // Valid single modifier combinations
        assert!(validate_hot_key_internal("CmdOrCtrl+Space".to_string())
            .await
            .is_ok());
        assert!(validate_hot_key_internal("Alt+Tab".to_string())
            .await
            .is_ok());
        assert!(validate_hot_key_internal("Shift+F1".to_string())
            .await
            .is_ok());

        // Valid multiple modifier combinations
        assert!(validate_hot_key_internal("CmdOrCtrl+Alt+Space".to_string())
            .await
            .is_ok());
        assert!(validate_hot_key_internal("Cmd+Shift+V".to_string())
            .await
            .is_ok());
        assert!(validate_hot_key_internal("Ctrl+Alt+Delete".to_string())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_validate_hot_key_internal_invalid_format() {
        // Empty or whitespace-only input
        assert!(validate_hot_key_internal("".to_string()).await.is_err());
        assert!(validate_hot_key_internal("   ".to_string()).await.is_err());

        // No modifiers
        assert!(validate_hot_key_internal("Space".to_string())
            .await
            .is_err());
        assert!(validate_hot_key_internal("Tab".to_string()).await.is_err());

        // Missing separator
        assert!(validate_hot_key_internal("CmdSpace".to_string())
            .await
            .is_err());
        assert!(validate_hot_key_internal("AltTab".to_string())
            .await
            .is_err());

        // Ending with modifier
        assert!(validate_hot_key_internal("Cmd+Alt+".to_string())
            .await
            .is_err());
        assert!(validate_hot_key_internal("CmdOrCtrl+Shift".to_string())
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_validate_hot_key_internal_edge_cases() {
        // Whitespace handling
        assert!(
            validate_hot_key_internal("  CmdOrCtrl+Alt+Space  ".to_string())
                .await
                .is_ok()
        );

        // Single character keys
        assert!(validate_hot_key_internal("Cmd+A".to_string()).await.is_ok());
        assert!(validate_hot_key_internal("Alt+1".to_string()).await.is_ok());

        // Function keys
        assert!(validate_hot_key_internal("Shift+F12".to_string())
            .await
            .is_ok());
    }
}
