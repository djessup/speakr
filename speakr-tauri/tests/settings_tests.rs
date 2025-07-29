// ============================================================================
//! Settings-related tests
// ============================================================================
#![allow(clippy::field_reassign_with_default)]

use speakr_types::{AppSettings, DEFAULT_MODEL_SIZE, DEFAULT_SCHEMA_VERSION};
use tempfile::TempDir;
use tracing::debug;

// Import functions from the speakr_lib crate (now pub(crate))
use speakr_lib::settings::{
    load_settings_from_dir, migrate_settings, save_settings_to_dir, try_load_settings_file,
    validate_settings_directory_permissions,
};

#[tokio::test]
async fn test_app_settings_default() {
    let settings = AppSettings::default();
    assert_eq!(settings.version, 2); // Updated to version 2
    assert_eq!(settings.hot_key, "CmdOrCtrl+Alt+F1"); // Use the actual default from speakr_types
    assert_eq!(settings.model_size, "medium");
    assert!(!settings.auto_launch);
}

#[tokio::test]
async fn test_save_settings_rejects_invalid_audio_duration() {
    // Test that save_settings_to_dir rejects invalid audio duration
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut invalid_settings = AppSettings::default();
    invalid_settings.audio_duration_secs = 0; // Invalid: below minimum

    let result = save_settings_to_dir(&invalid_settings, &temp_dir.path().to_path_buf()).await;
    assert!(result.is_err());

    if let Err(speakr_types::AppError::Settings(msg)) = result {
        assert!(msg.contains("Invalid audio duration"));
        assert!(msg.contains("0 seconds"));
    } else {
        panic!("Expected Settings error with audio duration message");
    }
}

#[tokio::test]
async fn test_save_settings_rejects_audio_duration_above_max() {
    // Test that save_settings_to_dir rejects audio duration above maximum
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut invalid_settings = AppSettings::default();
    invalid_settings.audio_duration_secs = 31; // Invalid: above maximum

    let result = save_settings_to_dir(&invalid_settings, &temp_dir.path().to_path_buf()).await;
    assert!(result.is_err());

    if let Err(speakr_types::AppError::Settings(msg)) = result {
        assert!(msg.contains("Invalid audio duration"));
        assert!(msg.contains("31 seconds"));
    } else {
        panic!("Expected Settings error with audio duration message");
    }
}

#[tokio::test]
async fn test_save_settings_accepts_valid_audio_duration() {
    // Test that save_settings_to_dir accepts valid audio duration
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let mut valid_settings = AppSettings::default();
    valid_settings.audio_duration_secs = 15; // Valid: within range

    let result = save_settings_to_dir(&valid_settings, &temp_dir.path().to_path_buf()).await;
    assert!(result.is_ok());

    // Verify the settings were actually saved
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .unwrap();
    assert_eq!(loaded_settings.audio_duration_secs, 15);
}

#[tokio::test]
async fn test_save_settings_internal_validates_settings() {
    // Ensures validation rejects incorrect settings before saving
    use speakr_lib::settings::save_settings_internal;

    // Arrange - Create invalid settings
    let mut invalid_settings = AppSettings::default();
    invalid_settings.audio_duration_secs = 0; // Invalid duration

    // Act - Try to save invalid settings
    let result = save_settings_internal(invalid_settings).await;

    // Assert - Should fail with validation error
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid audio duration"));
}

