//! Speakr Tauri backend module.
//!
//! This module provides the Tauri commands and backend functionality for the Speakr
//! dictation application, including:
//! - Settings management and persistence
//! - Global hot-key registration using tauri-plugin-global-shortcut
//! - Model file validation
//! - System integration

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};
use thiserror::Error;

/// Unified error type for all Tauri backend operations.
/// This matches the error variants expected by the frontend.
#[derive(Error, Debug, Serialize)]
pub enum TauriError {
    /// Settings operation failed
    #[error("Settings error: {0}")]
    Settings(String),

    /// File system operation failed
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Hot-key registration or validation failed
    #[error("Hot-key error: {0}")]
    HotKey(String),

    /// Hot-key conflict detected
    #[error("Hot-key conflict: {0}")]
    HotKeyConflict(String),

    /// Hot-key not found
    #[error("Hot-key not found: {0}")]
    HotKeyNotFound(String),

    /// General Tauri command error
    #[error("Tauri command error: {0}")]
    TauriError(String),
}

/// Error type for global hot-key related operations
#[derive(Debug, thiserror::Error)]
pub enum GlobalHotkeyError {
    #[error("Failed to register global hot-key: {0}")]
    RegistrationFailed(String),
    #[error("Hot-key conflict detected: {0}")]
    ConflictDetected(String),
    #[error("Hot-key not found: {0}")]
    NotFound(String),
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),
}

/// Configuration for global hot-key settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotkeyConfig {
    pub shortcut: String,
    pub enabled: bool,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            shortcut: "CmdOrCtrl+Alt+Space".to_string(),
            enabled: true,
        }
    }
}

/// Unified settings data structure - matches frontend AppSettings exactly.
/// This ensures consistency between frontend and backend representations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    /// Global hot-key combination in Tauri v2 format (e.g., "CmdOrCtrl+Alt+Space")
    pub hot_key: String,
    /// Selected model size ("small", "medium", "large")
    pub model_size: String,
    /// Whether to auto-launch the app on system startup
    pub auto_launch: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            // Use CmdOrCtrl for cross-platform compatibility - matches frontend
            hot_key: "CmdOrCtrl+Alt+Space".to_string(),
            model_size: "medium".to_string(),
            auto_launch: false,
        }
    }
}

/// Gets the settings file path in the app data directory.
fn get_settings_path() -> Result<PathBuf, TauriError> {
    let app_data = dirs::config_dir()
        .ok_or_else(|| TauriError::Settings("Could not find config directory".to_string()))?;

    let speakr_dir = app_data.join("speakr");
    if !speakr_dir.exists() {
        fs::create_dir_all(&speakr_dir)
            .map_err(|e| TauriError::FileSystem(format!("Failed to create config dir: {e}")))?;
    }

    Ok(speakr_dir.join("settings.json"))
}

/// Saves application settings to disk.
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
/// Returns `TauriError` if the settings cannot be saved.
#[tauri::command]
async fn save_settings(settings: AppSettings) -> Result<(), TauriError> {
    let settings_path = get_settings_path()?;

    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| TauriError::Settings(format!("Failed to serialize settings: {e}")))?;

    fs::write(&settings_path, json)
        .map_err(|e| TauriError::FileSystem(format!("Failed to write settings file: {e}")))?;

    Ok(())
}

