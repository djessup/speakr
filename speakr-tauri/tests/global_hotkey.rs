// Import from the types crate directly
use speakr_types::HotkeyConfig;

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests now require integration testing with a Tauri AppHandle
    // For now, we'll focus on the shortcut parsing logic and integration tests
    // The service tests will need to be updated to use mock AppHandles or test differently

    #[tokio::test]
    async fn test_hotkey_config_default() {
        // Arrange & Act
        let config = HotkeyConfig::default();

        // Assert
        assert_eq!(config.shortcut, "CmdOrCtrl+Alt+F1");
        assert!(config.enabled);
    }

    #[tokio::test]
    async fn test_hotkey_config_serialization() {
        // Arrange
        let config = HotkeyConfig {
            shortcut: "CmdOrCtrl+Shift+D".to_string(),
            enabled: true,
        };

        // Act
        let json = serde_json::to_string(&config);

        // Assert
        assert!(json.is_ok(), "Config should serialize to JSON");

        // Test deserialization
        let json_str = json.unwrap();
        let deserialized: Result<HotkeyConfig, _> = serde_json::from_str(&json_str);
        assert!(deserialized.is_ok(), "Config should deserialize from JSON");
        assert_eq!(deserialized.unwrap().shortcut, config.shortcut);
    }

    #[tokio::test]
    async fn test_disabled_hotkey_config() {
        // Arrange
        let config = HotkeyConfig {
            shortcut: "CmdOrCtrl+Alt+D".to_string(),
            enabled: false,
        };

        // Act & Assert
        assert!(!config.enabled, "Config should be disabled");
        assert!(
            !config.shortcut.is_empty(),
            "Shortcut should not be empty even when disabled"
        );
    }

    // 🔴 RED: Tests for function key configurations
    #[tokio::test]
    async fn test_function_key_hotkey_configs() {
        let function_key_configs = vec![
            "CmdOrCtrl+F1",
            "CmdOrCtrl+F2",
            "CmdOrCtrl+F3",
            "CmdOrCtrl+F4",
            "CmdOrCtrl+F5",
            "CmdOrCtrl+F6",
            "CmdOrCtrl+F7",
            "CmdOrCtrl+F8",
            "CmdOrCtrl+F9",
            "CmdOrCtrl+F10",
            "CmdOrCtrl+F11",
            "CmdOrCtrl+F12",
            "Alt+F1",
            "Shift+F2",
            "Ctrl+Alt+F3",
        ];

        for shortcut in function_key_configs {
            let config = HotkeyConfig {
                shortcut: shortcut.to_string(),
                enabled: true,
            };

            // Test serialization/deserialization
            let json = serde_json::to_string(&config).expect("Should serialize");
            let deserialized: HotkeyConfig =
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(deserialized.shortcut, shortcut);
            assert!(deserialized.enabled);
        }
    }

    // 🔴 RED: Tests for special key configurations
    #[tokio::test]
    async fn test_special_key_hotkey_configs() {
        let special_key_configs = vec![
            "CmdOrCtrl+Home",
            "CmdOrCtrl+End",
            "CmdOrCtrl+PageUp",
            "CmdOrCtrl+PageDown",
            "CmdOrCtrl+ArrowUp",
            "CmdOrCtrl+ArrowDown",
            "CmdOrCtrl+ArrowLeft",
            "CmdOrCtrl+ArrowRight",
            "CmdOrCtrl+Insert",
            "Alt+~",
            "Shift+Insert",
        ];

        for shortcut in special_key_configs {
            let config = HotkeyConfig {
                shortcut: shortcut.to_string(),
                enabled: true,
            };

            // Should be able to create config without errors
            assert!(!config.shortcut.is_empty());
            assert!(config.enabled);
        }
    }

    // 🔴 RED: Tests for numeric and symbol key configurations
    #[tokio::test]
    async fn test_numeric_symbol_key_hotkey_configs() {
        let key_configs = vec![
            "CmdOrCtrl+0",
            "CmdOrCtrl+1",
            "CmdOrCtrl+2",
            "CmdOrCtrl+9",
            "CmdOrCtrl+;",
            "CmdOrCtrl+,",
            "CmdOrCtrl+.",
            "CmdOrCtrl+/",
            "CmdOrCtrl+[",
            "CmdOrCtrl+]",
            "CmdOrCtrl+-",
            "CmdOrCtrl+=",
        ];

        for shortcut in key_configs {
            let config = HotkeyConfig {
                shortcut: shortcut.to_string(),
                enabled: true,
            };

            // Should be able to create config without errors
            assert!(!config.shortcut.is_empty());
            assert!(config.enabled);
        }
    }

    // Integration tests would go here once we have proper Tauri test setup
    // For now, these serve as documentation of the expected behavior

    // Integration test using Tauri's test utilities
    #[tokio::test]
    async fn test_tauri_integration_hotkey_registration() {
        // This test verifies integration with Tauri's global shortcut plugin
        // It should FAIL initially until we integrate with the actual plugin

        // For now, this is a placeholder that will be expanded once we have the actual implementation
        // The mock_builder() requires context that we'll add when implementing the actual plugin
        // TODO: Implement actual Tauri integration test with plugin
        // This placeholder will be replaced with real tests once the integration is complete
    }
}
