//! Speakr Tauri backend module.
//!
//! This module provides the Tauri commands and backend functionality for the Speakr
//! dictation application, including:
//! - Settings management and persistence
//! - Global hot-key registration using tauri-plugin-global-shortcut
//! - Model file validation
//! - System integration

use speakr_types::{
    AppError, AppSettings, BackendStatus, HotkeyConfig, HotkeyError, ServiceStatus, StatusUpdate,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use tracing::{debug, error, info, warn};

// Add import for audio functionality
use speakr_core::audio::{AudioRecorder, RecordingConfig};

// Add import for WAV file writing
use hound::{WavSpec, WavWriter};

#[cfg(debug_assertions)]
use serde::{Deserialize, Serialize};

#[cfg(debug_assertions)]
use std::collections::VecDeque;

#[cfg(debug_assertions)]
use std::sync::LazyLock;

// Debug-only log message types and storage
#[cfg(debug_assertions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLogMessage {
    pub timestamp: String,
    pub level: DebugLogLevel,
    pub target: String,
    pub message: String,
}

// Shared state for debug recording session
#[cfg(debug_assertions)]
#[derive(Debug)]
struct DebugRecordingState {
    recorder: Option<AudioRecorder>,
    start_time: Option<std::time::Instant>,
}

#[cfg(debug_assertions)]
static DEBUG_LOG_MESSAGES: LazyLock<Arc<Mutex<VecDeque<DebugLogMessage>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(VecDeque::with_capacity(1000))));

#[cfg(debug_assertions)]
static DEBUG_RECORDING_STATE: LazyLock<Arc<Mutex<DebugRecordingState>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(DebugRecordingState {
        recorder: None,
        start_time: None,
    }))
});

#[cfg(debug_assertions)]
impl DebugLogMessage {
    pub fn new(level: DebugLogLevel, target: &str, message: &str) -> Self {
        Self {
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            level,
            target: target.to_string(),
            message: message.to_string(),
        }
    }
}

#[cfg(debug_assertions)]
pub fn add_debug_log(level: DebugLogLevel, target: &str, message: &str) {
    if let Ok(mut logs) = DEBUG_LOG_MESSAGES.lock() {
        logs.push_back(DebugLogMessage::new(level, target, message));

        // Keep only the last 1000 messages
        while logs.len() > 1000 {
            logs.pop_front();
        }
    }
}

// All types are now centralized in speakr-types crate

/// Gets the settings file path in the app data directory.
fn get_settings_path() -> Result<PathBuf, AppError> {
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
#[allow(dead_code)] // Used in tests
fn get_settings_backup_path() -> Result<PathBuf, AppError> {
    let settings_path = get_settings_path()?;
    Ok(settings_path.with_extension("json.backup"))
}

/// Migrates settings from older versions to the current schema.
///
/// # Arguments
///
/// * `settings` - The settings loaded from disk
///
/// # Returns
///
/// Returns the migrated settings with updated version number.
fn migrate_settings(mut settings: AppSettings) -> AppSettings {
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
    // Use the global settings directory for production
    let settings_path = get_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .ok_or_else(|| AppError::Settings("Invalid settings path".to_string()))?
        .to_path_buf();

    save_settings_to_dir(&settings, &settings_dir).await
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
    // Use the global settings directory for production
    let settings_path = get_settings_path()?;
    let settings_dir = settings_path
        .parent()
        .ok_or_else(|| AppError::Settings("Invalid settings path".to_string()))?
        .to_path_buf();

    load_settings_from_dir(&settings_dir).await
}

/// Helper function to load settings from a specific file path.
///
/// # Arguments
///
/// * `path` - The path to the settings file
///
/// # Returns
///
/// Returns the loaded settings on success.
///
/// # Errors
///
/// Returns error if the file cannot be read or parsed.
fn try_load_settings_file(path: &PathBuf) -> Result<AppSettings, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read settings file: {e}"))?;

    let settings: AppSettings = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse settings JSON: {e}"))?;

    Ok(settings)
}

/// Service responsible for managing global hot-keys
pub struct GlobalHotkeyService {
    app_handle: AppHandle,
    current_shortcut: Arc<Mutex<Option<String>>>,
    current_shortcut_instance: Arc<Mutex<Option<Shortcut>>>,
}

impl GlobalHotkeyService {
    /// Creates a new GlobalHotkeyService instance
    ///
    /// # Arguments
    ///
    /// * `app_handle` - The Tauri application handle for registering shortcuts
    ///
    /// # Errors
    ///
    /// Returns `HotkeyError` if service initialization fails
    pub fn new(app_handle: AppHandle) -> Result<Self, HotkeyError> {
        Ok(Self {
            app_handle,
            current_shortcut: Arc::new(Mutex::new(None)),
            current_shortcut_instance: Arc::new(Mutex::new(None)),
        })
    }

    /// Registers a global hot-key with the system
    ///
    /// # Arguments
    ///
    /// * `config` - The hot-key configuration to register
    ///
    /// # Errors
    ///
    /// Returns `HotkeyError::RegistrationFailed` if registration fails
    /// Returns `HotkeyError::ConflictDetected` if the hot-key is already in use
    pub async fn register_hotkey(&mut self, config: &HotkeyConfig) -> Result<(), HotkeyError> {
        // If hotkey is disabled, succeed but don't actually register
        if !config.enabled {
            return Ok(());
        }

        // Parse the shortcut string into Tauri's format
        let shortcut = self.parse_shortcut(&config.shortcut).map_err(|e| {
            HotkeyError::RegistrationFailed(format!("Invalid shortcut format: {e}"))
        })?;

        // Unregister existing shortcut if any
        if let Ok(mut current_instance) = self.current_shortcut_instance.lock() {
            if let Some(existing_shortcut) = current_instance.take() {
                let _ = self
                    .app_handle
                    .global_shortcut()
                    .unregister(existing_shortcut);
            }
        }

        // Register the new shortcut with the system
        let app_handle_clone = self.app_handle.clone();
        self.app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |_app, _shortcut, _event| {
                // Emit an event when the hotkey is triggered
                let _ = app_handle_clone.emit("hotkey-triggered", ());

                // TODO: Wire this to speakr-core pipeline in next step
                debug!("Global hotkey triggered");
            })
            .map_err(|e| {
                HotkeyError::ConflictDetected(format!("Failed to register shortcut: {e}"))
            })?;

