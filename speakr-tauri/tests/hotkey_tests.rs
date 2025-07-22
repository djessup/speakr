// Hotkey-related tests extracted from lib.rs

// No unused imports

// Import functions from the speakr_lib crate (now pub)
use speakr_lib::validate_hot_key_internal;

#[tokio::test]
async fn test_validate_hot_key_success() {
    // Arrange
    let valid_keys = vec![
        "CmdOrCtrl+Alt+Space".to_string(),
        "Ctrl+Shift+F".to_string(),
        "Alt+`".to_string(),
        "CMD+SPACE".to_string(), // Legacy format support
    ];

    // Act & Assert
    for key in valid_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_ok(), "Should accept valid key: {key}");
    }
}

#[tokio::test]
async fn test_validate_hot_key_failures() {
    let invalid_keys = vec![
        "".to_string(),      // Empty
        "Space".to_string(), // No modifier
        "A+B".to_string(),   // No modifier keys
    ];

    for key in invalid_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_err(), "Should reject invalid key: {key}");
    }
}

// 🔴 RED: Tests for function key support (should fail initially)
#[tokio::test]
async fn test_validate_function_keys() {
    let function_keys = vec![
        "CmdOrCtrl+F1".to_string(),
        "CmdOrCtrl+F2".to_string(),
        "CmdOrCtrl+F3".to_string(),
        "CmdOrCtrl+F4".to_string(),
        "CmdOrCtrl+F5".to_string(),
        "CmdOrCtrl+F6".to_string(),
        "CmdOrCtrl+F7".to_string(),
        "CmdOrCtrl+F8".to_string(),
        "CmdOrCtrl+F9".to_string(),
        "CmdOrCtrl+F10".to_string(),
        "CmdOrCtrl+F11".to_string(),
        "CmdOrCtrl+F12".to_string(),
        "Alt+F1".to_string(),
        "Shift+F2".to_string(),
        "Ctrl+Alt+F3".to_string(),
    ];

    for key in function_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_ok(), "Should accept function key: {key}");
    }
}

// 🔵 REFACTOR: Tests for supported special keys (refined based on Tauri support)
#[tokio::test]
async fn test_validate_special_keys() {
    let special_keys = vec![
        "CmdOrCtrl+Home".to_string(),
        "CmdOrCtrl+End".to_string(),
        "CmdOrCtrl+PageUp".to_string(),
        "CmdOrCtrl+PageDown".to_string(),
        "CmdOrCtrl+ArrowUp".to_string(),
        "CmdOrCtrl+ArrowDown".to_string(),
        "CmdOrCtrl+ArrowLeft".to_string(),
        "CmdOrCtrl+ArrowRight".to_string(),
        "CmdOrCtrl+Insert".to_string(),
        "Shift+Insert".to_string(),
    ];

    for key in special_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_ok(), "Should accept special key: {key}");
    }
}

// 🔴 RED: Tests for numeric keys
#[tokio::test]
async fn test_validate_numeric_keys() {
    let numeric_keys = vec![
        "CmdOrCtrl+0".to_string(),
        "CmdOrCtrl+1".to_string(),
        "CmdOrCtrl+2".to_string(),
        "CmdOrCtrl+3".to_string(),
        "CmdOrCtrl+4".to_string(),
        "CmdOrCtrl+5".to_string(),
        "CmdOrCtrl+6".to_string(),
        "CmdOrCtrl+7".to_string(),
        "CmdOrCtrl+8".to_string(),
        "CmdOrCtrl+9".to_string(),
    ];

    for key in numeric_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_ok(), "Should accept numeric key: {key}");
    }
}

// 🔵 REFACTOR: Tests for commonly supported punctuation and symbol keys
#[tokio::test]
async fn test_validate_symbol_keys() {
    let symbol_keys = vec![
        "CmdOrCtrl+;".to_string(),
        "CmdOrCtrl+,".to_string(),
        "CmdOrCtrl+.".to_string(),
        "CmdOrCtrl+/".to_string(),
        "CmdOrCtrl+[".to_string(),
        "CmdOrCtrl+]".to_string(),
        "CmdOrCtrl+-".to_string(),
        "CmdOrCtrl+=".to_string(),
    ];

    for key in symbol_keys {
        let result = validate_hot_key_internal(key.clone()).await;
        assert!(result.is_ok(), "Should accept symbol key: {key}");
    }
}
