// Import from the main library
use speakr_lib::HotkeyConfig;

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
        assert_eq!(config.shortcut, "CmdOrCtrl+Alt+Space");
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
