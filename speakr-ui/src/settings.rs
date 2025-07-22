//! Settings UI module for Speakr application.
//!
//! This module provides the Settings Panel component for configuring:
//! - Global hot-key combinations
//! - Whisper model selection (small, medium, large)
//! - Auto-launch on system startup
//! - Settings persistence via Tauri commands and local storage
//!
//! All settings management follows Tauri v2 plugin architecture with
//! @tauri-apps/plugin-global-shortcut for hot-key functionality.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use speakr_types::{AppSettings, ModelSize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// External bindings to Tauri APIs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Helper function to invoke Tauri commands
async fn tauri_invoke<T: for<'de> Deserialize<'de>, U: Serialize>(
    cmd: &str,
    args: &U,
) -> Result<T, String> {
    let js_args =
        serde_wasm_bindgen::to_value(args).map_err(|e| format!("Failed to serialize args: {e}"))?;

    let result = invoke(cmd, js_args).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Failed to deserialize result: {e}"))
}

/// Helper function for commands without arguments
async fn tauri_invoke_no_args<T: for<'de> Deserialize<'de>>(cmd: &str) -> Result<T, String> {
    let result = invoke(cmd, JsValue::NULL).await;

    serde_wasm_bindgen::from_value(result).map_err(|e| format!("Failed to deserialize result: {e}"))
}

/// Error type for settings operations
pub type SettingsError = String;

// All types now centralized in speakr-types crate

/// Settings manager that handles persistence and Tauri integration.
pub struct SettingsManager;

impl SettingsManager {
    /// Loads settings from the backend
    pub async fn load() -> Result<AppSettings, SettingsError> {
        tauri_invoke_no_args("load_settings")
            .await
            .map_err(|e| format!("Failed to load settings: {e}"))
    }

    /// Saves settings to the backend
    pub async fn save(settings: &AppSettings) -> Result<(), SettingsError> {
        web_sys::console::log_1(&"🔧 SettingsManager::save called".into());
        web_sys::console::log_1(
            &format!("📤 Invoking Tauri command 'save_settings' with: {settings:?}").into(),
        );

        // Pass settings directly to match the Tauri command signature
        match tauri_invoke::<(), _>("save_settings", settings).await {
            Ok(result) => {
                web_sys::console::log_1(
                    &"✅ Tauri command 'save_settings' returned successfully".into(),
                );
                Ok(result)
            }
            Err(e) => {
                web_sys::console::error_1(
                    &format!("❌ Tauri command 'save_settings' failed: {e}").into(),
                );
                Err(format!("Failed to save settings: {e}"))
            }
        }
    }

    /// Validates a hot-key combination
    pub async fn validate_hot_key(hot_key: &str) -> Result<(), SettingsError> {
        // Pass the hot_key string directly to match the Tauri command signature
        tauri_invoke::<(), _>("validate_hot_key", &hot_key.to_string())
            .await
            .map_err(|e| format!("Invalid hot-key: {e}"))
    }

    /// Checks model availability
    pub async fn check_model_availability(model_size: &str) -> Result<bool, SettingsError> {
        // Pass the model_size string directly to match the Tauri command signature
        tauri_invoke("check_model_availability", &model_size.to_string()).await
    }

    /// Sets auto-launch preference
    pub async fn set_auto_launch(enable: bool) -> Result<(), SettingsError> {
        // Pass the boolean directly to match the Tauri command signature
        tauri_invoke::<(), _>("set_auto_launch", &enable).await
    }
}

/// Global shortcut manager using Tauri v2 plugin APIs.
///
/// This manager handles registration/unregistration of global shortcuts
/// using the @tauri-apps/plugin-global-shortcut JavaScript APIs.
pub struct GlobalShortcutManager;

