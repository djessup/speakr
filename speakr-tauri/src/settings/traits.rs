// ============================================================================
//! Settings Traits for Dependency Injection and Testing
//!
//! This module provides trait abstractions for settings loading to enable
//! dependency injection and isolated testing without global state.
// ============================================================================

use async_trait::async_trait;
use speakr_types::{AppError, AppSettings};
use std::path::PathBuf;

/// Trait for loading application settings
///
/// This trait enables dependency injection for settings loading,
/// allowing tests to use isolated temporary directories while
/// production code uses the global settings directory.
#[async_trait]
pub trait SettingsLoader: Send + Sync {
    /// Load application settings
    ///
    /// # Returns
    ///
    /// Returns the loaded settings or default settings if the file doesn't exist.
    /// If the file is corrupt, attempts to recover from backup, then falls back to defaults.
    ///
    /// # Errors
    ///
    /// Returns `AppError` if all recovery attempts fail.
    async fn load_settings(&self) -> Result<AppSettings, AppError>;
}

/// Production settings loader that uses the global settings directory
pub struct GlobalSettingsLoader;

#[async_trait]
impl SettingsLoader for GlobalSettingsLoader {
    async fn load_settings(&self) -> Result<AppSettings, AppError> {
        crate::settings::commands::load_settings_internal().await
    }
}

/// Test settings loader that uses an isolated directory
pub struct IsolatedSettingsLoader {
    settings_dir: PathBuf,
}

impl IsolatedSettingsLoader {
    /// Create a new isolated settings loader
    ///
    /// # Arguments
    ///
    /// * `settings_dir` - The directory to load settings from
    pub fn new(settings_dir: PathBuf) -> Self {
        Self { settings_dir }
    }
}

#[async_trait]
impl SettingsLoader for IsolatedSettingsLoader {
    async fn load_settings(&self) -> Result<AppSettings, AppError> {
        crate::settings::persistence::load_settings_from_dir(&self.settings_dir).await
    }
}

pub mod test_utils {
    use super::*;
    use mockall::mock;

    mock! {
        pub SettingsLoader {}

        #[async_trait]
        impl SettingsLoader for SettingsLoader {
            async fn load_settings(&self) -> Result<AppSettings, AppError>;
        }
    }
}