/// Loads application settings from disk.
///
/// # Returns
///
/// Returns the loaded settings or default settings if the file doesn't exist.
///
/// # Errors
///
/// Returns `TauriError` if the settings file exists but cannot be read or parsed.
#[tauri::command]
async fn load_settings() -> Result<AppSettings, TauriError> {
    let settings_path = get_settings_path()?;

    if !settings_path.exists() {
        return Ok(AppSettings::default());
    }

    let content = fs::read_to_string(&settings_path)
        .map_err(|e| TauriError::FileSystem(format!("Failed to read settings file: {e}")))?;

    let settings: AppSettings = serde_json::from_str(&content)
        .map_err(|e| TauriError::Settings(format!("Failed to parse settings: {e}")))?;

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
    /// Returns `GlobalHotkeyError` if service initialization fails
    pub fn new(app_handle: AppHandle) -> Result<Self, GlobalHotkeyError> {
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
    /// Returns `GlobalHotkeyError::RegistrationFailed` if registration fails
    /// Returns `GlobalHotkeyError::ConflictDetected` if the hot-key is already in use
    pub async fn register_hotkey(
        &mut self,
        config: &HotkeyConfig,
    ) -> Result<(), GlobalHotkeyError> {
        // If hotkey is disabled, succeed but don't actually register
        if !config.enabled {
            return Ok(());
        }

        // Parse the shortcut string into Tauri's format
        let shortcut = self.parse_shortcut(&config.shortcut).map_err(|e| {
            GlobalHotkeyError::RegistrationFailed(format!("Invalid shortcut format: {e}"))
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
                println!("Global hotkey triggered");
            })
            .map_err(|e| {
                GlobalHotkeyError::ConflictDetected(format!("Failed to register shortcut: {e}"))
            })?;

        // Register the shortcut for system-wide listening
        self.app_handle
            .global_shortcut()
            .register(shortcut)
            .map_err(|e| {
                GlobalHotkeyError::ConflictDetected(format!(
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

        println!("Successfully registered global hotkey: {}", config.shortcut);
        Ok(())
    }

    /// Unregisters the currently registered hot-key
    ///
    /// # Errors
    ///
    /// Returns `GlobalHotkeyError::NotFound` if no hot-key is currently registered
    pub async fn unregister_hotkey(&mut self) -> Result<(), GlobalHotkeyError> {
        let mut current_instance = self.current_shortcut_instance.lock().map_err(|_| {
            GlobalHotkeyError::RegistrationFailed("Failed to acquire lock".to_string())
        })?;

        if let Some(shortcut) = current_instance.take() {
            self.app_handle
                .global_shortcut()
                .unregister(shortcut)
                .map_err(|e| {
                    GlobalHotkeyError::RegistrationFailed(format!(
                        "Failed to unregister shortcut: {e}"
                    ))
                })?;

            // Clear current shortcut
            if let Ok(mut current) = self.current_shortcut.lock() {
                *current = None;
            }

            println!("Successfully unregistered global hotkey");
            Ok(())
        } else {
            Err(GlobalHotkeyError::NotFound(
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
    /// Returns `GlobalHotkeyError` if the update fails
    pub async fn update_hotkey(
        &mut self,
        new_config: &HotkeyConfig,
    ) -> Result<(), GlobalHotkeyError> {
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
/// Returns `TauriError::HotKey` if the hot-key is invalid.
#[tauri::command]
async fn validate_hot_key(hot_key: String) -> Result<(), TauriError> {
    if hot_key.is_empty() {
        return Err(TauriError::HotKey("Hot-key cannot be empty".to_string()));
    }

    // Enhanced validation - supports both old and new formats
    let modifiers = ["CMD", "CMDORCTRL", "CTRL", "ALT", "OPTION", "SHIFT"];
    let has_modifier = modifiers.iter().any(|m| hot_key.to_uppercase().contains(m));

    if !has_modifier {
        return Err(TauriError::HotKey(
            "Hot-key must contain at least one modifier key".to_string(),
        ));
    }

    // Test if the shortcut can be parsed using the same logic as GlobalHotkeyService
    let parts: Vec<&str> = hot_key.split('+').collect();
    if parts.is_empty() {
        return Err(TauriError::HotKey("Empty shortcut string".to_string()));
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
                        return Err(TauriError::HotKey(format!("Unsupported key: {part}")));
                    }
                }
            }
        }
    }

    if !has_key {
        return Err(TauriError::HotKey(
            "No key specified in shortcut".to_string(),
        ));
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
async fn check_model_availability(model_size: String) -> Result<bool, TauriError> {
    let filename = match model_size.as_str() {
        "small" => "ggml-small.bin",
        "medium" => "ggml-medium.bin",
        "large" => "ggml-large.bin",
        _ => {
            return Err(TauriError::Settings(format!(
                "Unknown model size: {model_size}"
            )));
        }
    };

    // Check in models directory relative to the app
    let models_dir = std::env::current_dir()
        .map_err(|e| TauriError::FileSystem(format!("Failed to get current dir: {e}")))?
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
/// Returns `TauriError::HotKey` if registration fails.
#[tauri::command]
async fn register_hot_key(hot_key: String) -> Result<(), TauriError> {
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
/// Returns `TauriError` if the operation fails.
#[tauri::command]
async fn set_auto_launch(enable: bool) -> Result<(), TauriError> {
    // TODO: Implement actual auto-launch registration using system APIs
    // For now, just return success
    let _ = enable;
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_settings,
            load_settings,
            validate_hot_key,
            check_model_availability,
            register_hot_key,
            set_auto_launch,
            register_global_hotkey,
            unregister_global_hotkey
        ])
        .setup(|app| {
            // Register default hotkey on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let default_config = HotkeyConfig::default();
                if let Ok(mut service) = GlobalHotkeyService::new(app_handle) {
                    if let Err(e) = service.register_hotkey(&default_config).await {
                        eprintln!("Failed to register default hotkey: {e}");
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

    #[tokio::test]
    async fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.hot_key, "CmdOrCtrl+Alt+Space");
        assert_eq!(settings.model_size, "medium");
        assert!(!settings.auto_launch);
    }

    #[tokio::test]
    async fn test_save_and_load_settings() {
        // Arrange
        let test_settings = AppSettings {
            hot_key: "CmdOrCtrl+Alt+S".to_string(),
            model_size: "large".to_string(),
            auto_launch: true,
        };

        // Act - Save settings
        let save_result = save_settings(test_settings.clone()).await;
        assert!(save_result.is_ok(), "Should save settings successfully");

        // Act - Load settings
        let load_result = load_settings().await;
        assert!(load_result.is_ok(), "Should load settings successfully");

        // Assert
        let loaded_settings = load_result.unwrap();
        assert_eq!(loaded_settings, test_settings);
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
            hot_key: "CmdOrCtrl+Alt+D".to_string(),
            model_size: "large".to_string(),
            auto_launch: true,
        };

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(settings, deserialized);
    }
}
