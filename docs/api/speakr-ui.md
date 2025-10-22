# UI (`speakr-ui`)

Leptos-based WASM frontend bundled into Tauri.

## Root

- `pub fn main()` – WASM start function that sets up panic hook and mounts `<App/>`.
- `pub use app::App` – exported for integration tests and mount points.

## Components

- `App` – root application shell. Loads backend status, shows header with status, main content, and footer. In debug builds toggles `DebugPanel`.
- `SettingsPanel` – settings UI: edit hot-key with validation and registration, select model size with availability indicators, toggle auto-launch, and save.
- `LoggingConsole` (debug) – live log viewer with filtering by level and auto-refresh.
- `DebugPanel` (debug) – audio test, push-to-talk recording, log console.

## Command Invocation Helpers

The UI bindings use `wasm-bindgen` to call Tauri commands.

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
```

Higher-level helpers in `settings.rs` and `debug.rs` handle serialisation and response parsing. For example:

```rust
// Load settings
let settings: AppSettings = tauri_invoke_no_args("load_settings").await?;

// Validate hot-key
tauri_invoke::<(), _>("validate_hot_key", &serde_json::json!({"hotKey": "CmdOrCtrl+Alt+F1"})).await?;
```

## Styling and UX

- Modern, minimal settings layout with clear status indicators.
- All processing occurs locally; UI reflects `BackendStatus` from backend.
