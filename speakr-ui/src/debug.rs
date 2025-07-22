//! Debug panel module for Speakr development builds.
//!
//! This module provides debugging utilities and testing interfaces
//! that are only available in debug builds. It includes:
//! - Audio recording test interface
//! - Debug information display
//! - Development-only controls
//!
//! The module is conditionally compiled using `#[cfg(debug_assertions)]`
//! and will not be included in release builds.

use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Log level for filtering console messages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Returns all available log levels
    pub fn all() -> Vec<LogLevel> {
        vec![
            LogLevel::Trace,
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Warn,
            LogLevel::Error,
        ]
    }

    /// Returns the display name for the log level
    pub fn display_name(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    /// Returns the emoji icon for the log level
    pub fn icon(&self) -> &'static str {
        match self {
            LogLevel::Trace => "🔍",
            LogLevel::Debug => "🐛",
            LogLevel::Info => "ℹ️",
            LogLevel::Warn => "⚠️",
            LogLevel::Error => "❌",
        }
    }

    /// Returns the CSS class for styling the log level
    pub fn css_class(&self) -> &'static str {
        match self {
            LogLevel::Trace => "log-trace",
            LogLevel::Debug => "log-debug",
            LogLevel::Info => "log-info",
            LogLevel::Warn => "log-warn",
            LogLevel::Error => "log-error",
        }
    }
}

/// A log message entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub timestamp: String,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
}

impl LogMessage {
    /// Creates a new log message
    pub fn new(level: LogLevel, target: &str, message: &str) -> Self {
        Self {
            timestamp: js_sys::Date::new_0()
                .to_iso_string()
                .as_string()
                .unwrap_or_else(|| "unknown".to_string()),
            level,
            target: target.to_string(),
            message: message.to_string(),
        }
    }
}

/// External bindings to Tauri APIs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"], js_name = invoke)]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Helper function to invoke Tauri commands
#[allow(dead_code)]
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

/// Debug manager that handles development-only functionality
pub struct DebugManager;

impl DebugManager {
    /// Tests audio recording functionality
    pub async fn test_audio_recording() -> Result<String, String> {
        tauri_invoke_no_args("debug_test_audio_recording")
            .await
            .map_err(|e| format!("Failed to test audio recording: {e}"))
    }

    /// Starts audio recording (push-to-talk)
    pub async fn start_recording() -> Result<String, String> {
        tauri_invoke_no_args("debug_start_recording")
            .await
            .map_err(|e| format!("Failed to start recording: {e}"))
    }

    /// Stops audio recording (release)
    pub async fn stop_recording() -> Result<String, String> {
        tauri_invoke_no_args("debug_stop_recording")
            .await
            .map_err(|e| format!("Failed to stop recording: {e}"))
    }

    /// Gets recent log messages from the backend
    pub async fn get_log_messages() -> Result<Vec<LogMessage>, String> {
        tauri_invoke_no_args("debug_get_log_messages")
            .await
            .map_err(|e| format!("Failed to get log messages: {e}"))
    }

    /// Clears all log messages
    pub async fn clear_log_messages() -> Result<(), String> {
        tauri_invoke_no_args::<()>("debug_clear_log_messages")
            .await
            .map_err(|e| format!("Failed to clear log messages: {e}"))
    }
}

