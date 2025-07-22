//! Debug test for save functionality
use speakr_types::AppSettings;
use tempfile::TempDir;

#[tokio::test]
async fn test_debug_save_functionality() {
    println!("🔧 Debug: Testing save functionality...");

    // Create a test directory
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let settings_dir = temp_dir.path().to_path_buf();

    println!("📁 Test directory: {settings_dir:?}");

    // Create test settings
    let test_settings = AppSettings {
        version: 1,
        hot_key: "CmdOrCtrl+Alt+T".to_string(),
        model_size: "small".to_string(),
        auto_launch: true,
    };

    println!("⚙️  Test settings: {test_settings:?}");

    // Use the function from the library with correct name from Cargo.toml
    match speakr_lib::save_settings_to_dir(&test_settings, &settings_dir).await {
        Ok(()) => {
            println!("✅ Save succeeded!");

            // Check if file was actually created
            let settings_path = settings_dir.join("settings.json");
            if settings_path.exists() {
                println!("📄 File created at: {settings_path:?}");

                // Try to read and display content
                match std::fs::read_to_string(&settings_path) {
                    Ok(content) => println!("📖 File content:\n{content}"),
                    Err(e) => println!("❌ Failed to read file: {e}"),
                }
            } else {
                println!("❌ File was not created");
            }
        }
        Err(e) => {
            println!("❌ Save failed: {e}");
        }
    }
}
