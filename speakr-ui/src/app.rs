use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::settings::SettingsPanel;
use speakr_types::BackendStatus;

#[cfg(debug_assertions)]
use crate::debug::DebugPanel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Helper function to invoke Tauri commands that return backend status
async fn get_backend_status() -> Result<BackendStatus, String> {
    let args = JsValue::NULL;
    let result = invoke("get_backend_status", args).await;
    let json_str = js_sys::JSON::stringify(&result)
        .map_err(|_| "Failed to stringify response".to_string())?
        .as_string()
        .ok_or("Failed to convert to string".to_string())?;

    serde_json::from_str::<BackendStatus>(&json_str)
        .map_err(|e| format!("Failed to parse backend status: {e}"))
}

/// Main application view focused on settings configuration.
/// This is a modern, clean interface for Speakr dictation settings.
#[component]
pub fn App() -> impl IntoView {
    #[cfg(debug_assertions)]
    let (show_debug_panel, set_show_debug_panel) = signal(false);

    // Backend status state
    let (backend_status, set_backend_status) = signal(BackendStatus::new_starting());

    // Load initial backend status
    Effect::new(move || {
        spawn_local(async move {
            match get_backend_status().await {
                Ok(status) => {
                    set_backend_status.set(status);
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Failed to load backend status: {e}").into(),
                    );
                }
            }
        });
    });

    // TODO: Add event listener for "speakr-status-changed" events from backend
    // This would update the status in real-time when services change state

    view! {
        <div class="app">
            // Header with app branding
            <header class="app-header">
                <div class="header-content">
                    <div class="brand">
                        <div class="brand-text">
                            <h1 class="brand-title">"Speakr"</h1>
                            <p class="brand-subtitle">"Privacy-first dictation"</p>
                        </div>
                    </div>
                    <div class="header-status">
                        {move || {
                            let status = backend_status.get();
                            let status_class = if status.is_ready() { "ready" } else { "starting" };
                            let status_text = if status.is_ready() {
                                "Ready"
                            } else {
                                "Starting..."
                            };

                            view! {
                                <div class=format!("status-indicator {}", status_class)>
                                    <div class="status-dot"></div>
                                    <span>{status_text}</span>
                                </div>
                            }
                        }}

                        // Debug button only visible in debug builds
                        {move || {
                            #[cfg(debug_assertions)]
                            {
                                view! {
                                    <button
                                        class="debug-toggle-btn"
                                        on:click=move |_| set_show_debug_panel.update(|show| *show = !*show)
                                        title="Toggle Debug Panel (Debug Build Only)"
                                    >
                                        {move || if show_debug_panel.get() { "🛠️ Hide Debug" } else { "🛠️ Debug" }}
                                    </button>
                                }.into_any()
                            }
                            #[cfg(not(debug_assertions))]
                            {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </header>

            // Main content area
            <main class="main-content">
                <div class="content-container">
                    {move || {
                        #[cfg(debug_assertions)]
                        {
                            if show_debug_panel.get() {
                                view! { <DebugPanel /> }.into_any()
                            } else {
                                view! { <SettingsPanel /> }.into_any()
                            }
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            view! { <SettingsPanel /> }.into_any()
                        }
                    }}
                </div>
            </main>

            // Footer with version info
            <footer class="app-footer">
                <div class="footer-content">
                    <span class="version-info">"Speakr v0.1.0"</span>
                    <span class="privacy-note">"✓ All processing happens locally on your device"</span>
                </div>
            </footer>
        </div>
    }
}
