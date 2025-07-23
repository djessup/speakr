//! Speakr Tauri backend module.
//!
//! This module provides the Tauri commands and backend functionality for the Speakr
//! dictation application, including:
//! - Settings management and persistence
//! - Global hot-key registration using tauri-plugin-global-shortcut
//! - Model file validation
//! - System integration

pub mod commands;
pub mod services;
pub mod settings;

pub mod audio;
#[cfg(debug_assertions)]
pub mod debug;

use services::{
    get_backend_status_internal,
    hotkey::{register_global_hotkey_internal, unregister_global_hotkey_internal},
    update_service_status_internal, ServiceComponent,
};

use settings::{load_settings_internal, save_settings_internal};

use commands::{
    legacy::{greet_internal, register_hot_key_internal},
    system::{check_model_availability_internal, set_auto_launch_internal},
    validation::validate_hot_key_internal,
};

#[cfg(debug_assertions)]
use debug::{
    add_debug_log, debug_clear_log_messages_internal, debug_get_log_messages_internal,
    debug_start_recording_internal, debug_stop_recording_internal,
    debug_test_audio_recording_internal, DebugLogLevel, DebugLogMessage,
};

// Audio functions are accessed through full module paths in tests

use speakr_types::{AppError, AppSettings, HotkeyConfig, ServiceStatus, StatusUpdate};
// File system and path utilities are used in specific functions only
use tauri::{AppHandle, Manager};
use tracing::{error, info, warn};

// Core audio and WAV functionality is now in the audio module

// Debug and audio functionality is now in separate modules

/// Saves application settings to disk atomically.
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
#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), AppError> {
    save_settings_internal(settings).await
}

/// Loads application settings from disk with corruption recovery.
///
/// # Returns
///
/// Returns the loaded settings or default settings if the file doesn't exist.
/// If the file is corrupt, attempts to recover from backup, then falls back to defaults.
///
/// # Errors
///
/// Returns `AppError` if all recovery attempts fail.
#[tauri::command]
async fn load_settings() -> Result<AppSettings, AppError> {
    load_settings_internal().await
}

/// Tauri command wrapper for hot-key validation.
#[tauri::command]
async fn validate_hot_key(hot_key: String) -> Result<(), AppError> {
    validate_hot_key_internal(hot_key).await
}

/// Tauri command wrapper for model availability check.
#[tauri::command]
async fn check_model_availability(model_size: String) -> Result<bool, AppError> {
    check_model_availability_internal(model_size).await
}

/// Registers a global hot-key with the system (simple interface).
///
/// # Arguments
///
/// * `hot_key` - The hot-key combination to register
///
/// # Returns
///
/// Returns `Ok(())` if registration succeeds.
///
/// # Errors
///
/// Returns `AppError::HotKey` if registration fails.
#[tauri::command]
async fn register_hot_key(hot_key: String) -> Result<(), AppError> {
    register_hot_key_internal(hot_key).await
}

/// Tauri command wrapper to register a global hotkey using the GlobalHotkeyService
#[tauri::command]
async fn register_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String> {
    register_global_hotkey_internal(app_handle, config).await
}

/// Tauri command wrapper to unregister the current global hotkey
#[tauri::command]
async fn unregister_global_hotkey(app_handle: AppHandle) -> Result<(), String> {
    unregister_global_hotkey_internal(app_handle).await
}

/// Sets the auto-launch preference for the application.
///
/// # Arguments
///
/// * `enable` - Whether to enable auto-launch
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[tauri::command]
async fn set_auto_launch(enable: bool) -> Result<(), AppError> {
    set_auto_launch_internal(enable).await
}

/// Debug command to test audio recording functionality.
///
/// This command is only available in debug builds and provides
/// a stub implementation for testing the audio recording interface.
///
/// # Returns
///
/// Returns a success message for testing purposes.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_test_audio_recording() -> Result<String, AppError> {
    debug_test_audio_recording_internal().await
}

/// Debug command to start push-to-talk recording with real audio backend.
///
/// This command is only available in debug builds and starts
/// real audio recording using the speakr-core AudioRecorder.
///
/// # Returns
///
/// Returns a success message when recording starts.
///
/// # Errors
///
/// Returns `AppError` if recording fails to start.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_start_recording() -> Result<String, AppError> {
    debug_start_recording_internal().await
}

/// Debug command to stop push-to-talk recording.
///
/// This command is only available in debug builds and stops
/// the current recording session.
///
/// # Returns
///
/// Returns a success message when recording stops.
///
/// # Errors
///
/// Returns `AppError` if no recording is active or if stopping fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_stop_recording() -> Result<String, AppError> {
    debug_stop_recording_internal().await
}

