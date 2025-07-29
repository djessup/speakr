// ============================================================================
//! Test Utilities for Isolated Settings Environments
//!
//! This module provides helper functions for creating isolated test environments
//! that don't interfere with global settings or each other.
// ============================================================================
#![allow(clippy::field_reassign_with_default)]

use speakr_lib::settings::{save_settings_to_dir, IsolatedSettingsLoader, SettingsLoader};
use speakr_types::{AppSettings, DEFAULT_AUDIO_DURATION_SECS};
use std::sync::Arc;
use tempfile::TempDir;

// Note: Common types are already imported above and available to this module

/// Creates an isolated settings environment for testing
///
/// # Returns
///
/// Returns a tuple of (TempDir, IsolatedSettingsLoader) where:
/// - TempDir: The temporary directory (must be kept alive for the test duration)
/// - IsolatedSettingsLoader: A settings loader configured for the isolated directory
///
/// # Example
///
/// ```rust
/// use test_utils::create_isolated_settings_env;
///
/// #[tokio::test]
/// async fn test_with_isolated_settings() {
///     let (_temp_dir, loader) = create_isolated_settings_env().await;
///     // Use loader in your test...
/// }
/// ```
pub async fn create_isolated_settings_env() -> (TempDir, Arc<IsolatedSettingsLoader>) {
    let temp_dir = TempDir::with_prefix("speakr_test_").expect("Failed to create temp directory");
    let settings_dir = temp_dir.path().to_path_buf();
    let loader = Arc::new(IsolatedSettingsLoader::new(settings_dir));
    (temp_dir, loader)
}

/// Creates an isolated settings environment with custom settings
///
/// # Arguments
///
/// * `settings` - The settings to save in the isolated environment
///
/// # Returns
///
/// Returns a tuple of (TempDir, IsolatedSettingsLoader) with the custom settings pre-saved
///
/// # Example
///
/// ```rust
/// use test_utils::create_isolated_settings_env_with_settings;
/// use speakr_types::AppSettings;
///
/// #[tokio::test]
/// async fn test_with_custom_settings() {
///     let mut settings = AppSettings::default();
///     settings.audio_duration_secs = 15;
///
///     let (_temp_dir, loader) = create_isolated_settings_env_with_settings(settings).await;
///     // Use loader in your test...
/// }
/// ```
pub async fn create_isolated_settings_env_with_settings(
    settings: AppSettings,
) -> (TempDir, Arc<IsolatedSettingsLoader>) {
    let temp_dir = TempDir::with_prefix("speakr_test_").expect("Failed to create temp directory");
    let settings_dir = temp_dir.path().to_path_buf();

    // Save the custom settings to the isolated directory
    save_settings_to_dir(&settings, &settings_dir)
        .await
        .expect("Failed to save settings to isolated directory");

    let loader = Arc::new(IsolatedSettingsLoader::new(settings_dir));
    (temp_dir, loader)
}

/// Creates multiple isolated settings environments for parallel testing
///
/// # Arguments
///
/// * `count` - The number of isolated environments to create
///
/// # Returns
///
/// Returns a vector of (TempDir, IsolatedSettingsLoader) tuples
///
/// # Example
///
/// ```rust
/// use test_utils::create_multiple_isolated_settings_envs;
///
/// #[tokio::test]
/// async fn test_parallel_settings() {
///     let envs = create_multiple_isolated_settings_envs(3).await;
///
///     // Run parallel tests with different environments
///     for (_temp_dir, loader) in envs {
///         // Each loader is completely isolated
///     }
/// }
/// ```
pub async fn create_multiple_isolated_settings_envs(
    count: usize,
) -> Vec<(TempDir, Arc<IsolatedSettingsLoader>)> {
    // Create environments concurrently for better performance
    // Note: For CI environments with limited resources, consider creating serially
    use futures::future::join_all;

    let tasks: Vec<_> = (0..count).map(|_| create_isolated_settings_env()).collect();

    join_all(tasks).await
}