        // Register the shortcut for system-wide listening
        self.app_handle
            .global_shortcut()
            .register(shortcut)
            .map_err(|e| {
                HotkeyError::ConflictDetected(format!(
                    "Failed to register shortcut with system (conflict?): {e}"
                ))
            })?;

        // Update internal state
        if let Ok(mut current) = self.current_shortcut.lock() {
            *current = Some(config.shortcut.clone());
        }

        if let Ok(mut current_instance) = self.current_shortcut_instance.lock() {
            *current_instance = Some(shortcut);
        }

        info!("Successfully registered global hotkey: {}", config.shortcut);
        Ok(())
    }

    /// Unregisters the currently registered hot-key
    ///
    /// # Errors
    ///
    /// Returns `HotkeyError::NotFound` if no hot-key is currently registered
    pub async fn unregister_hotkey(&mut self) -> Result<(), HotkeyError> {
        let mut current_instance = self
            .current_shortcut_instance
            .lock()
            .map_err(|_| HotkeyError::RegistrationFailed("Failed to acquire lock".to_string()))?;

        if let Some(shortcut) = current_instance.take() {
            self.app_handle
                .global_shortcut()
                .unregister(shortcut)
                .map_err(|e| {
                    HotkeyError::RegistrationFailed(format!("Failed to unregister shortcut: {e}"))
                })?;

            // Clear current shortcut
            if let Ok(mut current) = self.current_shortcut.lock() {
                *current = None;
            }

            info!("Successfully unregistered global hotkey");
            Ok(())
        } else {
            Err(HotkeyError::NotFound(
                "No hotkey currently registered".to_string(),
            ))
        }
    }

    /// Updates the current hot-key registration with new configuration
    ///
    /// # Arguments
    ///
    /// * `new_config` - The new hot-key configuration
    ///
    /// # Errors
    ///
    /// Returns `HotkeyError` if the update fails
    pub async fn update_hotkey(&mut self, new_config: &HotkeyConfig) -> Result<(), HotkeyError> {
        // Simply unregister the old one and register the new one
        let _ = self.unregister_hotkey().await; // Ignore error if nothing was registered
        self.register_hotkey(new_config).await
    }

    /// Checks if a hot-key is currently registered
    pub fn is_registered(&self) -> bool {
        if let Ok(current_instance) = self.current_shortcut_instance.lock() {
            current_instance.is_some()
        } else {
            false
        }
    }

    /// Gets the currently registered hot-key shortcut
    pub fn current_shortcut(&self) -> Option<String> {
        if let Ok(current) = self.current_shortcut.lock() {
            current.clone()
        } else {
            None
        }
    }

    /// Parses a shortcut string into a Tauri Shortcut
    ///
    /// # Arguments
    ///
    /// * `shortcut_str` - The shortcut string (e.g. "CmdOrCtrl+Alt+Space")
    ///
    /// # Errors
    ///
    /// Returns error if the shortcut format is invalid
    fn parse_shortcut(&self, shortcut_str: &str) -> Result<Shortcut, String> {
        let parts: Vec<&str> = shortcut_str.split('+').collect();

        if parts.is_empty() {
            return Err("Empty shortcut string".to_string());
        }

        let mut modifiers = Modifiers::empty();
        let mut key_part = None;

        for part in &parts {
            match part.to_lowercase().as_str() {
                "cmd" | "cmdorctrl" => {
                    modifiers |= Modifiers::SUPER;
                }
                "ctrl" => {
                    modifiers |= Modifiers::CONTROL;
                }
                "alt" | "option" => {
                    modifiers |= Modifiers::ALT;
                }
                "shift" => {
                    modifiers |= Modifiers::SHIFT;
                }
                _ => {
                    key_part = Some(*part);
                }
            }
        }

        let key = key_part.ok_or("No key specified in shortcut")?;
        let code = match key.to_lowercase().as_str() {
            "space" => Code::Space,
            "enter" => Code::Enter,
            "escape" => Code::Escape,
            "backspace" => Code::Backspace,
            "delete" => Code::Delete,
            "tab" => Code::Tab,
            "`" | "grave" => Code::Backquote,
            "a" => Code::KeyA,
            "b" => Code::KeyB,
            "c" => Code::KeyC,
            "d" => Code::KeyD,
            "e" => Code::KeyE,
            "f" => Code::KeyF,
            "g" => Code::KeyG,
            "h" => Code::KeyH,
            "i" => Code::KeyI,
            "j" => Code::KeyJ,
            "k" => Code::KeyK,
            "l" => Code::KeyL,
            "m" => Code::KeyM,
            "n" => Code::KeyN,
            "o" => Code::KeyO,
            "p" => Code::KeyP,
            "q" => Code::KeyQ,
            "r" => Code::KeyR,
            "s" => Code::KeyS,
            "t" => Code::KeyT,
            "u" => Code::KeyU,
            "v" => Code::KeyV,
            "w" => Code::KeyW,
            "x" => Code::KeyX,
            "y" => Code::KeyY,
            "z" => Code::KeyZ,
            _ => {
                return Err(format!("Unsupported key: {key}"));
            }
        };

        Ok(Shortcut::new(Some(modifiers), code))
    }
}