/// Debug command to get all log messages.
///
/// This command is only available in debug builds and returns
/// all accumulated debug log messages for display in the frontend.
///
/// # Returns
///
/// Returns a vector of debug log messages.
///
/// # Errors
///
/// Returns `AppError` if retrieval fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_get_log_messages() -> Result<Vec<DebugLogMessage>, AppError> {
    debug_get_log_messages_internal().await
}

/// Debug command to clear all log messages.
///
/// This command is only available in debug builds and clears
/// the accumulated debug log messages.
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if clearing fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_clear_log_messages() -> Result<(), AppError> {
    debug_clear_log_messages_internal().await
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    greet_internal(name)
}

/// Tauri command to get current backend status
#[tauri::command]
async fn get_backend_status() -> Result<StatusUpdate, AppError> {
    get_backend_status_internal().await
}

/// Tauri command to update service status
#[tauri::command]
async fn update_service_status(
    component: ServiceComponent,
    status: ServiceStatus,
) -> Result<(), AppError> {
    update_service_status_internal(component, status).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler({
            #[cfg(debug_assertions)]
            {
                tauri::generate_handler![
                    greet,
                    save_settings,
                    load_settings,
                    validate_hot_key,
                    check_model_availability,
                    register_hot_key,
                    set_auto_launch,
                    register_global_hotkey,
                    unregister_global_hotkey,
                    debug_test_audio_recording,
                    debug_start_recording,
                    debug_stop_recording,
                    debug_get_log_messages,
                    debug_clear_log_messages,
                    get_backend_status,
                    update_service_status
                ]
            }
            #[cfg(not(debug_assertions))]
            {
                tauri::generate_handler![
                    greet,
                    save_settings,
                    load_settings,
                    validate_hot_key,
                    check_model_availability,
                    register_hot_key,
                    set_auto_launch,
                    register_global_hotkey,
                    unregister_global_hotkey,
                    get_backend_status,
                    update_service_status
                ]
            }
        })
        .setup(|app| {
            // Add initial debug log messages
            #[cfg(debug_assertions)]
            {
                add_debug_log(
                    DebugLogLevel::Info,
                    "speakr-tauri",
                    "Application starting in debug mode",
                );
            }

            info!("🎙️  Speakr backend starting up...");

            // Set up default global hotkey
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                // Default hotkey configuration
                let default_config = HotkeyConfig {
                    shortcut: "CmdOrCtrl+Alt+Space".to_string(),
                    enabled: true,
                };

                info!(
                    "⌨️  Registering default hotkey: {}",
                    default_config.shortcut
                );

                #[cfg(debug_assertions)]
                add_debug_log(
                    DebugLogLevel::Info,
                    "speakr-tauri",
                    &format!("Registering default hotkey: {}", default_config.shortcut),
                );

                if let Err(e) =
                    register_global_hotkey_internal(app_handle.clone(), default_config.clone())
                        .await
                {
                    error!(
                        "⚠️  Failed to register default hotkey '{}': {}",
                        default_config.shortcut, e
                    );

                    #[cfg(debug_assertions)]
                    add_debug_log(
                        DebugLogLevel::Error,
                        "speakr-tauri",
                        &format!("Failed to register default hotkey: {e}"),
                    );

                    warn!("💡 You can change the hotkey in Settings to avoid conflicts");

                    // Try a fallback hotkey if the default fails
                    let fallback_config = HotkeyConfig {
                        shortcut: "CmdOrCtrl+Alt+F2".to_string(),
                        enabled: true,
                    };

                    if let Err(e2) =
                        register_global_hotkey_internal(app_handle, fallback_config.clone()).await
                    {
                        error!(
                            "⚠️  Fallback hotkey '{}' also failed: {}",
                            fallback_config.shortcut, e2
                        );

                        #[cfg(debug_assertions)]
                        add_debug_log(
                            DebugLogLevel::Error,
                            "speakr-tauri",
                            &format!("Fallback hotkey also failed: {e2}"),
                        );

                        warn!(
                            "🔧 App will start without global hotkey - configure one in Settings"
                        );
                    } else {
                        info!("✅ Using fallback hotkey: {}", fallback_config.shortcut);

                        #[cfg(debug_assertions)]
                        add_debug_log(
                            DebugLogLevel::Info,
                            "speakr-tauri",
                            &format!("Using fallback hotkey: {}", fallback_config.shortcut),
                        );
                    }
                } else {
                    info!("✅ Default hotkey registered: {}", default_config.shortcut);

                    #[cfg(debug_assertions)]
                    add_debug_log(
                        DebugLogLevel::Info,
                        "speakr-tauri",
                        &format!("Default hotkey registered: {}", default_config.shortcut),
                    );
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