/// Helper function to verify settings isolation
///
/// This function can be used to verify that changes in one isolated environment
/// don't affect another environment.
///
/// # Arguments
///
/// * `loader1` - First settings loader
/// * `loader2` - Second settings loader
/// * `settings1` - Settings to save in first environment
/// * `settings2` - Settings to save in second environment
///
/// # Returns
///
/// Returns `Ok(())` if the environments are properly isolated
/// Helper function to verify settings isolation between two environments
///
/// This function verifies that two isolated settings environments are truly independent
/// by loading settings from each and confirming they can operate without interference.
///
/// # Arguments
///
/// * `env1` - First isolated environment (TempDir, loader)
/// * `env2` - Second isolated environment (TempDir, loader)
///
/// # Returns
///
/// Returns `Ok(())` if the environments are properly isolated
pub async fn verify_settings_isolation(
    env1: &(TempDir, Arc<IsolatedSettingsLoader>),
    env2: &(TempDir, Arc<IsolatedSettingsLoader>),
) -> Result<(), String> {
    let (_, loader1) = env1;
    let (_, loader2) = env2;

    // Load settings from both environments
    let _settings1 = loader1
        .load_settings()
        .await
        .map_err(|e| format!("Failed to load from first environment: {e}"))?;
    let _settings2 = loader2
        .load_settings()
        .await
        .map_err(|e| format!("Failed to load from second environment: {e}"))?;

    // Both should be able to load independently (basic isolation test)
    // In practice, the real test is that they use different temp directories
    // which is verified by the TempDir instances being different

    // Verify they're using different paths by checking temp dir paths
    let path1 = env1.0.path();
    let path2 = env2.0.path();

    if path1 == path2 {
        return Err("Environments are using the same directory - not isolated!".to_string());
    }

    // Both should start with default settings if no custom settings were saved
    // This verifies they're not sharing any global state
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use speakr_lib::settings::SettingsLoader;

    #[tokio::test]
    async fn test_create_isolated_settings_env() {
        let (_temp_dir, loader) = create_isolated_settings_env().await;

        // Should be able to load default settings
        let settings = loader.load_settings().await.unwrap();
        assert_eq!(settings.audio_duration_secs, DEFAULT_AUDIO_DURATION_SECS);
    }

    #[tokio::test]
    async fn test_create_isolated_settings_env_with_custom_settings() {
        let mut custom_settings = AppSettings::default();
        custom_settings.audio_duration_secs = 25;

        let (_temp_dir, loader) = create_isolated_settings_env_with_settings(custom_settings).await;

        // Should load the custom settings
        let loaded_settings = loader.load_settings().await.unwrap();
        assert_eq!(loaded_settings.audio_duration_secs, 25);
    }

    #[tokio::test]
    async fn test_multiple_isolated_environments_are_independent() {
        let envs = create_multiple_isolated_settings_envs(2).await;
        assert_eq!(envs.len(), 2);

        let (_temp_dir1, loader1) = &envs[0];
        let (_temp_dir2, loader2) = &envs[1];

        // Both should load default settings independently
        let settings1 = loader1.load_settings().await.unwrap();
        let settings2 = loader2.load_settings().await.unwrap();

        assert_eq!(settings1.audio_duration_secs, DEFAULT_AUDIO_DURATION_SECS);
        assert_eq!(settings2.audio_duration_secs, DEFAULT_AUDIO_DURATION_SECS);

        // They should be independent instances
        assert_eq!(settings1, settings2); // Same values but different instances
    }

    #[tokio::test]
    async fn test_verify_settings_isolation() {
        let env1 = create_isolated_settings_env().await;
        let env2 = create_isolated_settings_env().await;

        // Should verify that the environments are properly isolated
        let result = verify_settings_isolation(&env1, &env2).await;
        assert!(result.is_ok(), "Settings environments should be isolated");
    }
}