impl GlobalShortcutManager {
    /// Registers a global shortcut using the Tauri v2 plugin
    pub async fn register(shortcut: &str) -> Result<(), SettingsError> {
        let result = js_sys::eval(&format!(
            r#"
            (async () => {{
                try {{
                    const {{ register }} = await import("@tauri-apps/plugin-global-shortcut");
                    await register("{shortcut}", (event) => {{
                        console.log("Global shortcut triggered:", event);
                        // TODO: Wire to speakr-core pipeline
                    }});
                    return {{ success: true }};
                }} catch (error) {{
                    return {{ success: false, error: error.message }};
                }}
            }})()
            "#
        ));

        match result {
            Ok(promise) => {
                let promise: js_sys::Promise = promise.into();
                let result = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|e| format!("Hot-key conflict: Failed to register shortcut: {e:?}"))?;

                // Check if registration was successful
                let success = js_sys::Reflect::get(&result, &JsValue::from("success"))
                    .map_err(|_| "Command error: Invalid response format")?;

                if success.as_bool() == Some(true) {
                    Ok(())
                } else {
                    let error = js_sys::Reflect::get(&result, &JsValue::from("error"))
                        .unwrap_or_else(|_| JsValue::from("Unknown error"));
                    Err(format!("Hot-key conflict: Registration failed: {error:?}"))
                }
            }
            Err(e) => Err(format!(
                "Command error: Failed to execute registration: {e:?}"
            )),
        }
    }

    /// Unregisters a global shortcut
    pub async fn unregister(shortcut: &str) -> Result<(), SettingsError> {
        let result = js_sys::eval(&format!(
            r#"
            (async () => {{
                try {{
                    const {{ unregister }} = await import("@tauri-apps/plugin-global-shortcut");
                    await unregister("{shortcut}");
                    return {{ success: true }};
                }} catch (error) {{
                    return {{ success: false, error: error.message }};
                }}
            }})()
            "#
        ));

        match result {
            Ok(promise) => {
                let promise: js_sys::Promise = promise.into();
                let _result = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|e| format!("Command error: Failed to unregister shortcut: {e:?}"))?;
                Ok(())
            }
            Err(e) => Err(format!(
                "Command error: Failed to execute unregistration: {e:?}"
            )),
        }
    }

    /// Unregisters all global shortcuts
    #[allow(dead_code)]
    pub async fn unregister_all() -> Result<(), SettingsError> {
        let result = js_sys::eval(
            r#"
            (async () => {
                try {
                    const { unregisterAll } = await import("@tauri-apps/plugin-global-shortcut");
                    await unregisterAll();
                    return { success: true };
                } catch (error) {
                    return { success: false, error: error.message };
                }
            })()
            "#,
        );

        match result {
            Ok(promise) => {
                let promise: js_sys::Promise = promise.into();
                let _result = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|e| {
                        format!("Command error: Failed to unregister all shortcuts: {e:?}")
                    })?;
                Ok(())
            }
            Err(e) => Err(format!(
                "Command error: Failed to execute unregister all: {e:?}"
            )),
        }
    }
}