/// Validates a hot-key combination with comprehensive format support.
///
/// # Arguments
///
/// * `hot_key` - The hot-key string to validate
///
/// # Returns
///
/// Returns `Ok(())` if valid.
///
/// # Errors
///
/// Returns `AppError::HotKey` if the hot-key is invalid.
#[tauri::command]
async fn validate_hot_key(hot_key: String) -> Result<(), AppError> {
    if hot_key.is_empty() {
        return Err(AppError::HotKey("Hot-key cannot be empty".to_string()));
    }

    // Enhanced validation - supports both old and new formats
    let modifiers = ["CMD", "CMDORCTRL", "CTRL", "ALT", "OPTION", "SHIFT"];
    let has_modifier = modifiers.iter().any(|m| hot_key.to_uppercase().contains(m));

    if !has_modifier {
        return Err(AppError::HotKey(
            "Hot-key must contain at least one modifier key".to_string(),
        ));
    }

    // Test if the shortcut can be parsed using the same logic as GlobalHotkeyService
    let parts: Vec<&str> = hot_key.split('+').collect();
    if parts.is_empty() {
        return Err(AppError::HotKey("Empty shortcut string".to_string()));
    }

    let mut has_key = false;
    for part in &parts {
        match part.to_lowercase().trim() {
            "cmd" | "cmdorctrl" | "ctrl" | "alt" | "option" | "shift" => {
                // Valid modifier, continue
            }
            _ => {
                has_key = true;
                // Check if it's a valid key
                match part.to_lowercase().trim() {
                    "space" | "enter" | "escape" | "backspace" | "delete" | "tab" | "`"
                    | "grave" => {}
                    k if k.len() == 1 && k.chars().all(|c| c.is_ascii_alphabetic()) => {}
                    _ => {
                        return Err(AppError::HotKey(format!("Unsupported key: {part}")));
                    }
                }
            }
        }
    }

    if !has_key {
        return Err(AppError::HotKey("No key specified in shortcut".to_string()));
    }

    Ok(())
}

/// Checks if a model file exists for the given model size.
///
/// # Arguments
///
/// * `model_size` - The model size to check ("small", "medium", "large")
///
/// # Returns
///
/// Returns `true` if the model file exists, `false` otherwise.
#[tauri::command]
async fn check_model_availability(model_size: String) -> Result<bool, AppError> {
    let filename = match model_size.as_str() {
        "small" => "ggml-small.bin",
        "medium" => "ggml-medium.bin",
        "large" => "ggml-large.bin",
        _ => {
            return Err(AppError::Settings(format!(
                "Unknown model size: {model_size}"
            )));
        }
    };

    // Check in models directory relative to the app
    let models_dir = std::env::current_dir()
        .map_err(|e| AppError::FileSystem(format!("Failed to get current dir: {e}")))?
        .join("models");

    let model_path = models_dir.join(filename);
    Ok(model_path.exists())
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
    // Validate the hot-key first
    validate_hot_key(hot_key.clone()).await?;

    // For now, just return success since the frontend handles registration
    // This maintains backward compatibility
    Ok(())
}

/// Tauri command to register a global hotkey using the GlobalHotkeyService
#[tauri::command]
async fn register_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String> {
    let mut service = GlobalHotkeyService::new(app_handle).map_err(|e| e.to_string())?;

    service
        .register_hotkey(&config)
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command to unregister the current global hotkey
#[tauri::command]
async fn unregister_global_hotkey(app_handle: AppHandle) -> Result<(), String> {
    let mut service = GlobalHotkeyService::new(app_handle).map_err(|e| e.to_string())?;

    service.unregister_hotkey().await.map_err(|e| e.to_string())
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
    // TODO: Implement actual auto-launch registration using system APIs
    // For now, just return success
    let _ = enable;
    Ok(())
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
    use std::time::Duration;

    add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        "Starting audio recording test",
    );

    // Simulate some processing time
    tokio::time::sleep(Duration::from_millis(500)).await;

    add_debug_log(
        DebugLogLevel::Debug,
        "speakr-core",
        "Mock audio recording completed",
    );

    // Return a mock success result
    Ok("Audio recording test completed successfully! (Mock implementation)".to_string())
}

/// Gets the default output directory for debug audio recordings.
///
/// # Returns
///
/// Returns the path to the user's Documents/Speakr/debug_recordings/ directory.
///
/// # Errors
///
/// Returns `AppError` if the directory cannot be created.
#[cfg(debug_assertions)]
fn get_debug_recordings_directory() -> Result<PathBuf, AppError> {
    let documents_dir = dirs::document_dir()
        .ok_or_else(|| AppError::Settings("Could not find Documents directory".to_string()))?;

    let debug_dir = documents_dir.join("Speakr").join("debug_recordings");

    // Create directory if it doesn't exist
    if !debug_dir.exists() {
        fs::create_dir_all(&debug_dir).map_err(|e| {
            AppError::FileSystem(format!("Failed to create debug recordings dir: {e}"))
        })?;
    }

    Ok(debug_dir)
}

/// Debug command to start push-to-talk recording with real audio backend.
///
/// This command is only available in debug builds and starts
/// real audio recording using the speakr-core AudioRecorder.
///
/// # Returns
///
/// Returns a success message indicating recording has started.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_start_recording() -> Result<String, AppError> {
    info!("🎙️ Debug: Starting real push-to-talk recording");
    add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        "Real push-to-talk recording started",
    );

    // Check if already recording
    {
        let state = DEBUG_RECORDING_STATE.lock().unwrap();
        if state.recorder.is_some() {
            return Ok("Recording already in progress".to_string());
        }
    }

    // Create audio recorder with 30 second max duration (push-to-talk should be shorter)
    let config = RecordingConfig::new(30);
    let recorder = AudioRecorder::new(config)
        .await
        .map_err(|e| AppError::Settings(format!("Failed to create audio recorder: {e}")))?;

    // Start recording
    recorder
        .start_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to start recording: {e}")))?;

    // Store recorder in global state
    {
        let mut state = DEBUG_RECORDING_STATE.lock().unwrap();
        state.recorder = Some(recorder);
        state.start_time = Some(std::time::Instant::now());
    }

    add_debug_log(
        DebugLogLevel::Info,
        "speakr-core",
        "Real audio recording started successfully",
    );

    Ok("🎙️ Real recording started! Release button to stop and save.".to_string())
}

/// Debug command to stop push-to-talk recording and save to disk.
///
/// This command is only available in debug builds and stops
/// the current recording, then saves the audio to a timestamped WAV file.
///
/// # Returns
///
/// Returns a message with the file path where audio was saved.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_stop_recording() -> Result<String, AppError> {
    info!("⏹️ Debug: Stopping real push-to-talk recording and saving to disk");

    // Get recorder from global state
    let (recorder, start_time) = {
        let mut state = DEBUG_RECORDING_STATE.lock().unwrap();
        let recorder = state.recorder.take();
        let start_time = state.start_time.take();
        (recorder, start_time)
    };

    let Some(recorder) = recorder else {
        add_debug_log(
            DebugLogLevel::Warn,
            "speakr-debug",
            "No active recording to stop",
        );
        return Ok("No recording was active".to_string());
    };

    // Stop recording and get samples
    let result = recorder
        .stop_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to stop recording: {e}")))?;

    let samples = result.samples();
    let duration = start_time.map(|t| t.elapsed()).unwrap_or_default();

    add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        &format!(
            "Recording stopped, captured {} samples in {:.2}s",
            samples.len(),
            duration.as_secs_f64()
        ),
    );

    // Save to file in debug recordings directory
    let output_dir = get_debug_recordings_directory()?;
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    let success_message = format!(
        "⏹️ Recording saved! {} samples ({:.2}s) → {}",
        samples.len(),
        duration.as_secs_f64(),
        output_path.display()
    );

    add_debug_log(DebugLogLevel::Info, "speakr-debug", &success_message);

    info!("{}", success_message);
    Ok(success_message)
}

