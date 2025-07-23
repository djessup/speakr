//! Settings migration logic for handling version upgrades.

use speakr_types::AppSettings;
use tracing::warn;

/// Migrates settings from older versions to the current version.
///
/// # Arguments
///
/// * `settings` - The settings loaded from disk
///
/// # Returns
///
/// Returns the migrated settings with updated version number.
///
/// # Migration Strategy
///
/// - Version 0 → 1: No structural changes needed for current implementation
/// - Future versions will be handled incrementally
/// - Invalid versions are reset to defaults with warning
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn migrate_settings(mut settings: AppSettings) -> AppSettings {
    match settings.version {
        0 => {
            // Migrate from version 0 to 1 - no changes needed for now
            settings.version = 1;
        }
        1 => {
            // Current version - no migration needed
        }
        v if v > 1 => {
            // Future version - log warning but don't modify
            warn!("Warning: Settings file has newer version {v} than supported (1). Using as-is.");
        }
        _ => {
            // Invalid version - reset to defaults
            warn!(
                "Warning: Invalid settings version {}. Resetting to defaults.",
                settings.version
            );
            settings = AppSettings::default();
        }
    }
    settings
}
