use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::settings::SettingsPanel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

/// Main application view with navigation between main and settings views.
#[component]
pub fn App() -> impl IntoView {
    // State for switching between main view and settings
    let (current_view, set_current_view) = signal("main");
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <div class="app">
            // Navigation bar
            <nav class="nav-bar">
                <div class="nav-buttons">
                    <button
                        class={move || if current_view.get() == "main" { "nav-btn active" } else { "nav-btn" }}
                        on:click=move |_| set_current_view.set("main")
                    >
                        "Main"
                    </button>
                    <button
                        class={move || if current_view.get() == "settings" { "nav-btn active" } else { "nav-btn" }}
                        on:click=move |_| set_current_view.set("settings")
                    >
                        "Settings"
                    </button>
                </div>
            </nav>

            // Main content area
            <main class="main-content">
                {move || {
                    let view = current_view.get();
                    if view == "settings" {
                        view! { <SettingsPanel /> }.into_any()
                    } else {
                        view! {
                            <div class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            <p>{ move || greet_msg.get() }</p>
                            </div>
                        }.into_any()
                    }
                }}
        </main>
        </div>
    }
}