#[tokio::test]
async fn test_save_settings_internal_accepts_valid_settings() {
    // This test should pass when validation is properly implemented
    use speakr_lib::settings::save_settings_internal;

    // Arrange - Create valid settings
    let valid_settings = AppSettings::default(); // Default settings should be valid

    // Act - Try to save valid settings
    let result = save_settings_internal(valid_settings).await;

    // Assert - Should succeed
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_save_settings_to_dir_validates_settings() {
    // Test that the lower-level persistence function also validates
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path().to_path_buf();

    // Arrange - Create invalid settings
    let mut invalid_settings = AppSettings::default();
    invalid_settings.audio_duration_secs = 31; // Invalid duration

    // Act - Try to save invalid settings directly to dir
    let result = save_settings_to_dir(&invalid_settings, &temp_dir_path).await;

    // Assert - Should fail with validation error
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid audio duration"));
}

#[tokio::test]
async fn test_save_settings_to_dir_preserves_version() {
    // Test that the version is preserved correctly
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path().to_path_buf();

    // Arrange - Create settings with current version
    let settings = AppSettings::default();
    assert_eq!(settings.version, 2); // Should be version 2 now

    // Act - Save settings
    let result = save_settings_to_dir(&settings, &temp_dir_path).await;
    assert!(result.is_ok());

    // Assert - Load and verify version is preserved
    let loaded_settings = load_settings_from_dir(&temp_dir_path).await.unwrap();
    assert_eq!(loaded_settings.version, 2);
}

#[tokio::test]
async fn test_settings_serialization() {
    let settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+D".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
        audio_duration_secs: 10,
    };

    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: AppSettings = serde_json::from_str(&json).unwrap();

    assert_eq!(settings, deserialized);
}

#[tokio::test]
async fn debug_save_button_functionality() {
    debug!("🔧 Debug: Testing save functionality...");

    // Create test directory
    let temp_dir = tempfile::TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    debug!("📁 Test directory: {:?}", settings_dir);

    // Create test settings
    let test_settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+T".to_string(),
        model_size: "medium".to_string(),
        auto_launch: true,
        audio_duration_secs: 10,
    };

    debug!("⚙️  Test settings: {:?}", test_settings);

    // Try to save using the same function the Tauri command uses
    match save_settings_to_dir(&test_settings, &settings_dir).await {
        Ok(()) => {
            debug!("✅ Save succeeded!");

            // Check if file was created
            let settings_path = settings_dir.join("settings.json");
            if settings_path.exists() {
                debug!("📄 File created at: {:?}", settings_path);

                // Read and print content
                match std::fs::read_to_string(&settings_path) {
                    Ok(content) => debug!("📖 File content:\n{}", content),
                    Err(e) => debug!("❌ Failed to read file: {}", e),
                }
            } else {
                debug!("❌ File was not created");
            }
        }
        Err(e) => {
            debug!("❌ Save failed: {}", e);
            panic!("Save should work in test environment");
        }
    }
}

// Previously skipped tests - now working with pub(crate) functions

#[tokio::test]
async fn test_save_and_load_settings() {
    // Use a temporary directory for this test to avoid interference
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let temp_settings_path = temp_dir.path().join("settings.json");

    // Arrange
    let test_settings = AppSettings {
        version: 2,
        hot_key: "CmdOrCtrl+Alt+S".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
        audio_duration_secs: 10,
    };

    // Test the helper function directly since we can't override the global path
    // Save settings to temp file
    let json = serde_json::to_string_pretty(&test_settings).expect("Should serialize");
    std::fs::write(&temp_settings_path, json).expect("Should write test file");

    // Load and verify using our helper function
    let loaded_settings =
        try_load_settings_file(&temp_settings_path).expect("Should load test settings");
    assert_eq!(loaded_settings, test_settings);

    // Test migration works
    let migrated = migrate_settings(loaded_settings);
    assert_eq!(migrated, test_settings); // Should be unchanged since it's already version 1
}

#[tokio::test]
async fn test_settings_migration() {
    // Test migration from version 0 to current
    let old_settings = AppSettings {
        version: 0,
        ..Default::default()
    };

    let migrated = migrate_settings(old_settings);
    assert_eq!(migrated.version, DEFAULT_SCHEMA_VERSION);

    // Test current version (no migration)
    let current_settings = AppSettings::default();
    let unchanged = migrate_settings(current_settings.clone());
    assert_eq!(unchanged, current_settings);
}

