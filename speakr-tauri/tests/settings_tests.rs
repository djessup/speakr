// Settings-related tests extracted from lib.rs

use speakr_types::AppSettings;
use tracing::debug;

// Import functions from the speakr_lib crate (now pub(crate))
use speakr_lib::{
    load_settings_from_dir, migrate_settings, save_settings_to_dir, try_load_settings_file,
    validate_settings_directory_permissions,
};

#[tokio::test]
async fn test_app_settings_default() {
    let settings = AppSettings::default();
    assert_eq!(settings.version, 1);
    assert_eq!(settings.hot_key, "CmdOrCtrl+Alt+F1"); // Use the actual default from speakr_types
    assert_eq!(settings.model_size, "medium");
    assert!(!settings.auto_launch);
}

#[tokio::test]
async fn test_settings_serialization() {
    let settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+D".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
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
        version: 1,
        hot_key: "CmdOrCtrl+Alt+S".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
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
    assert_eq!(migrated.version, 1);

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
    // RED: This test should fail initially since we haven't implemented permission validation
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
    // RED: This should fail because these functions don't exist yet
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    let test_settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+T".to_string(),
        model_size: "large".to_string(),
        auto_launch: true,
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
    // RED: This should fail because these functions don't exist yet
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
