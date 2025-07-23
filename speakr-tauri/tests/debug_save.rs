//! Debug test for save functionality
use speakr_lib::settings::save_settings_to_dir;
use speakr_types::AppSettings;
use tempfile::TempDir;
use tracing::debug;

/// Test helper to save settings in an isolated manner
///
/// This function demonstrates the proper test isolation pattern:
/// - Uses temporary directory for complete isolation
/// - No contamination of user data
/// - Automatic cleanup when TempDir goes out of scope
#[tokio::test]
async fn test_isolated_settings_save() {
    // Create isolated test environment
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    // Create test settings
    let test_settings = AppSettings::default();

    // Use the isolated function
    debug!("🔧 Debug: Testing save_settings_to_dir with proper isolation...");

    // This function call is completely isolated from production user data
    match save_settings_to_dir(&test_settings, &settings_dir).await {
        Ok(_) => {
            debug!("✅ Debug: Settings saved successfully to test directory");

            // Verify file was created
            let settings_file = settings_dir.join("settings.json");
            assert!(
                settings_file.exists(),
                "Settings file should exist in test directory"
            );
        }
        Err(e) => {
            debug!("❌ Debug: Failed to save settings: {:?}", e);
            panic!("Settings save should not fail in test environment");
        }
    }

    // Automatic cleanup happens when temp_dir goes out of scope
    debug!("🧹 Debug: Test cleanup will happen automatically");
}