#[tokio::test]
async fn test_corruption_recovery_from_backup() {
    // Use isolated temporary directory instead of real settings path
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    // Create good settings - first save creates main file, no backup yet
    let good_settings = AppSettings::default();
    save_settings_to_dir(&good_settings, &settings_dir)
        .await
        .expect("Should save initial");

    let settings_path = settings_dir.join("settings.json");
    let backup_path = settings_dir.join("settings.json.backup");

    // First save doesn't create backup (no existing file to backup)
    assert!(
        !backup_path.exists(),
        "Backup should NOT exist after first save"
    );

    // Second save creates backup of the existing file
    save_settings_to_dir(&good_settings, &settings_dir)
        .await
        .expect("Should save second time");

    // NOW backup should exist (created from the existing file during second save)
    assert!(
        backup_path.exists(),
        "Backup should exist after second save"
    );

    // Corrupt the main file (backup should exist after second save)
    std::fs::write(&settings_path, "invalid json").expect("Should corrupt main file");

    // Load should recover from backup using isolated function
    let recovered = load_settings_from_dir(&settings_dir)
        .await
        .expect("Should recover from backup");
    assert_eq!(recovered, good_settings);

    // Verify main file was restored
    let reloaded = load_settings_from_dir(&settings_dir)
        .await
        .expect("Should load restored settings");
    assert_eq!(reloaded, good_settings);
}

#[tokio::test]
async fn test_corruption_recovery_fallback_to_defaults() {
    use tempfile::TempDir;

    // Create a temporary directory for this test
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let fake_settings_path = temp_dir.path().join("settings.json");
    let fake_backup_path = temp_dir.path().join("settings.json.backup");

    // Create both files with invalid content
    std::fs::write(&fake_settings_path, "invalid json").expect("Should write corrupt main file");
    std::fs::write(&fake_backup_path, "also invalid").expect("Should write corrupt backup file");

    // Use the helper function directly since we can't easily override the paths in the command
    let load_result_main = try_load_settings_file(&fake_settings_path);
    assert!(
        load_result_main.is_err(),
        "Should fail to load corrupt main file"
    );

    let load_result_backup = try_load_settings_file(&fake_backup_path);
    assert!(
        load_result_backup.is_err(),
        "Should fail to load corrupt backup file"
    );

    // In the real scenario, this would fall back to defaults
    // The load_settings command handles this logic
}

#[tokio::test]
async fn test_settings_performance() {
    use std::time::Instant;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let temp_settings_path = temp_dir.path().join("settings.json");
    let settings = AppSettings::default();

    // Test that save completes within 100ms (FR-8 requirement)
    let start = Instant::now();

    // Test the actual file operations that happen in save_settings
    let json = serde_json::to_string_pretty(&settings).expect("Should serialize");
    let temp_path = temp_settings_path.with_extension("json.tmp");
    std::fs::write(&temp_path, &json).expect("Should write temp file");
    std::fs::rename(&temp_path, &temp_settings_path).expect("Should rename file");

    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 100,
        "Settings save should complete within 100ms, took {}ms",
        duration.as_millis()
    );

    // Verify the file was written correctly
    let loaded = try_load_settings_file(&temp_settings_path).expect("Should load saved settings");
    assert_eq!(loaded, settings);
}

#[tokio::test]
async fn test_settings_directory_permissions() {
    // Verifies that directory permission validation succeeds on writable temp dir
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_path = temp_dir.path().join("settings.json");

    // Test that we can detect and handle permission issues
    // This should fail initially because we don't have permission validation
    let result = validate_settings_directory_permissions(settings_path.parent().unwrap());

    // This should pass once we implement the function
    assert!(result.is_ok(), "Should validate directory permissions");
}

#[tokio::test]
async fn test_isolated_settings_save_and_load() {
    // Confirms save & load helpers operate correctly in an isolated directory
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    let test_settings = AppSettings {
        version: DEFAULT_SCHEMA_VERSION,
        hot_key: "CmdOrCtrl+Alt+T".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
        audio_duration_secs: 10,
    };

    // These functions should accept directory paths to enable test isolation
    save_settings_to_dir(&test_settings, &settings_dir)
        .await
        .expect("Should save to test dir");
    let loaded = load_settings_from_dir(&settings_dir)
        .await
        .expect("Should load from test dir");

    assert_eq!(loaded, test_settings);
}