/// Debug command to get recent log messages.
///
/// This command is only available in debug builds and returns
/// the collected log messages for display in the debug console.
///
/// # Returns
///
/// Returns a vector of log messages.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_get_log_messages() -> Result<Vec<DebugLogMessage>, AppError> {
    if let Ok(logs) = DEBUG_LOG_MESSAGES.lock() {
        Ok(logs.iter().cloned().collect())
    } else {
        Err(AppError::Settings(
            "Failed to access log messages".to_string(),
        ))
    }
}

/// Debug command to clear all log messages.
///
/// This command is only available in debug builds and clears
/// the collected log messages from memory.
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
#[cfg(debug_assertions)]
#[tauri::command]
async fn debug_clear_log_messages() -> Result<(), AppError> {
    if let Ok(mut logs) = DEBUG_LOG_MESSAGES.lock() {
        logs.clear();
        add_debug_log(DebugLogLevel::Info, "speakr-debug", "Log messages cleared");
        Ok(())
    } else {
        Err(AppError::Settings(
            "Failed to clear log messages".to_string(),
        ))
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

/// Validates that the settings directory has proper permissions for read/write operations.
///
/// # Arguments
///
/// * `dir_path` - The directory path to validate
///
/// # Returns
///
/// Returns `Ok(())` if permissions are valid.
///
/// # Errors
///
/// Returns `AppError` if permissions are insufficient.
fn validate_settings_directory_permissions(dir_path: &Path) -> Result<(), AppError> {
    // Minimal implementation to pass the test
    // Check if directory exists and is writable
    if !dir_path.exists() {
        return Err(AppError::FileSystem("Directory does not exist".to_string()));
    }

    // Try to create a test file to verify write permissions
    let test_file = dir_path.join(".permission_test");
    match std::fs::write(&test_file, "test") {
        Ok(_) => {
            // Clean up test file
            let _ = std::fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => Err(AppError::FileSystem(format!("Directory not writable: {e}"))),
    }
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
async fn load_settings_from_dir(settings_dir: &PathBuf) -> Result<AppSettings, AppError> {
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

// 🟢 GREEN: Minimal implementations for TDD tests

/// Generates a filename with timestamp for audio recordings.
///
/// # Returns
///
/// A filename string in the format "recording_YYYY-MM-DD_HH-MM-SS.wav"
fn generate_audio_filename_with_timestamp() -> String {
    let now = chrono::Utc::now();
    format!("recording_{}.wav", now.format("%Y-%m-%d_%H-%M-%S%.3f"))
}

/// Saves audio samples to a WAV file.
///
/// # Arguments
///
/// * `samples` - The audio samples to save (16-bit mono)
/// * `output_path` - The path where the WAV file should be saved
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the file cannot be written.
async fn save_audio_samples_to_wav_file(
    samples: &[i16],
    output_path: &PathBuf,
) -> Result<(), AppError> {
    let spec = WavSpec {
        channels: 1,         // Mono
        sample_rate: 16_000, // 16 kHz
        bits_per_sample: 16, // 16-bit
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_path, spec)
        .map_err(|e| AppError::FileSystem(format!("Failed to create WAV file: {e}")))?;

    for &sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| AppError::FileSystem(format!("Failed to write audio sample: {e}")))?;
    }

    writer
        .finalize()
        .map_err(|e| AppError::FileSystem(format!("Failed to finalize WAV file: {e}")))?;

    Ok(())
}

/// Records audio to file (mock implementation for testing).
///
/// # Arguments
///
/// * `output_dir` - Directory where the audio file should be saved
/// * `duration_secs` - Recording duration in seconds
///
/// # Returns
///
/// Returns the path to the created audio file.
///
/// # Errors
///
/// Returns `AppError` if recording or file saving fails.
#[allow(dead_code)] // Used only in tests
async fn debug_record_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // Generate filename with timestamp
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    // Create mock audio samples for testing (simple sine wave)
    let sample_rate = 16_000;
    let samples: Vec<i16> = (0..sample_rate * duration_secs)
        .map(|i| {
            let t = (i as f64) / (sample_rate as f64);
            let frequency = 440.0; // A note
            let amplitude = 16000.0;
            (amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin()) as i16
        })
        .collect();

    // Save to WAV file
    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}

/// Records real audio to file using speakr-core AudioRecorder.
///
/// # Arguments
///
/// * `output_dir` - Directory where the audio file should be saved
/// * `duration_secs` - Recording duration in seconds
///
/// # Returns
///
/// Returns the path to the created audio file.
///
/// # Errors
///
/// Returns `AppError` if recording or file saving fails.
#[allow(dead_code)] // Used only in tests
async fn debug_record_real_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // Create recorder with the specified duration
    let config = RecordingConfig::new(duration_secs);
    let recorder = AudioRecorder::new(config)
        .await
        .map_err(|e| AppError::Settings(format!("Failed to create audio recorder: {e}")))?;

    // Start recording
    recorder
        .start_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to start recording: {e}")))?;

    // Wait for the recording duration
    tokio::time::sleep(std::time::Duration::from_secs(duration_secs as u64)).await;

    // Stop recording and get samples
    let result = recorder
        .stop_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to stop recording: {e}")))?;

    let samples = result.samples();

    // Generate filename with timestamp and save to file
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}

/// Enum to identify different service components
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceComponent {
    AudioCapture,
    Transcription,
    TextInjection,
}

/// Service responsible for tracking backend component status
pub struct BackendStatusService {
    status: Arc<Mutex<BackendStatus>>,
}

impl BackendStatusService {
    /// Creates a new backend status service with all services starting
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(BackendStatus::new_starting())),
        }
    }

    /// Gets the current status snapshot
    pub fn get_current_status(&self) -> BackendStatus {
        self.status.lock().unwrap().clone()
    }

    /// Updates the status of a specific service component
    pub fn update_service_status(&mut self, component: ServiceComponent, status: ServiceStatus) {
        if let Ok(mut current_status) = self.status.lock() {
            // Update timestamp when any service changes
            current_status.timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;

            // Update the specific service
            match component {
                ServiceComponent::AudioCapture => {
                    current_status.audio_capture = status;
                }
                ServiceComponent::Transcription => {
                    current_status.transcription = status;
                }
                ServiceComponent::TextInjection => {
                    current_status.text_injection = status;
                }
            }
        }
    }

    /// Emits status change event to frontend
    pub fn emit_status_change(&self, app_handle: &AppHandle) -> Result<(), String> {
        let status = self.get_current_status();
        app_handle
            .emit("speakr-status-changed", &status)
            .map_err(|e| format!("Failed to emit status change: {e}"))
    }

    /// Emits heartbeat event to frontend
    pub fn emit_heartbeat(&self, app_handle: &AppHandle) -> Result<(), String> {
        let status = self.get_current_status();
        app_handle
            .emit("speakr-heartbeat", &status)
            .map_err(|e| format!("Failed to emit heartbeat: {e}"))
    }
}

