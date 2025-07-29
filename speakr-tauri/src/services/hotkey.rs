// ============================================================================
//! Global Hotkey Service
// ============================================================================

use speakr_types::{HotkeyConfig, HotkeyError};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tracing::{debug, info};

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
    pub(crate) fn new(app_handle: AppHandle) -> Result<Self, HotkeyError> {
        Ok(Self {
            app_handle,
            current_shortcut: Arc::new(Mutex::new(None)),
            current_shortcut_instance: Arc::new(Mutex::new(None)),
        })
    }

    /// Determines whether the given shortcut event should trigger the
    /// application workflow.  At the moment we are only interested in the
    /// *Pressed* state (the initial key-down).  Filtering here prevents the
    /// *Released* state from causing a duplicate invocation.
    #[inline]
    pub fn should_handle_hotkey_event(state: ShortcutState) -> bool {
        state == ShortcutState::Pressed
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

        // Use Tauri's native shortcut parsing
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
            .on_shortcut(shortcut, move |_app, _shortcut, event| {
                // Only react to the key *press* event; ignore the release to
                // prevent duplicate workflow invocations.
                if Self::should_handle_hotkey_event(event.state()) {
                    // Emit an event when the hotkey is triggered
                    let _ = app_handle_clone.emit("hotkey-triggered", ());

                    // TODO: Wire this to speakr-core pipeline in next step
                    debug!("Global hotkey triggered");
                }
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

/// Update the global hotkey by unregistering the current one and registering a new one
pub async fn update_global_hotkey_internal(
    app_handle: AppHandle,
    config: HotkeyConfig,
) -> Result<(), String> {
    let mut service = GlobalHotkeyService::new(app_handle).map_err(|e| e.to_string())?;

    // The register_hotkey method already handles unregistering existing shortcuts
    // so we can just call it directly
    service
        .register_hotkey(&config)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_handle_hotkey_event() {
        assert!(GlobalHotkeyService::should_handle_hotkey_event(
            ShortcutState::Pressed
        ));
    }

    // ============================================================================
    // Duplicate Invocation Regression Test
    // ============================================================================

    /// [`ShortcutState`] supplied by the `tauri_plugin_global_shortcut` callback
    /// for *Pressed* **and** *Released* events
    /// A single physical shortcut press should only trigger the workflow once.
    #[test]
    fn test_single_hotkey_press_triggers_once() {
        // Simulate the plugin callback firing for both *Pressed* and *Released*.
        let states = vec![ShortcutState::Pressed, ShortcutState::Released];

        let workflow_invocations = states
            .into_iter()
            .filter(|s| GlobalHotkeyService::should_handle_hotkey_event(*s))
            .count();

        // Assert – only the *Pressed* state should be counted as a valid trigger.
        assert_eq!(
            workflow_invocations, 1,
            "Exactly one workflow invocation expected on key press"
        );
    }
}
