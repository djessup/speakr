//! Settings persistence and file I/O operations.

use crate::settings::{
    migration::migrate_settings, validation::validate_settings_directory_permissions,
};
use speakr_types::{AppError, AppSettings};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

/// Gets the settings file path in the app data directory.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn get_settings_path() -> Result<PathBuf, AppError> {
    let app_data = dirs::config_dir()
        .ok_or_else(|| AppError::Settings("Could not find config directory".to_string()))?;

    let speakr_dir = app_data.join("speakr");
    if !speakr_dir.exists() {
        fs::create_dir_all(&speakr_dir)
            .map_err(|e| AppError::FileSystem(format!("Failed to create config dir: {e}")))?;
    }

    // Validate directory permissions after creation
    validate_settings_directory_permissions(&speakr_dir)?;

    Ok(speakr_dir.join("settings.json"))
}

/// Gets the backup settings file path for corruption recovery.
///
/// # Internal API
/// This function is only intended for internal use and testing.
#[allow(dead_code)] // Used in tests
pub fn get_settings_backup_path() -> Result<PathBuf, AppError> {
    let settings_path = get_settings_path()?;
    Ok(settings_path.with_extension("json.backup"))
}

/// Attempts to load settings from a specific file path.
///
/// # Arguments
///
/// * `path` - The file path to load from
///
/// # Returns
///
/// Returns the loaded settings.
///
/// # Errors
///
/// Returns error string if the file cannot be read or parsed.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn try_load_settings_file(path: &PathBuf) -> Result<AppSettings, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read settings file: {e}"))?;

    let settings: AppSettings = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse settings JSON: {e}"))?;

    Ok(settings)
}

/// Saves application settings to a specific directory (for testing and isolation).
///
/// # Arguments
///
/// * `settings` - The settings to save
/// * `settings_dir` - The directory where settings should be saved
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the settings cannot be saved.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub async fn save_settings_to_dir(
    settings: &AppSettings,
    settings_dir: &PathBuf,
) -> Result<(), AppError> {
    // Ensure directory exists
    if !settings_dir.exists() {
        fs::create_dir_all(settings_dir)
            .map_err(|e| AppError::FileSystem(format!("Failed to create settings dir: {e}")))?;
    }

    let settings_path = settings_dir.join("settings.json");
    let backup_path = settings_dir.join("settings.json.backup");

    // Ensure settings have current version
    let mut settings_to_save = settings.clone();
    settings_to_save.version = 1;

    let json = serde_json::to_string_pretty(&settings_to_save)
        .map_err(|e| AppError::Settings(format!("Failed to serialize settings: {e}")))?;

    // Atomic write: write to temporary file first, then rename
    let temp_path = settings_path.with_extension("json.tmp");

    // Write to temporary file
    fs::write(&temp_path, &json)
        .map_err(|e| AppError::FileSystem(format!("Failed to write temp settings file: {e}")))?;

    // Create backup of existing file if it exists
    if settings_path.exists() {
        fs::copy(&settings_path, &backup_path)
            .map_err(|e| AppError::FileSystem(format!("Failed to create settings backup: {e}")))?;
    }

    // Atomically move temp file to final location
    fs::rename(&temp_path, &settings_path)
        .map_err(|e| AppError::FileSystem(format!("Failed to move temp settings file: {e}")))?;

    Ok(())
}

/// Loads application settings from a specific directory with corruption recovery.
///
/// # Arguments
///
/// * `settings_dir` - The directory to load settings from
///
/// # Returns
///
/// Returns the loaded settings or default settings if the file doesn't exist.
/// If the file is corrupt, attempts to recover from backup, then falls back to defaults.
///
/// # Errors
///
/// Returns `AppError` if all recovery attempts fail.
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub async fn load_settings_from_dir(settings_dir: &PathBuf) -> Result<AppSettings, AppError> {
    let settings_path = settings_dir.join("settings.json");
    let backup_path = settings_dir.join("settings.json.backup");

    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    // Try to load from main settings file
    match try_load_settings_file(&settings_path) {
        Ok(settings) => {
            let original_version = settings.version;
            let migrated_settings = migrate_settings(settings);
            // If migration changed the version, save the updated settings
            if migrated_settings.version != original_version {
                // Save the migrated settings (fire and forget)
                let _ = save_settings_to_dir(&migrated_settings, settings_dir).await;
            }
            Ok(migrated_settings)
        }
        Err(main_error) => {
            error!("Warning: Main settings file corrupt: {main_error}");

            // Try to recover from backup
            if backup_path.exists() {
                match try_load_settings_file(&backup_path) {
                    Ok(backup_settings) => {
                        info!("Successfully recovered settings from backup");
                        let migrated_settings = migrate_settings(backup_settings);

                        // Save the recovered settings to main file
                        if let Err(save_error) =
                            save_settings_to_dir(&migrated_settings, settings_dir).await
                        {
                            error!("Warning: Failed to save recovered settings: {save_error}");
                        }

                        Ok(migrated_settings)
                    }
                    Err(backup_error) => {
                        error!("Warning: Backup settings file also corrupt: {backup_error}");

                        // Move corrupt files aside for debugging
                        let _ = fs::rename(
                            &settings_path,
                            settings_path.with_extension("json.corrupt"),
                        );
                        let _ =
                            fs::rename(&backup_path, backup_path.with_extension("json.corrupt"));

                        // Return defaults and save them
                        let defaults = AppSettings::default();
                        if let Err(save_error) = save_settings_to_dir(&defaults, settings_dir).await
                        {
                            error!("Warning: Failed to save default settings: {save_error}");
                        }

                        Ok(defaults)
                    }
                }
            } else {
                info!("No backup file available. Using defaults.");

                // Move corrupt file aside and save defaults
                let _ = fs::rename(&settings_path, settings_path.with_extension("json.corrupt"));
                let defaults = AppSettings::default();
                if let Err(save_error) = save_settings_to_dir(&defaults, settings_dir).await {
                    error!("Warning: Failed to save default settings: {save_error}");
                }

                Ok(defaults)
            }
        }
    }
}