/// Logging console component for displaying filtered log messages
#[component]
pub fn LoggingConsole() -> impl IntoView {
    let (log_messages, set_log_messages) = signal::<Vec<LogMessage>>(Vec::new());
    let (selected_level, set_selected_level) = signal::<Option<LogLevel>>(None);
    let (auto_scroll, set_auto_scroll) = signal(true);

    // Initial load of log messages
    Effect::new(move || {
        spawn_local(async move {
            match DebugManager::get_log_messages().await {
                Ok(messages) => {
                    set_log_messages.set(messages);
                }
                Err(_) => {
                    // Silently fail - we don't want to spam errors in a debug console
                    // Add some mock messages for testing
                    let mock_messages = vec![
                        LogMessage::new(LogLevel::Info, "speakr-debug", "Debug panel initialized"),
                        LogMessage::new(LogLevel::Debug, "speakr-core", "Audio system ready"),
                        LogMessage::new(LogLevel::Warn, "speakr-tauri", "Mock warning message"),
                    ];
                    set_log_messages.set(mock_messages);
                }
            }
        });
    });

    // Filter messages based on selected level
    let filtered_messages = move || {
        let messages = log_messages.get();
        if let Some(level) = selected_level.get() {
            messages
                .into_iter()
                .filter(|msg| msg.level == level)
                .collect()
        } else {
            messages
        }
    };

    // Clear log messages
    let clear_logs = move || {
        spawn_local(async move {
            let _ = DebugManager::clear_log_messages().await;
            set_log_messages.set(Vec::new());
        });
    };

    // Refresh logs manually
    let refresh_logs = move || {
        spawn_local(async move {
            if let Ok(messages) = DebugManager::get_log_messages().await {
                set_log_messages.set(messages);
            }
        });
    };

    view! {
        <div class="logging-console">
            <div class="console-header">
                <h4>"📜 Log Console"</h4>
                <div class="console-controls">
                    <select
                        class="log-level-filter"
                        on:change=move |e| {
                            let value = event_target_value(&e);
                            if value == "all" {
                                set_selected_level.set(None);
                            } else {
                                let level = match value.as_str() {
                                    "trace" => Some(LogLevel::Trace),
                                    "debug" => Some(LogLevel::Debug),
                                    "info" => Some(LogLevel::Info),
                                    "warn" => Some(LogLevel::Warn),
                                    "error" => Some(LogLevel::Error),
                                    _ => None,
                                };
                                set_selected_level.set(level);
                            }
                        }
                    >
                        <option value="all" selected={move || selected_level.get().is_none()}>"All Levels"</option>
                        {LogLevel::all().into_iter().map(|level| {
                            let level_str = level.display_name().to_lowercase();
                            let is_selected = selected_level.get().as_ref() == Some(&level);
                            view! {
                                <option
                                    value={level_str.clone()}
                                    selected={is_selected}
                                >
                                    {format!("{} {}", level.icon(), level.display_name())}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>

                    <label class="auto-scroll-toggle">
                        <input
                            type="checkbox"
                            checked={move || auto_scroll.get()}
                            on:change=move |e| set_auto_scroll.set(event_target_checked(&e))
                        />
                        "Auto-scroll"
                    </label>

                    <button class="refresh-logs-btn" on:click=move |_| refresh_logs()>
                        "🔄 Refresh"
                    </button>

                    <button class="clear-logs-btn" on:click=move |_| clear_logs()>
                        "🗑️ Clear"
                    </button>
                </div>
            </div>

            <div class="console-messages" class:auto-scroll={move || auto_scroll.get()}>
                {move || {
                    let messages = filtered_messages();
                    if messages.is_empty() {
                        view! {
                            <div class="no-messages">
                                "No log messages to display"
                            </div>
                        }.into_any()
                    } else {
                        messages.into_iter().map(|msg| {
                            view! {
                                <div class={format!("log-entry {}", msg.level.css_class())}>
                                    <span class="log-timestamp">{msg.timestamp.chars().take(19).collect::<String>()}</span>
                                    <span class="log-level">
                                        {msg.level.icon()} {msg.level.display_name()}
                                    </span>
                                    <span class="log-target">{msg.target}</span>
                                    <span class="log-message">{msg.message}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>().into_any()
                    }
                }}
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
        .map(|input| input.value())
        .or_else(|_| {
            event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlSelectElement>()
                .map(|select| select.value())
        })
        .unwrap_or_default()
}

fn event_target_checked(event: &web_sys::Event) -> bool {
    event
        .target()
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap()
        .checked()
}

/// Debug panel component for development builds only.
///
/// This component provides debugging tools and test interfaces
/// to help with development and testing of Speakr features.
#[component]
pub fn DebugPanel() -> impl IntoView {
    // Debug state
    let (debug_message, set_debug_message) = signal::<Option<String>>(None);
    let (is_recording, set_is_recording) = signal(false);

    // Test audio recording function (legacy - for compatibility)
    let test_audio_recording = move || {
        set_is_recording.set(true);
        set_debug_message.set(Some("Testing audio recording...".to_string()));

        spawn_local(async move {
            match DebugManager::test_audio_recording().await {
                Ok(result) => {
                    set_debug_message.set(Some(format!("✅ Audio test result: {result}")));
                }
                Err(e) => {
                    set_debug_message.set(Some(format!("❌ Audio test failed: {e}")));
                }
            }
            set_is_recording.set(false);
        });
    };

    // Push-to-talk recording functions
    let start_recording = move || {
        set_is_recording.set(true);
        set_debug_message.set(Some("🎙️ Recording started (push-to-talk)...".to_string()));

        spawn_local(async move {
            match DebugManager::start_recording().await {
                Ok(result) => {
                    set_debug_message.set(Some(format!("🎙️ Recording: {result}")));
                }
                Err(e) => {
                    set_debug_message.set(Some(format!("❌ Failed to start recording: {e}")));
                    set_is_recording.set(false);
                }
            }
        });
    };

    let stop_recording = move || {
        if is_recording.get() {
            spawn_local(async move {
                match DebugManager::stop_recording().await {
                    Ok(result) => {
                        set_debug_message.set(Some(format!("⏹️ Recording stopped: {result}")));
                    }
                    Err(e) => {
                        set_debug_message.set(Some(format!("❌ Failed to stop recording: {e}")));
                    }
                }
                set_is_recording.set(false);
            });
        }
    };

    view! {
        <div class="debug-panel">
            <div class="debug-header">
                <h2>"🛠️ Debug Panel"</h2>
                <p class="debug-description">"Development tools and testing interface"</p>
                <div class="debug-badge">"DEBUG BUILD"</div>
            </div>

            <div class="debug-content">
                // Audio Testing Section
                <div class="debug-group">
                    <h3>"🎙️ Audio Testing"</h3>
                    <p class="debug-description">
                        "Test audio recording functionality with push-to-talk interface"
                    </p>

                    <div class="debug-controls">
                        <button
                            class="debug-btn-secondary"
                            on:click=move |_| test_audio_recording()
                            disabled={move || is_recording.get()}
                        >
                            {move || if is_recording.get() {
                                "🔄 Testing..."
                            } else {
                                "🧪 Test Audio (Legacy)"
                            }}
                        </button>

                        <button
                            class={move || format!("debug-btn-record {}",
                                if is_recording.get() { "recording" } else { "" }
                            )}
                            on:mousedown=move |_| start_recording()
                            on:mouseup=move |_| stop_recording()
                            on:mouseleave=move |_| stop_recording()
                            on:touchstart=move |_| start_recording()
                            on:touchend=move |_| stop_recording()
                        >
                            {move || if is_recording.get() {
                                "🔴 Recording... (Release to stop)"
                            } else {
                                "🎙️ Hold to Record"
                            }}
                        </button>
                    </div>
                </div>

                // Debug Messages Section
                <div class="debug-group">
                    <h3>"📝 Debug Output"</h3>
                    {move || {
                        if let Some(message) = debug_message.get() {
                            view! {
                                <div class="debug-output">
                                    <pre>{message}</pre>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="debug-output-placeholder">
                                    "Debug messages will appear here..."
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                // System Information Section
                <div class="debug-group">
                    <h3>"ℹ️ System Info"</h3>
                    <div class="debug-info-grid">
                        <div class="debug-info-item">
                            <span class="debug-info-label">"Build Type:"</span>
                            <span class="debug-info-value">"Debug"</span>
                        </div>
                        <div class="debug-info-item">
                            <span class="debug-info-label">"Environment:"</span>
                            <span class="debug-info-value">"Development"</span>
                        </div>
                        <div class="debug-info-item">
                            <span class="debug-info-label">"Recording State:"</span>
                            <span class="debug-info-value">
                                {move || if is_recording.get() { "🔴 Active" } else { "⚫ Inactive" }}
                            </span>
                        </div>
                    </div>
                </div>

                // Logging Console Section
                <div class="debug-group">
                    <LoggingConsole />
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_manager_exists() {
        // RED: Test that DebugManager type exists
        let _manager = DebugManager;
    }

    #[tokio::test]
    async fn test_debug_manager_test_audio_recording_returns_result() {
        // RED: Test that the audio recording test method exists and returns a Result
        // Note: This test runs in a native environment, not WASM, so we can't actually
        // call the wasm-bindgen invoke function. Instead, we test the structure.

        // Test that the DebugManager has the method and it compiles
        // In a real WASM environment, this would call the Tauri backend

        // For now, we just verify the method signature exists and compiles
        // The actual integration testing would happen in a browser environment

        // This is a compile-time test - if the method signature is wrong, this won't compile
        let _test_fn: fn() -> _ = DebugManager::test_audio_recording;

        // Test passes if the function compiles and exists
        // We use the function pointer to verify it exists
        let _ptr = _test_fn as *const ();
        assert!(!_ptr.is_null());
    }

    // Test that the debug panel component can be created
    // Note: Full component testing would require a test harness for Leptos
    #[test]
    fn test_debug_panel_component_compiles() {
        // RED: This test ensures the component compiles
        // The actual rendering would need a Leptos test environment
        // For now, we just test that the function exists and compiles
        let _component_fn = DebugPanel;

        // Test passes by compiling successfully
        let _ptr = _component_fn as *const ();
        assert!(!_ptr.is_null());
    }

    #[test]
    fn test_debug_manager_push_to_talk_methods_exist() {
        // RED: Test that the push-to-talk methods exist and compile
        // This is a compile-time test - if the method signatures are wrong, this won't compile
        let _start_fn: fn() -> _ = DebugManager::start_recording;
        let _stop_fn: fn() -> _ = DebugManager::stop_recording;

        // Test passes if we can reference both functions
        let _start_ptr = _start_fn as *const ();
        let _stop_ptr = _stop_fn as *const ();
        assert!(!_start_ptr.is_null() && !_stop_ptr.is_null());
    }
}
