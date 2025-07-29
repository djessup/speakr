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
pub mod workflow;

// =========================
// External Imports
// =========================
use commands::{
    legacy::register_hot_key_internal,
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
    hotkey::{
        register_global_hotkey_internal, unregister_global_hotkey_internal,
        update_global_hotkey_internal,
    },
    update_service_status_internal, ServiceComponent,
};
use settings::{load_settings_internal, save_settings_internal};
use speakr_types::{AppError, AppSettings, HotkeyConfig, ServiceStatus, StatusUpdate};
use tauri::{App, AppHandle, Listener, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tracing::{error, info, warn};
use tracing_subscriber::fmt::fmt;
use tracing_subscriber::EnvFilter;
use workflow::execute_dictation_workflow;

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
/// Updates the global hotkey by re-registering with a new configuration.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
/// * `config` - The new hotkey configuration
///
/// # Returns
/// Returns `Ok(())` if update succeeds, or an error string otherwise.
#[tauri::command]
async fn update_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String> {
    update_global_hotkey_internal(app_handle, config).await
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

// Helper to centralise application setup logic
fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Speakr backend starting up...");

    #[cfg(desktop)]
    {
        let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
        app.handle().plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, shortcut, event| {
                    println!("{shortcut:?}");
                    if shortcut == &ctrl_n_shortcut {
                        match event.state() {
                            ShortcutState::Pressed => {
                                println!("Ctrl-N Pressed!");
                            }
                            ShortcutState::Released => {
                                println!("Ctrl-N Released!");
                            }
                        }
                    }
                })
                .build(),
        )?;

        app.global_shortcut().register(ctrl_n_shortcut)?;
    }

    // Set up the hotkey-triggered listener
    setup_hotkey_trigger_listener(app);

    // Spawn task to register the default global hotkey
    spawn_register_default_hotkey(app.app_handle().clone());

    Ok(())
}

// Sets up the event listener for the "hotkey-triggered" event
fn setup_hotkey_trigger_listener(app: &App) {
    let app_handle_for_listener = app.app_handle().clone();
    app.listen("hotkey-triggered", move |_event| {
        let app_handle = app_handle_for_listener.clone();
        tauri::async_runtime::spawn(async move {
            info!("🔥 Hotkey triggered, starting dictation workflow");

            #[cfg(debug_assertions)]
            add_debug_log(
                DebugLogLevel::Info,
                "workflow",
                "Hotkey triggered, starting dictation workflow",
            );

            if let Err(e) = execute_dictation_workflow(app_handle.clone()).await {
                error!("Dictation workflow failed: {}", e);

                #[cfg(debug_assertions)]
                add_debug_log(
                    DebugLogLevel::Error,
                    "workflow",
                    &format!("Dictation workflow failed: {e}"),
                );
            }
        });
    });
}

// Spawns the async task to register the default global hotkey
fn spawn_register_default_hotkey(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        register_default_hotkey(app_handle).await;
    });
}

// Performs default and fallback hotkey registration
async fn register_default_hotkey(app_handle: AppHandle) {
    // Load hotkey from persisted settings, falling back to default if loading fails
    let hotkey_config = match load_settings_internal().await {
        Ok(settings) => {
            info!("Loaded hotkey from settings: {}", settings.hot_key);
            HotkeyConfig {
                shortcut: settings.hot_key,
                enabled: true,
            }
        }
        Err(e) => {
            warn!("Failed to load settings, using default hotkey: {}", e);
            HotkeyConfig {
                shortcut: "CmdOrCtrl+Alt+Space".to_string(),
                enabled: true,
            }
        }
    };

    info!("Registering hotkey: {}", hotkey_config.shortcut);
    // #[cfg(debug_assertions)]
    // add_debug_log(
    //     DebugLogLevel::Info,
    //     "speakr-tauri",
    //     &format!("Registering default hotkey: {}", default_config.shortcut)
    // );

    if let Err(e) = register_global_hotkey_internal(app_handle.clone(), hotkey_config.clone()).await
    {
        error!(
            "⚠️  Failed to register hotkey '{}': {}",
            hotkey_config.shortcut, e
        );
        warn!("💡 You can change the hotkey in Settings to avoid conflicts");

        // Fallback hotkey
        let fallback_config = HotkeyConfig {
            shortcut: "CmdOrCtrl+Alt+F2".to_string(),
            enabled: true,
        };

        if let Err(e2) = register_global_hotkey_internal(app_handle, fallback_config.clone()).await
        {
            error!(
                "⚠️  Fallback hotkey '{}' also failed: {}",
                fallback_config.shortcut, e2
            );
            warn!("App will start without global hotkey - configure one in Settings");
        } else {
            info!("Using fallback hotkey: {}", fallback_config.shortcut);
        }
    } else {
        info!("Hotkey registered: {}", hotkey_config.shortcut);
        #[cfg(debug_assertions)]
        add_debug_log(
            DebugLogLevel::Info,
            "speakr-tauri",
            &format!("Hotkey registered: {}", hotkey_config.shortcut),
        );
    }
}

/// Runs the Tauri application, registering all plugins and commands.
///
/// This function sets up the Tauri builder, registers plugins, configures the invoke handler,
/// and performs initial setup (including default hotkey registration).
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    // // Enable instrumentation in development builds
    // #[cfg(debug_assertions)]
    // {
    //     builder = builder.plugin(tauri_plugin_devtools::init());
    // }

    // Initialise a logging subscriber that respects RUST_LOG
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    builder
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler({
            #[cfg(debug_assertions)]
            {
                tauri::generate_handler![
                    save_settings,
                    load_settings,
                    validate_hot_key,
                    check_model_availability,
                    register_hot_key,
                    set_auto_launch,
                    register_global_hotkey,
                    unregister_global_hotkey,
                    update_global_hotkey,
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
                    save_settings,
                    load_settings,
                    validate_hot_key,
                    check_model_availability,
                    register_hot_key,
                    set_auto_launch,
                    register_global_hotkey,
                    unregister_global_hotkey,
                    update_global_hotkey,
                    get_backend_status,
                    update_service_status
                ]
            }
        })
        .setup(move |app| setup_app(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// ===========================================================================