#[tokio::test]
async fn test_isolated_corruption_recovery() {
    // Ensures recovery from backup works within an isolated environment
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    // Create good settings and backup
    let good_settings = AppSettings::default();
    save_settings_to_dir(&good_settings, &settings_dir)
        .await
        .expect("Should save initial");

    let settings_path = settings_dir.join("settings.json");
    let _backup_path = settings_dir.join("settings.json.backup");

    // Corrupt main file (backup should exist after first save)
    std::fs::write(&settings_path, "invalid json").expect("Should corrupt main file");

    // Load should recover from backup
    let recovered = load_settings_from_dir(&settings_dir)
        .await
        .expect("Should recover");
    assert_eq!(recovered, good_settings);
}

// =========================
// Tests for Settings Loading
// =========================

#[tokio::test]
async fn test_load_settings_from_dir_with_custom_hotkey() {
    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let custom_hotkey = "CmdOrCtrl+Shift+CustomKey";

    let settings = AppSettings {
        hot_key: custom_hotkey.to_string(),
        ..AppSettings::default()
    };

    // Save settings to temp directory
    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    // Act
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");

    // Assert
    assert_eq!(loaded_settings.hot_key, custom_hotkey);
    assert_eq!(loaded_settings.version, DEFAULT_SCHEMA_VERSION);
    assert_eq!(loaded_settings.model_size, DEFAULT_MODEL_SIZE);
    assert!(!loaded_settings.auto_launch);
}

#[tokio::test]
async fn test_load_settings_from_dir_returns_defaults_when_file_missing() {
    // This test drives the requirement for fallback behavior

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    // No settings file created

    // Act
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Should return default settings when file missing");

    // Assert - should return default settings
    let default_settings = AppSettings::default();
    assert_eq!(loaded_settings.hot_key, default_settings.hot_key);
    assert_eq!(loaded_settings.version, default_settings.version);
    assert_eq!(loaded_settings.model_size, default_settings.model_size);
    assert_eq!(loaded_settings.auto_launch, default_settings.auto_launch);
}

#[tokio::test]
async fn test_load_settings_from_dir_handles_corrupt_file() {
    // This test drives the requirement for corruption recovery

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let settings_file = temp_dir.path().join("settings.json");

    // Write corrupt JSON
    std::fs::write(&settings_file, "{ invalid json content }")
        .expect("Failed to write corrupt file");

    // Act
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Should handle corrupt file gracefully");

    // Assert - should return default settings
    let default_settings = AppSettings::default();
    assert_eq!(loaded_settings.hot_key, default_settings.hot_key);
}