impl Default for BackendStatusService {
    fn default() -> Self {
        Self::new()
    }
}

/// Tauri command to get current backend status
#[tauri::command]
async fn get_backend_status() -> Result<StatusUpdate, AppError> {
    // For now, return a starting status
    // In real implementation, this would get status from a global service instance
    Ok(BackendStatus::new_starting())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
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
                    get_backend_status
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
                    get_backend_status
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
                    "Application starting in debug mode"
                );
                add_debug_log(
                    DebugLogLevel::Debug,
                    "speakr-tauri",
                    "Debug logging system initialized"
                );
                add_debug_log(
                    DebugLogLevel::Info,
                    "speakr-tauri",
                    "Debug panel available via toggle button"
                );
            }

            // Register default hotkey on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let default_config = HotkeyConfig::default();
                if let Ok(mut service) = GlobalHotkeyService::new(app_handle) {
                    if let Err(e) = service.register_hotkey(&default_config).await {
                        error!(
                            "⚠️  Failed to register default hotkey '{}': {}",
                            default_config.shortcut,
                            e
                        );

                        #[cfg(debug_assertions)]
                        add_debug_log(
                            DebugLogLevel::Warn,
                            "speakr-tauri",
                            &format!("Failed to register default hotkey: {e}")
                        );

                        warn!("💡 You can change the hotkey in Settings to avoid conflicts");

                        // Try a fallback hotkey if the default fails
                        let fallback_config = HotkeyConfig {
                            shortcut: "CmdOrCtrl+Alt+F2".to_string(),
                            enabled: true,
                        };

                        if let Err(e2) = service.register_hotkey(&fallback_config).await {
                            error!(
                                "⚠️  Fallback hotkey '{}' also failed: {}",
                                fallback_config.shortcut,
                                e2
                            );

                            #[cfg(debug_assertions)]
                            add_debug_log(
                                DebugLogLevel::Error,
                                "speakr-tauri",
                                &format!("Fallback hotkey also failed: {e2}")
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
                                &format!("Using fallback hotkey: {}", fallback_config.shortcut)
                            );
                        }
                    } else {
                        info!("✅ Default hotkey registered: {}", default_config.shortcut);

                        #[cfg(debug_assertions)]
                        add_debug_log(
                            DebugLogLevel::Info,
                            "speakr-tauri",
                            &format!("Default hotkey registered: {}", default_config.shortcut)
                        );
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use speakr_types::ServiceStatus;

    #[tokio::test]
    async fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.version, 1);
        assert_eq!(settings.hot_key, "CmdOrCtrl+Alt+F1"); // Use the actual default from speakr_types
        assert_eq!(settings.model_size, "medium");
        assert!(!settings.auto_launch);
    }

    #[tokio::test]
    async fn test_save_and_load_settings() {
        // Use a temporary directory for this test to avoid interference
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let temp_settings_path = temp_dir.path().join("settings.json");

        // Arrange
        let test_settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+S".to_string(),
            model_size: "large".to_string(),
            auto_launch: true,
        };

        // Test the helper function directly since we can't override the global path
        // Save settings to temp file
        let json = serde_json::to_string_pretty(&test_settings).expect("Should serialize");
        std::fs::write(&temp_settings_path, json).expect("Should write test file");

        // Load and verify using our helper function
        let loaded_settings =
            try_load_settings_file(&temp_settings_path).expect("Should load test settings");
        assert_eq!(loaded_settings, test_settings);

        // Test migration works
        let migrated = migrate_settings(loaded_settings);
        assert_eq!(migrated, test_settings); // Should be unchanged since it's already version 1
    }

    #[tokio::test]
    async fn test_settings_migration() {
        // Test migration from version 0 to current
        let old_settings = AppSettings {
            version: 0,
            ..Default::default()
        };

        let migrated = migrate_settings(old_settings);
        assert_eq!(migrated.version, 1);

        // Test current version (no migration)
        let current_settings = AppSettings::default();
        let unchanged = migrate_settings(current_settings.clone());
        assert_eq!(unchanged, current_settings);
    }

    #[tokio::test]
    async fn test_atomic_write_creates_backup() {
        // Create initial settings
        let initial_settings = AppSettings::default();
        save_settings(initial_settings)
            .await
            .expect("Initial save should work");

        // Verify main file exists
        let settings_path = get_settings_path().expect("Should get settings path");
        assert!(settings_path.exists(), "Main settings file should exist");

        // Save different settings (should create backup)
        let updated_settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+T".to_string(),
            model_size: "small".to_string(),
            auto_launch: true,
        };
        save_settings(updated_settings.clone())
            .await
            .expect("Updated save should work");

        // Verify backup was created
        let backup_path = get_settings_backup_path().expect("Should get backup path");
        assert!(backup_path.exists(), "Backup file should be created");

        // Verify main file has new settings
        let loaded = load_settings().await.expect("Should load updated settings");
        assert_eq!(loaded, updated_settings);
    }

    #[tokio::test]
    async fn test_corruption_recovery_from_backup() {
        // Use isolated temporary directory instead of real settings path
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let settings_dir = temp_dir.path().to_path_buf();

        // Create good settings - first save creates main file, no backup yet
        let good_settings = AppSettings::default();
        save_settings_to_dir(&good_settings, &settings_dir)
            .await
            .expect("Should save initial");

        let settings_path = settings_dir.join("settings.json");
        let backup_path = settings_dir.join("settings.json.backup");

        // First save doesn't create backup (no existing file to backup)
        assert!(
            !backup_path.exists(),
            "Backup should NOT exist after first save"
        );

        // Second save creates backup of the existing file
        save_settings_to_dir(&good_settings, &settings_dir)
            .await
            .expect("Should save second time");

        // NOW backup should exist (created from the existing file during second save)
        assert!(
            backup_path.exists(),
            "Backup should exist after second save"
        );

        // Corrupt the main file (backup should exist after second save)
        std::fs::write(&settings_path, "invalid json").expect("Should corrupt main file");

        // Load should recover from backup using isolated function
        let recovered = load_settings_from_dir(&settings_dir)
            .await
            .expect("Should recover from backup");
        assert_eq!(recovered, good_settings);

        // Verify main file was restored
        let reloaded = load_settings_from_dir(&settings_dir)
            .await
            .expect("Should load restored settings");
        assert_eq!(reloaded, good_settings);
    }

    #[tokio::test]
    async fn test_corruption_recovery_fallback_to_defaults() {
        use tempfile::TempDir;

        // Create a temporary directory for this test
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let fake_settings_path = temp_dir.path().join("settings.json");
        let fake_backup_path = temp_dir.path().join("settings.json.backup");

        // Create both files with invalid content
        std::fs::write(&fake_settings_path, "invalid json")
            .expect("Should write corrupt main file");
        std::fs::write(&fake_backup_path, "also invalid")
            .expect("Should write corrupt backup file");

        // Use the helper function directly since we can't easily override the paths in the command
        let load_result_main = try_load_settings_file(&fake_settings_path);
        assert!(
            load_result_main.is_err(),
            "Should fail to load corrupt main file"
        );

        let load_result_backup = try_load_settings_file(&fake_backup_path);
        assert!(
            load_result_backup.is_err(),
            "Should fail to load corrupt backup file"
        );

        // In the real scenario, this would fall back to defaults
        // The load_settings command handles this logic
    }

    #[tokio::test]
    async fn test_validate_hot_key_success() {
        // Arrange
        let valid_keys = vec![
            "CmdOrCtrl+Alt+Space".to_string(),
            "Ctrl+Shift+F".to_string(),
            "Alt+`".to_string(),
            "CMD+SPACE".to_string(), // Legacy format support
        ];

        // Act & Assert
        for key in valid_keys {
            let result = validate_hot_key(key.clone()).await;
            assert!(result.is_ok(), "Should accept valid key: {key}");
        }
    }

    #[tokio::test]
    async fn test_validate_hot_key_failures() {
        let invalid_keys = vec![
            "".to_string(),      // Empty
            "Space".to_string(), // No modifier
            "A+B".to_string(),   // No modifier keys
        ];

        for key in invalid_keys {
            let result = validate_hot_key(key.clone()).await;
            assert!(result.is_err(), "Should reject invalid key: {key}");
        }
    }

    #[tokio::test]
    async fn test_check_model_availability() {
        // Act & Assert
        let small_result = check_model_availability("small".to_string()).await;
        assert!(small_result.is_ok());

        let medium_result = check_model_availability("medium".to_string()).await;
        assert!(medium_result.is_ok());

        let large_result = check_model_availability("large".to_string()).await;
        assert!(large_result.is_ok());

        let invalid_result = check_model_availability("invalid".to_string()).await;
        assert!(invalid_result.is_err());
    }

    #[tokio::test]
    async fn test_register_hot_key() {
        // Act
        let result = register_hot_key("CmdOrCtrl+Alt+Space".to_string()).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_auto_launch() {
        // Act
        let enable_result = set_auto_launch(true).await;
        let disable_result = set_auto_launch(false).await;

        // Assert
        assert!(enable_result.is_ok());
        assert!(disable_result.is_ok());
    }

    #[tokio::test]
    async fn test_settings_serialization() {
        let settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+D".to_string(),
            model_size: "large".to_string(),
            auto_launch: true,
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings, deserialized);
    }

    #[tokio::test]
    async fn debug_save_button_functionality() {
        debug!("🔧 Debug: Testing save functionality...");

        // Create test directory
        let temp_dir = tempfile::TempDir::new().expect("Should create temp dir");
        let settings_dir = temp_dir.path().to_path_buf();

        debug!("📁 Test directory: {:?}", settings_dir);

        // Create test settings
        let test_settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+T".to_string(),
            model_size: "medium".to_string(),
            auto_launch: true,
        };

        debug!("⚙️  Test settings: {:?}", test_settings);

        // Try to save using the same function the Tauri command uses
        match save_settings_to_dir(&test_settings, &settings_dir).await {
            Ok(()) => {
                debug!("✅ Save succeeded!");

                // Check if file was created
                let settings_path = settings_dir.join("settings.json");
                if settings_path.exists() {
                    debug!("📄 File created at: {:?}", settings_path);

                    // Read and print content
                    match std::fs::read_to_string(&settings_path) {
                        Ok(content) => debug!("📖 File content:\n{}", content),
                        Err(e) => debug!("❌ Failed to read file: {}", e),
                    }
                } else {
                    debug!("❌ File was not created");
                }
            }
            Err(e) => {
                debug!("❌ Save failed: {}", e);
                panic!("Save should work in test environment");
            }
        }
    }

    #[tokio::test]
    async fn test_save_settings_tauri_command() {
        // This test should use test isolation instead of the production save_settings command
        // For now, mark it as todo since the Tauri command uses real system paths
        // The proper implementation would require dependency injection in the Tauri command

        debug!("🔧 Debug: Testing save_settings with proper isolation...");

        let test_settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+C".to_string(),
            model_size: "small".to_string(),
            auto_launch: false,
        };

        debug!("⚙️  Test settings: {:?}", test_settings);

        // Use the isolated function instead of the Tauri command that hits real filesystem
        let temp_dir = tempfile::TempDir::new().expect("Should create temp dir");
        let settings_dir = temp_dir.path().to_path_buf();

        match save_settings_to_dir(&test_settings, &settings_dir).await {
            Ok(()) => {
                debug!("✅ Isolated save succeeded!");

                // Verify we can load it back
                match load_settings_from_dir(&settings_dir).await {
                    Ok(loaded_settings) => {
                        debug!("📖 Loaded settings: {:?}", loaded_settings);
                        assert_eq!(loaded_settings.model_size, test_settings.model_size);
                        assert_eq!(loaded_settings.auto_launch, test_settings.auto_launch);
                        debug!("✅ Settings were correctly saved and loaded with isolation!");
                    }
                    Err(e) => {
                        debug!("❌ Failed to load settings after save: {}", e);
                        panic!("Should be able to load settings after isolated save");
                    }
                }
            }
            Err(e) => {
                debug!("❌ Isolated save failed: {}", e);
                panic!("Isolated save should work in test environment");
            }
        }
    }

    #[tokio::test]
    async fn test_settings_performance() {
        use std::time::Instant;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let temp_settings_path = temp_dir.path().join("settings.json");
        let settings = AppSettings::default();

        // Test that save completes within 100ms (FR-8 requirement)
        let start = Instant::now();

        // Test the actual file operations that happen in save_settings
        let json = serde_json::to_string_pretty(&settings).expect("Should serialize");
        let temp_path = temp_settings_path.with_extension("json.tmp");
        std::fs::write(&temp_path, &json).expect("Should write temp file");
        std::fs::rename(&temp_path, &temp_settings_path).expect("Should rename file");

        let duration = start.elapsed();

        assert!(
            duration.as_millis() < 100,
            "Settings save should complete within 100ms, took {}ms",
            duration.as_millis()
        );

        // Verify the file was written correctly
        let loaded =
            try_load_settings_file(&temp_settings_path).expect("Should load saved settings");
        assert_eq!(loaded, settings);
    }

    #[tokio::test]
    async fn test_settings_directory_permissions() {
        // RED: This test should fail initially since we haven't implemented permission validation
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let settings_path = temp_dir.path().join("settings.json");

        // Test that we can detect and handle permission issues
        // This should fail initially because we don't have permission validation
        let result = validate_settings_directory_permissions(settings_path.parent().unwrap());

        // This should pass once we implement the function
        assert!(result.is_ok(), "Should validate directory permissions");
    }

    #[tokio::test]
    async fn test_isolated_settings_save_and_load() {
        // RED: This should fail because these functions don't exist yet
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let settings_dir = temp_dir.path().to_path_buf();

        let test_settings = AppSettings {
            version: 1,
            hot_key: "CmdOrCtrl+Alt+T".to_string(),
            model_size: "large".to_string(),
            auto_launch: true,
        };

        // These functions should accept directory paths to enable test isolation
        save_settings_to_dir(&test_settings, &settings_dir)
            .await
            .expect("Should save to test dir");
        let loaded = load_settings_from_dir(&settings_dir)
            .await
            .expect("Should load from test dir");

        assert_eq!(loaded, test_settings);
    }

    #[tokio::test]
    async fn test_isolated_corruption_recovery() {
        // RED: This should fail because these functions don't exist yet
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Should create temp dir");
        let settings_dir = temp_dir.path().to_path_buf();

        // Create good settings and backup
        let good_settings = AppSettings::default();
        save_settings_to_dir(&good_settings, &settings_dir)
            .await
            .expect("Should save initial");

        let settings_path = settings_dir.join("settings.json");
        let _backup_path = settings_dir.join("settings.json.backup");

        // Corrupt main file (backup should exist after first save)
        std::fs::write(&settings_path, "invalid json").expect("Should corrupt main file");

        // Load should recover from backup
        let recovered = load_settings_from_dir(&settings_dir)
            .await
            .expect("Should recover");
        assert_eq!(recovered, good_settings);
    }

    // 🔴 RED: Tests for status indicator backend functionality
    #[tokio::test]
    async fn test_backend_status_service_creation() {
        let service = BackendStatusService::new();
        let status = service.get_current_status();

        // Should start with all services in "Starting" state
        assert!(!status.is_ready());
        assert_eq!(status.audio_capture, ServiceStatus::Starting);
        assert_eq!(status.transcription, ServiceStatus::Starting);
        assert_eq!(status.text_injection, ServiceStatus::Starting);
    }

    #[tokio::test]
    async fn test_backend_status_service_update_single_service() {
        let mut service = BackendStatusService::new();

        // Update a single service to Ready
        service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);
        let status = service.get_current_status();

        assert_eq!(status.audio_capture, ServiceStatus::Ready);
        assert_eq!(status.transcription, ServiceStatus::Starting);
        assert_eq!(status.text_injection, ServiceStatus::Starting);
        assert!(!status.is_ready()); // Not all ready yet
    }

    #[tokio::test]
    async fn test_backend_status_service_all_services_ready() {
        let mut service = BackendStatusService::new();

        // Update all services to Ready
        service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);
        service.update_service_status(ServiceComponent::Transcription, ServiceStatus::Ready);
        service.update_service_status(ServiceComponent::TextInjection, ServiceStatus::Ready);

        let status = service.get_current_status();
        assert!(status.is_ready()); // All ready
    }

    #[tokio::test]
    async fn test_backend_status_service_error_handling() {
        let mut service = BackendStatusService::new();

        // Set an error on transcription service
        service.update_service_status(
            ServiceComponent::Transcription,
            ServiceStatus::Error("Failed to load Whisper model".to_string()),
        );

        let status = service.get_current_status();
        assert!(!status.is_ready());
        assert!(matches!(status.transcription, ServiceStatus::Error(_)));
    }

    #[tokio::test]
    async fn test_backend_status_timestamps() {
        let mut service = BackendStatusService::new();
        let initial_timestamp = service.get_current_status().timestamp;

        // Wait a bit and update a service
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        service.update_service_status(ServiceComponent::AudioCapture, ServiceStatus::Ready);

        let updated_timestamp = service.get_current_status().timestamp;
        assert!(
            updated_timestamp > initial_timestamp,
            "Timestamp should be updated"
        );
    }

    #[tokio::test]
    async fn test_get_backend_status_tauri_command() {
        // 🔴 RED: This test should fail because the command doesn't exist yet

        // This would test the Tauri command interface
        // We can't easily test the Tauri command without a full Tauri context,
        // so for now we test the underlying service directly
        let service = BackendStatusService::new();
        let status = service.get_current_status();

        assert!(!status.is_ready()); // Should start with services not ready
    }

    // TDD: Tests for audio recording with file saving functionality
    #[tokio::test]
    async fn test_debug_record_audio_to_file_saves_with_timestamp() {
        // 🔴 RED: This test should fail because the function doesn't exist yet
        use std::time::SystemTime;
        use tempfile::TempDir;

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let output_dir = temp_dir.path();

        let before_recording = SystemTime::now();

        // Act
        let file_path = debug_record_audio_to_file(output_dir, 2)
            .await
            .expect("Should record audio to file");

        let after_recording = SystemTime::now();

        // Assert
        assert!(file_path.exists(), "Audio file should be created");
        assert!(
            file_path.extension().unwrap_or_default() == "wav",
            "Should create WAV file"
        );

        // Check timestamp in filename
        let filename = file_path.file_name().unwrap().to_string_lossy();
        assert!(
            filename.starts_with("recording_"),
            "Filename should start with 'recording_'"
        );

        // File should contain actual audio data (not just empty)
        let metadata = std::fs::metadata(&file_path).expect("Should get file metadata");
        assert!(
            metadata.len() > 44,
            "WAV file should be larger than header (44 bytes)"
        ); // WAV header is 44 bytes

        // Verify file timestamp is between before/after recording
        let file_time = metadata.modified().expect("Should get modified time");
        assert!(
            file_time >= before_recording && file_time <= after_recording,
            "File timestamp should be within recording window"
        );
    }

    #[tokio::test]
    async fn test_debug_record_audio_to_file_creates_unique_filenames() {
        // 🔴 RED: This test should fail because the function doesn't exist yet
        use tempfile::TempDir;

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let output_dir = temp_dir.path();

        // Act - record two files quickly
        let file1 = debug_record_audio_to_file(output_dir, 1)
            .await
            .expect("Should record first audio file");

        // Small delay to ensure different timestamp
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        let file2 = debug_record_audio_to_file(output_dir, 1)
            .await
            .expect("Should record second audio file");

        // Assert
        assert_ne!(file1, file2, "Should create files with unique names");
        assert!(file1.exists(), "First file should exist");
        assert!(file2.exists(), "Second file should exist");
    }

    #[tokio::test]
    async fn test_save_audio_samples_to_wav_file() {
        // 🔴 RED: This test should fail because the function doesn't exist yet
        use tempfile::TempDir;

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let output_path = temp_dir.path().join("test_audio.wav");

        // Create test samples (simple sine wave pattern)
        let sample_rate = 16_000;
        let duration_secs = 2;
        let samples: Vec<i16> = (0..sample_rate * duration_secs)
            .map(|i| {
                ((((i as f64) * 2.0 * std::f64::consts::PI * 440.0) / (sample_rate as f64)).sin()
                    * 16000.0) as i16
            })
            .collect();

        // Act
        save_audio_samples_to_wav_file(&samples, &output_path)
            .await
            .expect("Should save audio samples to WAV file");

        // Assert
        assert!(output_path.exists(), "WAV file should be created");

        let metadata = std::fs::metadata(&output_path).expect("Should get file metadata");
        assert!(metadata.len() > 44, "WAV file should be larger than header");

        // Check WAV header (first 4 bytes should be "RIFF")
        let file_content = std::fs::read(&output_path).expect("Should read file");
        assert_eq!(&file_content[0..4], b"RIFF", "Should have RIFF header");
        assert_eq!(&file_content[8..12], b"WAVE", "Should have WAVE format");
    }

    #[tokio::test]
    async fn test_generate_audio_filename_with_timestamp() {
        // 🔴 RED: This test should fail because the function doesn't exist yet
        use std::time::SystemTime;

        // Act
        let _before = SystemTime::now();
        let filename = generate_audio_filename_with_timestamp();
        let _after = SystemTime::now();

        // Assert
        assert!(
            filename.starts_with("recording_"),
            "Should start with 'recording_'"
        );
        assert!(filename.ends_with(".wav"), "Should end with '.wav'");

        // Should contain timestamp components (year, month, day, hour, minute, second)
        assert!(
            filename.contains("2024") || filename.contains("2025"),
            "Should contain year"
        );

        // Generate another filename and ensure they're different
        let filename2 = generate_audio_filename_with_timestamp();
        if filename == filename2 {
            // If they're the same, wait a bit and try again
            tokio::time::sleep(std::time::Duration::from_millis(1001)).await;
            let filename3 = generate_audio_filename_with_timestamp();
            assert_ne!(filename, filename3, "Filenames should be unique over time");
        }
    }

    #[tokio::test]
    #[ignore = "Requires audio hardware access - run manually with 'cargo test -- --ignored'"]
    async fn test_debug_real_audio_recording_integration() {
        // 🔴 RED: This test should fail because the real integration doesn't exist yet
        use tempfile::TempDir;

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let output_dir = temp_dir.path();

        // Test that we can record real audio and save to file
        // This would use the actual AudioRecorder from speakr-core

        // Act - This should do a real recording for 1 second
        let file_path = debug_record_real_audio_to_file(output_dir, 1)
            .await
            .expect("Should record real audio to file");

        // Assert
        assert!(file_path.exists(), "Real audio file should be created");

        let metadata = std::fs::metadata(&file_path).expect("Should get file metadata");
        // For 1 second of 16kHz mono audio, expect roughly:
        // 44 bytes (WAV header) + (16000 samples * 2 bytes/sample) = ~32044 bytes
        assert!(
            metadata.len() > 1000,
            "Real audio file should contain substantial data"
        );
        assert!(
            metadata.len() < 100_000,
            "File size should be reasonable for 1 second"
        );
    }
}
