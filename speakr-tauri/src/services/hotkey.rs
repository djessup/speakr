//! Global hotkey service implementation.

use speakr_types::{AppError, HotkeyConfig, HotkeyError};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use tracing::{debug, info};

/// Service responsible for managing global hot-keys
pub(crate) struct GlobalHotkeyService {
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
    pub(crate) fn new(app_handle: AppHandle) -> Result<Self, HotkeyError> {
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
    pub(crate) async fn register_hotkey(
        &mut self,
        config: &HotkeyConfig,
    ) -> Result<(), HotkeyError> {
        // If hotkey is disabled, succeed but don't actually register
        if !config.enabled {
            return Ok(());
        }

        // 🟢 GREEN: Use Tauri's native shortcut parsing instead of custom implementation
        let shortcut = config.shortcut.parse::<Shortcut>().map_err(|e| {
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
    pub(crate) async fn unregister_hotkey(&mut self) -> Result<(), HotkeyError> {
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
}

/// Internal hot-key validation logic using Tauri's native shortcut parsing.
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
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub async fn validate_hot_key_internal(hot_key: String) -> Result<(), AppError> {
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

    // 🟢 GREEN: Use Tauri's native shortcut parsing instead of custom logic
    // This supports function keys (F1-F12), special keys, numeric keys, and all standard keys
    match hot_key.parse::<Shortcut>() {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::HotKey(format!("Invalid shortcut format: {e}"))),
    }
}

/// Register a global hotkey using the GlobalHotkeyService
pub async fn register_global_hotkey_internal(
    app_handle: AppHandle,
    config: HotkeyConfig,
) -> Result<(), String> {
    let mut service = GlobalHotkeyService::new(app_handle).map_err(|e| e.to_string())?;
    service
        .register_hotkey(&config)
        .await
        .map_err(|e| e.to_string())
}

/// Unregister the current global hotkey using the GlobalHotkeyService
pub async fn unregister_global_hotkey_internal(app_handle: AppHandle) -> Result<(), String> {
    let mut service = GlobalHotkeyService::new(app_handle).map_err(|e| e.to_string())?;
    service.unregister_hotkey().await.map_err(|e| e.to_string())
}
