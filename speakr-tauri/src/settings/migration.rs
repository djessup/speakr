// ============================================================================
//! Settings Migration Logic
// ============================================================================

use speakr_types::{AppSettings, DEFAULT_AUDIO_DURATION_SECS, DEFAULT_SCHEMA_VERSION};
use tracing::{info, warn};

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
/// - Version 0 or 1 → 2: Adds `audio_duration_secs` field, otherwise no structural change
/// - Future versions will be handled incrementally
/// - Invalid versions are reset to defaults with warning
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn migrate_settings(mut settings: AppSettings) -> AppSettings {
    match settings.version {
        v if v < DEFAULT_SCHEMA_VERSION => {
            // Migrate from any version prior to 2 – add audio_duration_secs
            info!("Migrating settings from version {v} to 2");
            settings.audio_duration_secs = DEFAULT_AUDIO_DURATION_SECS;
            settings.version = 2;
        }
        DEFAULT_SCHEMA_VERSION => {
            // Current version - no migration needed
        }
        v if v > DEFAULT_SCHEMA_VERSION => {
            // Future version - log warning but don't modify
            warn!(
                "Warning: Settings file has newer version {v} than supported ({DEFAULT_SCHEMA_VERSION}). Using as-is."
            );
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

    match settings.validate() {
        Ok(_) => settings,
        Err(e) => {
            warn!("Invalid settings after migration: {}", e);
            AppSettings::default()
        }
    }
}
