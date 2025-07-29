// Hotkey-related tests extracted from lib.rs
#![allow(clippy::assertions_on_constants)] // Allows asserting on constant values in demonstration tests

// No unused imports

// Import functions from the speakr_lib crate (now pub)
use speakr_lib::commands::validation::validate_hot_key_internal;
use speakr_types::{AppSettings, HotkeyConfig};
// Additional imports for merged tests

#[allow(unused_imports)]
use tauri_plugin_global_shortcut::ShortcutState;

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

// Tests for function key support
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

// Tests for numeric key combinations
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

// ============================================================================
// HotkeyConfig & Update Hotkey Tests (merged from update_hotkey_tests.rs)
// ============================================================================

// #[tokio::test]
// async fn test_single_press_emits_event_once() {
//     use tauri::{test::mock_app, Manager};
//     use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
//     use std::time::Duration;
//     // 1. minimal mock app
//     let mut app = mock_app();
//     let app_handle = app.app_handle();
//     // 2. build the service & register one hot-key
//     let mut service = GlobalHotkeyService::new(app_handle.clone()).unwrap();
//     let cfg = HotkeyConfig { shortcut: "Ctrl+Alt+T".into(), enabled: true };
//     service.register_hotkey(&cfg).await.unwrap();
//     // 3. listen for the event and count hits
//     let (tx, rx) = std::sync::mpsc::channel();
//     app_handle.listen("hotkey-triggered", move |_| { tx.send(()).unwrap(); });
//     // 4. manually invoke the on_shortcut callback the same way the plugin does
//     let shortcut = cfg.shortcut.parse::<Shortcut>().unwrap();
//     let gs = app_handle.global_shortcut();
//     gs.trigger(shortcut, ShortcutState::Pressed);
//     gs.trigger(shortcut, ShortcutState::Released);
//     // 5. expect only one hit
//     assert!(rx.recv_timeout(Duration::from_millis(20)).is_ok());
//     assert!(rx.recv_timeout(Duration::from_millis(20)).is_err(),
//             "Released state emitted a second event");
// }

#[test]
fn test_hotkey_config_creation() {
    let config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+T".to_string(),
        enabled: true,
    };
    assert_eq!(config.shortcut, "CmdOrCtrl+Alt+T");
    assert!(config.enabled);
}

#[test]
fn test_hotkey_config_disabled() {
    let config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+T".to_string(),
        enabled: false,
    };
    assert_eq!(config.shortcut, "CmdOrCtrl+Alt+T");
    assert!(!config.enabled);
}

#[test]
fn test_hotkey_config_serialization() {
    let config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Shift+H".to_string(),
        enabled: true,
    };
    let json = serde_json::to_string(&config).expect("Failed to serialize");
    let deserialized: HotkeyConfig = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.shortcut, config.shortcut);
    assert_eq!(deserialized.enabled, config.enabled);
}

#[test]
fn test_hotkey_config_default() {
    let config = HotkeyConfig::default();
    assert_eq!(config.shortcut, "CmdOrCtrl+Alt+F1");
    assert!(config.enabled);
}

// ------------------------
// Property-based style
// ------------------------

#[test]
fn test_hotkey_config_various_formats() {
    let valid_hotkeys = vec![
        "CmdOrCtrl+Alt+A",
        "CmdOrCtrl+Shift+B",
        "CmdOrCtrl+Alt+Shift+C",
        "CmdOrCtrl+F1",
        "CmdOrCtrl+F12",
        "Alt+Space",
        "Shift+Escape",
    ];
    for hotkey in valid_hotkeys {
        let config = HotkeyConfig {
            shortcut: hotkey.to_string(),
            enabled: true,
        };
        assert_eq!(config.shortcut, hotkey);
        let json = serde_json::to_string(&config).expect("serialize");
        let _: HotkeyConfig = serde_json::from_str(&json).expect("deserialize");
    }
}

