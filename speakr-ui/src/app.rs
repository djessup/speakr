use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::settings::SettingsPanel;

#[cfg(debug_assertions)]
use crate::debug::DebugPanel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Main application view focused on settings configuration.
/// This is a modern, clean interface for Speakr dictation settings.
#[component]
pub fn App() -> impl IntoView {
    #[cfg(debug_assertions)]
    let (show_debug_panel, set_show_debug_panel) = signal(false);

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
