use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::settings::SettingsPanel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Main application view focused on settings configuration.
/// This is a modern, clean interface for Speakr dictation settings.
#[component]
pub fn App() -> impl IntoView {
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
                        <div class="status-indicator ready">
                            <div class="status-dot"></div>
                            <span>"Ready"</span>
                        </div>
                    </div>
                </div>
            </header>

            // Main content area
            <main class="main-content">
                <div class="content-container">
                    <SettingsPanel />
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
