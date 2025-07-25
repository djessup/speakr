// ============================================================================
//! Speakr Tauri backend module.
//!
//! This module provides the Tauri commands and backend functionality for the Speakr
//! dictation application, including:
//! - Settings management and persistence
//! - Global hot-key registration using tauri-plugin-global-shortcut
//! - Model file validation
//! - System integration
// ============================================================================

// =========================
// Module Declarations
// =========================
pub mod audio;
pub mod commands;
#[cfg(debug_assertions)]
pub mod debug;
pub mod services;
pub mod settings;

// =========================
// External Imports
// =========================
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
use services::{
    get_backend_status_internal,
    hotkey::{register_global_hotkey_internal, unregister_global_hotkey_internal},
    update_service_status_internal, ServiceComponent,
};
use settings::{load_settings_internal, save_settings_internal};
use speakr_types::{AppError, AppSettings, HotkeyConfig, ServiceStatus, StatusUpdate};
use tauri::{AppHandle, Manager};
use tracing::{error, info, warn};

// ============================================================================
// Tauri Command Definitions
// ============================================================================

// --------------------------------------------------------------------------
/// Saves application settings to disk atomically.
///
/// # Arguments
/// * `settings` - The settings to save
///
/// # Returns
/// Returns `Ok(())` on success.
///
/// # Errors
/// Returns `AppError` if the settings cannot be saved.
///
/// # Example
/// ```no_run
/// // In frontend: invoke('save_settings', { settings })
/// ```
#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), AppError> {
    save_settings_internal(settings).await
}

// --------------------------------------------------------------------------
/// Loads application settings from disk with corruption recovery.
///
/// # Returns
/// Returns the loaded settings or default settings if the file doesn't exist.
/// If the file is corrupt, attempts to recover from backup, then falls back to defaults.
///
/// # Errors
/// Returns `AppError` if all recovery attempts fail.
///
/// # Example
/// ```no_run
/// // In frontend: invoke('load_settings')
/// ```
#[tauri::command]
async fn load_settings() -> Result<AppSettings, AppError> {
    load_settings_internal().await
}

// --------------------------------------------------------------------------
/// Validates a hot-key string for correctness and conflicts.
///
/// # Arguments
/// * `hot_key` - The hot-key combination as a string
///
/// # Returns
/// Returns `Ok(())` if the hot-key is valid and available.
///
/// # Errors
/// Returns `AppError` if the hot-key is invalid or conflicts with system/global shortcuts.
#[tauri::command]
async fn validate_hot_key(hot_key: String) -> Result<(), AppError> {
    validate_hot_key_internal(hot_key).await
}

// --------------------------------------------------------------------------
/// Checks if a model file of the given size is available and valid.
///
/// # Arguments
/// * `model_size` - The model size identifier (e.g., "small", "medium")
///
/// # Returns
/// Returns `Ok(true)` if the model is available, `Ok(false)` otherwise.
///
/// # Errors
/// Returns `AppError` if the check fails due to IO or validation errors.
#[tauri::command]
async fn check_model_availability(model_size: String) -> Result<bool, AppError> {
    check_model_availability_internal(model_size).await
}

// --------------------------------------------------------------------------
/// Registers a global hot-key with the system (simple interface).
///
/// # Arguments
/// * `hot_key` - The hot-key combination to register
///
/// # Returns
/// Returns `Ok(())` if registration succeeds.
///
/// # Errors
/// Returns `AppError::HotKey` if registration fails.
#[tauri::command]
async fn register_hot_key(hot_key: String) -> Result<(), AppError> {
    register_hot_key_internal(hot_key).await
}

// --------------------------------------------------------------------------
/// Registers a global hotkey using the GlobalHotkeyService.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
/// * `config` - The hotkey configuration
///
/// # Returns
/// Returns `Ok(())` if registration succeeds, or an error string otherwise.
#[tauri::command]
async fn register_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String> {
    register_global_hotkey_internal(app_handle, config).await
}

// --------------------------------------------------------------------------
/// Unregisters the current global hotkey.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// Returns `Ok(())` if unregistration succeeds, or an error string otherwise.
#[tauri::command]
async fn unregister_global_hotkey(app_handle: AppHandle) -> Result<(), String> {
    unregister_global_hotkey_internal(app_handle).await
}

// --------------------------------------------------------------------------
/// Sets the auto-launch preference for the application.
///
/// # Arguments
/// * `enable` - Whether to enable auto-launch
///
/// # Returns
/// Returns `Ok(())` on success.
///
/// # Errors
/// Returns `AppError` if the operation fails.
#[tauri::command]
async fn set_auto_launch(enable: bool) -> Result<(), AppError> {
    set_auto_launch_internal(enable).await
}

// =========================
// Debug Commands (Debug Only)
// =========================
#[cfg(debug_assertions)]
/// Debug: Test audio recording functionality (stub for debug builds).
#[tauri::command]
async fn debug_test_audio_recording() -> Result<String, AppError> {
    debug_test_audio_recording_internal().await
}

#[cfg(debug_assertions)]
/// Debug: Start push-to-talk recording with real audio backend.
#[tauri::command]
async fn debug_start_recording() -> Result<String, AppError> {
    debug_start_recording_internal().await
}

#[cfg(debug_assertions)]
/// Debug: Stop push-to-talk recording.
#[tauri::command]
async fn debug_stop_recording() -> Result<String, AppError> {
    debug_stop_recording_internal().await
}

#[cfg(debug_assertions)]
/// Debug: Get all log messages for display in the frontend.
#[tauri::command]
async fn debug_get_log_messages() -> Result<Vec<DebugLogMessage>, AppError> {
    debug_get_log_messages_internal().await
}

#[cfg(debug_assertions)]
/// Debug: Clear all accumulated debug log messages.
#[tauri::command]
async fn debug_clear_log_messages() -> Result<(), AppError> {
    debug_clear_log_messages_internal().await
}

// --------------------------------------------------------------------------
/// Simple greeting command for legacy compatibility.
///
/// # Arguments
/// * `name` - The name to greet
///
/// # Returns
/// Returns a greeting string.
#[tauri::command]
fn greet(name: &str) -> String {
    greet_internal(name)
}

// --------------------------------------------------------------------------
/// Gets the current backend status for the frontend.
///
/// # Returns
/// Returns a `StatusUpdate` struct with the current backend status.
///
/// # Errors
/// Returns `AppError` if the status cannot be retrieved.
#[tauri::command]
async fn get_backend_status() -> Result<StatusUpdate, AppError> {
    get_backend_status_internal().await
}

// --------------------------------------------------------------------------
/// Updates the status of a backend service component.
///
/// # Arguments
/// * `component` - The service component to update
/// * `status` - The new status to set
///
/// # Returns
/// Returns `Ok(())` on success.
///
/// # Errors
/// Returns `AppError` if the update fails.
#[tauri::command]
async fn update_service_status(
    component: ServiceComponent,
    status: ServiceStatus,
) -> Result<(), AppError> {
    update_service_status_internal(component, status).await
}

// ============================================================================
// Application Entry Point
// ============================================================================

/// Runs the Tauri application, registering all plugins and commands.
///
/// This function sets up the Tauri builder, registers plugins, configures the invoke handler,
/// and performs initial setup (including default hotkey registration).
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
            // =========================
            // Initial Setup (Debug Logging, Hotkey Registration)
            // =========================
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
// ===========================================================================