#[test]
fn test_hotkey_config_edge_cases() {
    let edge_cases = vec![
        ("", false),                       // Empty string
        ("Space", true),                   // Single key, no modifier
        ("CmdOrCtrl+Alt+CmdOrCtrl", true), // Duplicate modifiers
    ];
    for (hotkey, should_work) in edge_cases {
        let config = HotkeyConfig {
            shortcut: hotkey.to_string(),
            enabled: true,
        };
        if should_work {
            let json = serde_json::to_string(&config).expect("serialize");
            let _: HotkeyConfig = serde_json::from_str(&json).expect("deserialize");
        }
    }
}

#[test]
fn test_hotkey_config_with_special_characters() {
    let config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+\"".to_string(),
        enabled: true,
    };
    let json = serde_json::to_string(&config).expect("serialize");
    let deserialized: HotkeyConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(deserialized.shortcut, config.shortcut);
}

// ------------------------
// Documentation-style examples
// ------------------------

#[test]
fn test_hotkey_config_documentation_examples() {
    let basic_config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+Space".to_string(),
        enabled: true,
    };
    assert!(basic_config.enabled);

    let disabled_config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+Space".to_string(),
        enabled: false,
    };
    assert!(!disabled_config.enabled);

    let function_key_config = HotkeyConfig {
        shortcut: "CmdOrCtrl+F2".to_string(),
        enabled: true,
    };
    assert_eq!(function_key_config.shortcut, "CmdOrCtrl+F2");
}

// ------------------------------------------------------------------
// Integration function existence (stub) – kept for future completion
// ------------------------------------------------------------------

#[tokio::test]
async fn test_update_global_hotkey_internal_function_exists() {
    use speakr_lib::services::hotkey::update_global_hotkey_internal;
    let config = HotkeyConfig {
        shortcut: "CmdOrCtrl+Alt+Test".to_string(),
        enabled: true,
    };
    let _ = config; // silence unused variable warning
    let _function_exists = update_global_hotkey_internal;
}

// -----------------------------------------------------------------------------
// Settings-related tests (documentation-style, not full integration tests)
// These validate the relationship between AppSettings and HotkeyConfig and
// document currently known issues highlighted in FR-1 review.
// -----------------------------------------------------------------------------

#[test]
fn test_app_settings_structure_supports_hotkey_configuration() {
    // Arrange
    let settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+TestKey".to_string(),
        model_size: "medium".to_string(),
        auto_launch: false,
        audio_duration_secs: 10,
    };

    // Assert
    assert_eq!(settings.hot_key, "CmdOrCtrl+Alt+TestKey");
    // TODO: Add `hotkey_enabled` field to AppSettings and update tests once implemented.
}

#[test]
fn test_hotkey_config_vs_app_settings_mismatch() {
    // Arrange
    let app_settings = AppSettings::default();
    let hotkey_config = HotkeyConfig {
        shortcut: app_settings.hot_key.clone(),
        enabled: true, // Always true - highlights mismatch with user preference
    };

    // Assert
    assert_eq!(hotkey_config.shortcut, app_settings.hot_key);
    assert!(hotkey_config.enabled);
    // TODO: Ensure HotkeyConfig respects user preference when `hotkey_enabled` is introduced.
}

#[test]
fn test_default_app_settings_have_expected_hotkey() {
    // Arrange
    let default_settings = AppSettings::default();

    // Assert
    assert_eq!(default_settings.hot_key, "CmdOrCtrl+Alt+F1");
    // TODO: Persist chosen default on first run to avoid repeated fallback.
}

#[test]
fn test_hotkey_service_lifetime_problem_documentation() {
    // This test documents the known service lifetime issue described in the FR-1 review.
    // register_global_hotkey_internal creates a fresh GlobalHotkeyService each time,
    // causing unregister_global_hotkey_internal to fail because the state is lost.
    assert!(true);
}