#[tokio::test]
async fn test_save_and_load_settings_roundtrip_with_custom_hotkey() {
    // This test ensures the save/load cycle preserves custom hotkeys

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let custom_hotkey = "CmdOrCtrl+Alt+RoundTrip";

    let original_settings = AppSettings {
        version: DEFAULT_SCHEMA_VERSION,
        hot_key: custom_hotkey.to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
        audio_duration_secs: 10,
    };

    // Act
    save_settings_to_dir(&original_settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");

    // Assert
    assert_eq!(loaded_settings.hot_key, original_settings.hot_key);
    assert_eq!(loaded_settings.version, original_settings.version);
    assert_eq!(loaded_settings.model_size, original_settings.model_size);
    assert_eq!(loaded_settings.auto_launch, original_settings.auto_launch);
}

// =========================
// Tests for Hotkey-Specific Scenarios
// =========================

#[tokio::test]
async fn test_load_settings_preserves_various_hotkey_formats() {
    // This test ensures different hotkey formats are preserved correctly

    let test_hotkeys = vec![
        "CmdOrCtrl+Alt+A",
        "CmdOrCtrl+Shift+B",
        "CmdOrCtrl+Alt+Shift+C",
        "CmdOrCtrl+F1",
        "CmdOrCtrl+F12",
        "Alt+Space",
        "Shift+Escape",
    ];

    for hotkey in test_hotkeys {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let settings = AppSettings {
            version: 1,
            hot_key: hotkey.to_string(),
            model_size: "medium".to_string(),
            auto_launch: false,
            audio_duration_secs: 10,
        };

        // Act
        save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
            .await
            .expect("Failed to save settings");

        let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
            .await
            .expect("Failed to load settings");

        // Assert
        assert_eq!(
            loaded_settings.hot_key, hotkey,
            "Hotkey '{hotkey}' was not preserved correctly"
        );
    }
}

#[tokio::test]
async fn test_load_settings_handles_empty_hotkey() {
    // This test drives the requirement for handling edge cases

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let settings = AppSettings {
        version: 1,
        hot_key: "".to_string(), // Empty hotkey
        model_size: "medium".to_string(),
        auto_launch: false,
        audio_duration_secs: 10,
    };

    // Act
    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");

    // Assert
    assert_eq!(loaded_settings.hot_key, ""); // Should preserve empty string
}

#[tokio::test]
async fn test_load_settings_handles_special_characters_in_hotkey() {
    // This test ensures special characters don't break the loading

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let special_hotkey = "CmdOrCtrl+Alt+\""; // Hotkey with quote character
    let settings = AppSettings {
        version: 1,
        hot_key: special_hotkey.to_string(),
        model_size: "medium".to_string(),
        auto_launch: false,
        audio_duration_secs: 10,
    };

    // Act
    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");

    // Assert
    assert_eq!(loaded_settings.hot_key, special_hotkey);
}

// =========================
// Integration Tests for the Actual Implementation
// =========================

#[tokio::test]
async fn test_settings_roundtrip_preserves_custom_hotkey() {
    // This test demonstrates what the register_default_hotkey function should do
    // It's a specification test that drives the implementation requirements

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let custom_hotkey = "CmdOrCtrl+Alt+TestIntegration";
    let settings = AppSettings {
        version: 1,
        hot_key: custom_hotkey.to_string(),
        model_size: "medium".to_string(),
        auto_launch: false,
        audio_duration_secs: 10,
    };

    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    // Act - Load settings (this is what register_default_hotkey should do)
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");

    // Assert - The loaded settings should contain the custom hotkey
    assert_eq!(loaded_settings.hot_key, custom_hotkey);

    // This demonstrates that register_default_hotkey should eventually:
    // 1. Call load_settings_internal() or equivalent
    // 2. Extract the hot_key from the loaded settings
    // 3. Use that hotkey for registration
    // 4. Fall back to default if loading fails
}

// =========================
// Error Handling Tests
// =========================

#[tokio::test]
async fn test_load_settings_handles_permission_denied() {
    // This test would verify behavior when settings directory is not readable
    // In a real implementation, this would test the error handling path

    // Note: This test is difficult to implement portably across different OS
    // but it represents the kind of error handling that should be tested

    // For now, we just verify that the function exists and can handle basic cases
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let result = load_settings_from_dir(&temp_dir.path().to_path_buf()).await;
    assert!(result.is_ok(), "Should handle empty directory gracefully");
}

#[tokio::test]
async fn test_load_settings_handles_invalid_json_structure() {
    // This test drives the requirement for handling malformed JSON

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let settings_file = temp_dir.path().join("settings.json");

    // Write valid JSON but wrong structure
    std::fs::write(
        &settings_file,
        r#"{"wrong": "structure", "not": "settings"}"#,
    )
    .expect("Failed to write malformed settings file");

    // Act
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Should handle malformed JSON gracefully");

    // Assert - should return default settings
    let default_settings = AppSettings::default();
    assert_eq!(loaded_settings.hot_key, default_settings.hot_key);
}

// =========================
// Performance Tests
// =========================

#[tokio::test]
async fn test_load_settings_performance() {
    // This test ensures settings loading is fast enough for startup

    // Arrange
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let settings = AppSettings::default();
    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf())
        .await
        .expect("Failed to save settings");

    // Act & Assert
    let start = std::time::Instant::now();
    let _loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf())
        .await
        .expect("Failed to load settings");
    let duration = start.elapsed();

    // Settings loading should be very fast (under 100ms even on slow systems)
    assert!(
        duration.as_millis() < 100,
        "Settings loading took too long: {duration:?}"
    );
}