/// Main Settings Panel component for the Speakr application.
///
/// This component provides a comprehensive interface for:
/// - Hot-key configuration with real-time validation
/// - Model selection with availability checking
/// - Auto-launch toggle with system integration
/// - Real-time settings persistence
#[component]
pub fn SettingsPanel() -> impl IntoView {
    // Settings state
    let (settings, set_settings) = signal(AppSettings::default());
    let (loading, set_loading) = signal(true);
    let (error_message, set_error_message) = signal::<Option<String>>(None);
    let (success_message, set_success_message) = signal::<Option<String>>(None);

    // Hot-key editing state
    let (editing_hotkey, set_editing_hotkey) = signal(false);
    let (temp_hotkey, set_temp_hotkey) = signal(String::new());
    let (hotkey_valid, set_hotkey_valid) = signal(true);

    // Model availability state
    let (model_availability, set_model_availability) =
        signal(std::collections::HashMap::<String, bool>::new());

    // Load settings on mount
    Effect::new(move || {
        spawn_local(async move {
            match SettingsManager::load().await {
                Ok(loaded_settings) => {
                    set_settings.set(loaded_settings);
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to load settings: {e}")));
                    set_loading.set(false);
                }
            }
        });
    });

    // Check model availability when settings change
    Effect::new(move || {
        let _current_settings = settings.get();
        spawn_local(async move {
            let model_sizes = vec!["small", "medium", "large"];
            let mut availability = std::collections::HashMap::new();

            for size in model_sizes {
                match SettingsManager::check_model_availability(size).await {
                    Ok(available) => {
                        availability.insert(size.to_string(), available);
                    }
                    Err(_) => {
                        availability.insert(size.to_string(), false);
                    }
                }
            }

            set_model_availability.set(availability);
        });
    });

    // Save settings function
    let save_settings = move || {
        spawn_local(async move {
            // Add debug logging
            web_sys::console::log_1(&"🔧 Save button clicked - attempting to save settings".into());

            let current_settings = settings.get();
            web_sys::console::log_1(&format!("📄 Settings to save: {current_settings:?}").into());

            match SettingsManager::save(&current_settings).await {
                Ok(_) => {
                    web_sys::console::log_1(&"✅ Backend save succeeded!".into());
                    set_success_message.set(Some("Settings saved successfully!".to_string()));
                    set_error_message.set(None);

                    // Clear success message after 3 seconds
                    spawn_local(async move {
                        gloo_timers::future::TimeoutFuture::new(3000).await;
                        set_success_message.set(None);
                    });
                }
                Err(e) => {
                    let error_msg = format!("Failed to save settings: {e}");
                    web_sys::console::error_1(
                        &format!("❌ Backend save failed: {error_msg}").into(),
                    );
                    set_error_message.set(Some(error_msg));
                    set_success_message.set(None);
                }
            }
        });
    };

    // Start editing hot-key
    let start_editing_hotkey = move || {
        set_temp_hotkey.set(settings.get().hot_key.clone());
        set_editing_hotkey.set(true);
        set_error_message.set(None);

        // Unregister current shortcut while editing
        let current_hotkey = settings.get().hot_key.clone();
        spawn_local(async move {
            let _ = GlobalShortcutManager::unregister(&current_hotkey).await;
        });
    };

    // Cancel editing hot-key
    let cancel_editing_hotkey = move || {
        set_editing_hotkey.set(false);
        set_temp_hotkey.set(String::new());
        set_hotkey_valid.set(true);

        // Re-register the original shortcut
        let original_hotkey = settings.get().hot_key.clone();
        spawn_local(async move {
            let _ = GlobalShortcutManager::register(&original_hotkey).await;
        });
    };

    // Save hot-key changes
    let save_hotkey = move || {
        let new_hotkey = temp_hotkey.get();

        spawn_local(async move {
            // Validate the new hot-key
            match SettingsManager::validate_hot_key(&new_hotkey).await {
                Ok(_) => {
                    // Try to register the new shortcut
                    match GlobalShortcutManager::register(&new_hotkey).await {
                        Ok(_) => {
                            // Update settings and save
                            set_settings.update(|s| {
                                s.hot_key = new_hotkey.clone();
                            });
                            set_editing_hotkey.set(false);
                            set_temp_hotkey.set(String::new());
                            set_hotkey_valid.set(true);

                            match SettingsManager::save(&settings.get()).await {
                                Ok(_) => {
                                    set_success_message
                                        .set(Some("Hot-key updated successfully!".to_string()));
                                    set_error_message.set(None);
                                }
                                Err(e) => {
                                    set_error_message.set(Some(format!(
                                        "Hot-key registered but failed to save: {e}"
                                    )));
                                }
                            }
                        }
                        Err(e) => {
                            set_error_message.set(Some(format!("Failed to register hot-key: {e}")));
                            set_hotkey_valid.set(false);
                        }
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Invalid hot-key: {e}")));
                    set_hotkey_valid.set(false);
                }
            }
        });
    };

    view! {
        <div class="settings-panel">
            <div class="settings-header">
                <h2>"Settings"</h2>
                <p class="setting-description">"Configure Speakr for your perfect dictation experience"</p>
                {move || loading.get().then(|| view! {
                    <div class="loading-indicator">
                        <div class="spinner"></div>
                        <span>"Loading settings..."</span>
                    </div>
                })}
            </div>

            {move || {
                if let Some(error) = error_message.get() {
                    view! {
                        <div class="error-message">
                            <span>"⚠️"</span>
                            <span>{error}</span>
                        </div>
                    }.into_any()
                } else if let Some(success) = success_message.get() {
                    view! {
                        <div class="success-message">
                            <span>"✅"</span>
                            <span>{success}</span>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            <div class="settings-content">
                // Hot-key Configuration Section
                <div class="setting-group">
                    <h3>"⌨️ Global Hot-key"</h3>
                    <p class="setting-description">
                        "Keyboard shortcut to activate Speakr from anywhere on your system. Press this combination to start dictating."
                    </p>

                    <div class="hotkey-section">
                        {move || {
                            if editing_hotkey.get() {
                                view! {
                                    <div class="hotkey-editor">
                                        <input
                                            type="text"
                                            class={move || format!("hotkey-input {}", if hotkey_valid.get() { "" } else { "invalid" })}
                                            placeholder="e.g., CmdOrCtrl+Alt+Space"
                                            prop:value={move || temp_hotkey.get()}
                                            on:input=move |e| {
                                                set_temp_hotkey.set(event_target_value(&e));
                                                set_hotkey_valid.set(true);
                                            }
                                        />
                                        <div class="hotkey-actions">
                                            <button
                                                class="btn-primary"
                                                on:click=move |_| save_hotkey()
                                                disabled={move || temp_hotkey.get().is_empty()}
                                            >
                                                "💾 Save"
                                            </button>
                                            <button
                                                class="btn-secondary"
                                                on:click=move |_| cancel_editing_hotkey()
                                            >
                                                "❌ Cancel"
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="hotkey-display">
                                        <div class="hotkey-value">
                                            <code>{move || settings.get().hot_key}</code>
                                        </div>
                                        <button
                                            class="btn-secondary"
                                            on:click=move |_| start_editing_hotkey()
                                        >
                                            "✏️ Edit"
                                        </button>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                // Model Selection Section
                <div class="setting-group">
                    <h3>"🧠 Transcription Model"</h3>
                    <p class="setting-description">
                        "Choose the Whisper model size based on your accuracy and speed preferences. Larger models are more accurate but require more resources."
                    </p>

                    <div class="model-options">
                        {move || {
                            let model_sizes = vec![ModelSize::Small, ModelSize::Medium, ModelSize::Large];
                            let current_model = &settings.get().model_size;
                            let availability = model_availability.get();

                            model_sizes.into_iter().map(|model| {
                                let model_key = model.to_string_value();
                                let is_selected = current_model == model_key;
                                let is_available = availability.get(model_key).unwrap_or(&false);

                                let (icon, description) = match model {
                                    ModelSize::Small => ("⚡", "Fast processing, good for quick notes"),
                                    ModelSize::Medium => ("⚖️", "Balanced accuracy and speed"),
                                    ModelSize::Large => ("🎯", "Highest accuracy, best for professional use"),
                                };

                                view! {
                                    <div class={format!("model-option {} {}",
                                        if is_selected { "selected" } else { "" },
                                        if *is_available { "available" } else { "unavailable" }
                                    )}>
                                        <input
                                            type="radio"
                                            name="model_size"
                                            id={format!("model_{model_key}")}
                                            value={model_key}
                                            checked=is_selected
                                            disabled={!is_available}
                                            on:change=move |_| {
                                                set_settings.update(|s| s.model_size = model_key.to_string());
                                                save_settings();
                                            }
                                        />
                                        <label for={format!("model_{model_key}")} class="model-label">
                                            <div class="model-info">
                                                <div class="model-header">
                                                    <span class="model-icon">{icon}</span>
                                                    <div class="model-name">{model.display_name()}</div>
                                                </div>
                                                <div class="model-description">{description}</div>
                                            </div>
                                            <div class="model-status">
                                                {if *is_available {
                                                    "✅ Available"
                                                } else {
                                                    "❌ Not Downloaded"
                                                }}
                                            </div>
                                        </label>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                    </div>
                </div>

                // Auto-launch Section
                <div class="setting-group">
                    <h3>"🚀 Auto-launch"</h3>
                    <p class="setting-description">
                        "Automatically start Speakr when you log in to your computer, so it's always ready when you need it."
                    </p>

                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            class="auto-launch-checkbox"
                            checked={move || settings.get().auto_launch}
                            on:change=move |e| {
                                let enabled = event_target_checked(&e);
                                set_settings.update(|s| s.auto_launch = enabled);

                                spawn_local(async move {
                                    match SettingsManager::set_auto_launch(enabled).await {
                                        Ok(_) => save_settings(),
                                        Err(e) => {
                                            set_error_message.set(Some(format!("Failed to update auto-launch: {e}")));
                                        }
                                    }
                                });
                            }
                        />
                        <div class="checkbox-content">
                            <span class="checkbox-label-text">"Start Speakr on login"</span>
                            <span class="checkbox-help">"Recommended for seamless workflow integration"</span>
                        </div>
                    </label>
                </div>

                // Quick Tips Section
                <div class="setting-group">
                    <h3>"💡 Quick Tips"</h3>
                    <div class="tips-list">
                        <div class="tip-item">
                            <span class="tip-icon">"🎙️"</span>
                            <div class="tip-content">
                                <div class="tip-title">"Clear Audio"</div>
                                <div class="tip-description">"Speak clearly and reduce background noise for better accuracy"</div>
                            </div>
                        </div>
                        <div class="tip-item">
                            <span class="tip-icon">"⏸️"</span>
                            <div class="tip-content">
                                <div class="tip-title">"Natural Pauses"</div>
                                <div class="tip-description">"Pause briefly between sentences for better punctuation"</div>
                            </div>
                        </div>
                        <div class="tip-item">
                            <span class="tip-icon">"🔒"</span>
                            <div class="tip-content">
                                <div class="tip-title">"Privacy First"</div>
                                <div class="tip-description">"All processing happens locally - your voice never leaves your device"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Helper functions for event handling
fn event_target_value(event: &web_sys::Event) -> String {
    event
        .target()
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .value()
}

fn event_target_checked(event: &web_sys::Event) -> bool {
    event
        .target()
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .checked()
}
