//! Internal implementations for settings Tauri commands.

use crate::settings::persistence::{
    get_settings_path, load_settings_from_dir, save_settings_to_dir,
};
use speakr_types::{AppError, AppSettings};

/// Internal implementation for saving settings.
///
/// # Arguments
///
/// * `settings` - The settings to save
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
pub async fn save_settings_internal(settings: AppSettings) -> Result<(), AppError> {
    // Use the global settings directory for production
    let settings_path = get_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .ok_or_else(|| AppError::Settings("Invalid settings path".to_string()))?
        .to_path_buf();

    save_settings_to_dir(&settings, &settings_dir).await
}

/// Internal implementation for loading settings.
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
pub async fn load_settings_internal() -> Result<AppSettings, AppError> {
    // Use the global settings directory for production
    let settings_path = get_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .ok_or_else(|| AppError::Settings("Invalid settings path".to_string()))?
        .to_path_buf();

    load_settings_from_dir(&settings_dir).await
}
