# Tauri Backend (`speakr-tauri`)

IPC commands exposed to the UI plus application setup and services.

## Commands (invoke from UI)

All commands are annotated with `#[tauri::command]` and must not be `pub` in code. The UI calls them via `window.__TAURI__.core.invoke` or bindings shown in the UI crate.

- `save_settings(settings: AppSettings) -> Result<(), AppError>`
- `load_settings() -> Result<AppSettings, AppError>`
- `validate_hot_key(hot_key: String) -> Result<(), AppError>`
- `check_model_availability(model_size: String) -> Result<bool, AppError>`
- `update_model_size(model_size: String) -> Result<(), AppError>`
- `update_language(language: String) -> Result<(), AppError>`
- `register_hot_key(hot_key: String) -> Result<(), AppError>`
- `register_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String>`
- `unregister_global_hotkey(app_handle: AppHandle) -> Result<(), String>`
- `update_global_hotkey(app_handle: AppHandle, config: HotkeyConfig) -> Result<(), String>`
- `set_auto_launch(enable: bool) -> Result<(), AppError>`
- `get_backend_status() -> Result<StatusUpdate, AppError>`
- `update_service_status(component: ServiceComponent, status: ServiceStatus) -> Result<(), AppError>`

Debug-only (compiled with `debug_assertions`):
- `debug_test_audio_recording() -> Result<String, AppError>`
- `debug_start_recording() -> Result<String, AppError>`
- `debug_stop_recording() -> Result<String, AppError>`
- `debug_get_log_messages() -> Result<Vec<DebugLogMessage>, AppError>`
- `debug_clear_log_messages() -> Result<(), AppError>`

## Services

- `GlobalHotkeyService` – registration and update of global shortcuts.
- `BackendStatusService` – tracks and emits status via `AppHandle::emit`.
- `ServiceComponent` – `AudioCapture | Transcription | TextInjection`.

## Application Entry

- `run()` configures plugins, registers the invoke handler, sets up a hot-key event listener, and attempts to register the default hot-key on startup.

## UI Invocation Examples

```javascript
// Save settings
await window.__TAURI_INTERNALS__.invoke("save_settings", settings);

// Validate hot-key
await window.__TAURI_INTERNALS__.invoke("validate_hot_key", { hotKey: "CmdOrCtrl+Alt+F1" });

// Status
const status = await window.__TAURI__.core.invoke("get_backend_status", null);
```
