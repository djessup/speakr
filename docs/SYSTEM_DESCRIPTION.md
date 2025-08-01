# System Description

## Speakr – a Local Dictation Utility (Rust + Tauri + Leptos)

A tiny, **privacy-first** macOS desktop app that listens for a global hot-key, records a short audio
clip, transcribes it locally with Whisper, then **types** the text into whatever currently has
focus.

Everything runs on-device; no network calls (besides the initial model download).

---

## 1. System Overview

```text
┌──────────────────────────────┐
│        Speakr (UI)           │  ← Leptos + Tauri WebView (optional window / tray)
└───────────────┬──────────────┘
                │ <invoke/emit>
        Global Shortcut   ▲    Settings (model path, hot-key, …)
                ▼         │
┌─────────────────────────┴──────────────────────────┐
│            speakr-core  (Rust lib)                 │
│                                                    │
│ 1. Audio capture  – **cpal**                       │
│ 2. Transcription  – **whisper-rs** (GGUF models)   │
│ 3. Text inject    – **enigo** (synthetic keys)     │
└────────────────────────────────────────────────────┘
```

_Global shortcut_, _audio_, and _keystroke injection_ all live in the backend so Speakr continues to
work when the UI window is hidden.

---

## 2. Key Crates & Decisions

| Concern             | Crate / Tool                         | Why it was chosen                                       |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| Hot-key             | `tauri-plugin-global-shortcut = "2"` | Official plugin, cross-platform, Tauri ≥ 2.0            |
| Audio capture       | `cpal = "0.15"`                      | Mature, async-friendly, works on macOS/Win/Linux        |
| Speech-to-Text      | `whisper-rs = "0.8"`                 | Safe Rust bindings to whisper.cpp; supports GGUF models |
| Keystroke injection | `enigo = "0.1"`                      | Simple cross-platform input simulation                  |
| UI                  | `leptos = "0.6"` + `trunk`           | All-Rust reactive UI compiled to WASM                   |
| Async runtime       | `tokio = "1"` (multi-thread)         | Needed for non-blocking recording & transcription       |

> Tip Quantised **small.en.gguf (~30 MB)** loads in ≈ 2 s on Apple Silicon and is usually accurate
> enough for notes & code comments.

---

## 3. Workspace Layout

```text
/speakr
├─ speakr-core        # library crate (audio → text → inject)
├─ speakr-tauri       # Tauri shell (`src-tauri` here)
├─ speakr-ui          # Leptos front-end (optional window)
└─ models/ggml-small.en.gguf  # user-downloaded Whisper model
```

Use a Cargo workspace so all three crates share versions and CI.

---

## 4. Current Implementation Status

### 4.1 Completed Components

- **Settings System**: Persistent configuration with validation, migration support, and
  comprehensive security measures
- **Global Hotkey**: System-wide hotkey registration with conflict detection
- **Audio Capture**: 16kHz mono recording with configurable duration (1-30 seconds)
- **Transcription Module Structure**: Complete module organisation with engine, models, language,
  and performance submodules ready for whisper-rs integration
- **Workflow Orchestration**: Complete pipeline integration with error handling
- **Testing Infrastructure**: Comprehensive test coverage with TDD practices
- **Security Framework**: Input validation, DoS protection, and secure data handling

### 4.2 Settings Integration & Security

The system includes robust settings integration with comprehensive security measures:

```rust
// Audio duration loaded from user settings with validation
pub async fn create_recording_config_from_settings() -> RecordingConfig {
    let settings = load_settings_internal().await.unwrap_or_default();
    let duration_secs = settings.audio_duration_secs; // 1-30 seconds, validated
    RecordingConfig::new(duration_secs)
}
```

**Security Features:**

- **File Size Limits**: Settings files are capped at 64KB to prevent DoS attacks
- **Schema Validation**: All settings structs reject unknown fields using
  `#[serde(deny_unknown_fields)]`
- **Input Sanitization**: Path traversal protection and comprehensive input validation
- **Atomic Operations**: Settings are written atomically with backup/recovery mechanisms
- **Enhanced Error Reporting**: Detailed JSON parsing errors with field-level diagnostics

### 4.3 Workflow Testing

Comprehensive integration tests validate the complete dictation pipeline:

```rust
#[tokio::test]
async fn test_workflow_loads_audio_duration_from_settings() {
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 25;
    save_settings_internal(settings).await.unwrap();

    let config = create_recording_config_from_settings().await;
    assert_eq!(config.max_duration_secs(), 25);
}
```

---

## 5. Core Library (speakr-core)

The core library provides audio capture functionality with configurable recording duration:

```rust
use speakr_core::audio::{AudioRecorder, RecordingConfig};

// Create recorder with settings-based configuration
let config = RecordingConfig::new(duration_secs); // From user settings
let recorder = AudioRecorder::new(config).await?;

// Start recording
recorder.start_recording().await?;

// Stop and get samples
let result = recorder.stop_recording().await?;
let samples = result.samples(); // Vec<i16> at 16kHz mono
```

### Audio Configuration

The system supports configurable recording duration:

- **Range**: 1-30 seconds (validated using `MIN_AUDIO_DURATION_SECS` and `MAX_AUDIO_DURATION_SECS`
  constants)
- **Default**: 10 seconds (defined by `DEFAULT_AUDIO_DURATION_SECS`)
- **Format**: 16kHz mono, 16-bit samples
- **Storage**: In-memory only, no disk writes

### Settings Integration

All configuration is loaded from persistent user settings:

```rust
#[derive(Serialize, Deserialize)]
pub struct AppSettings {
    pub audio_duration_secs: u32,  // 1-30 seconds (validated using constants)
    pub hot_key: String,           // Global hotkey combination
    pub model_size: String,        // Whisper model size
    pub auto_launch: bool,         // Start with system
}
```

---

## 6. Tauri Backend (speakr-tauri)

<details>
<summary>`src-tauri/Cargo.toml` extras</summary>

```toml
[dependencies]
speakr-core = { path = "../speakr-core" }
# Tauri ≥ 2.0 API-complete build
tauri       = { version = "2", features = ["api-all"] }
# Global hot-key plugin
tauri-plugin-global-shortcut = "2"
tokio       = "1"
anyhow      = "1"
```

</details>

The Tauri backend orchestrates the complete dictation workflow with settings integration:

```rust
use speakr_tauri::workflow::execute_dictation_workflow;
use speakr_tauri::settings::commands::load_settings_internal;

// Workflow orchestration with settings integration
pub async fn execute_dictation_workflow(app_handle: AppHandle) -> Result<(), AppError> {
    // Step 1: Load settings and create recording config
    let config = create_recording_config_from_settings().await;

    // Step 2: Capture audio with user-configured duration
    let audio_samples = capture_audio(&app_handle, config).await?;

    // Step 3: Transcribe (placeholder)
    let text = transcribe_audio(audio_samples, &app_handle).await?;

    // Step 4: Inject text (placeholder)
    inject_text(text, &app_handle).await?;

    Ok(())
}

// Settings-based recording configuration
pub async fn create_recording_config_from_settings() -> RecordingConfig {
    let settings = load_settings_internal().await.unwrap_or_default();
    RecordingConfig::new(settings.audio_duration_secs)
}
```

### Global Hotkey Integration

The hotkey system loads configuration from settings:

```rust
// Load hotkey from settings at startup
let settings = load_settings_internal().await?;
let hotkey = settings.hot_key; // e.g., "CmdOrCtrl+Alt+F1"

// Register with workflow integration
app.global_shortcut().register(&hotkey, move || {
    let handle = app.app_handle();
    tauri::async_runtime::spawn(async move {
        let _ = execute_dictation_workflow(handle).await;
    });
})?;
```

### Error Handling

Comprehensive error handling with user feedback:

```rust
pub enum AppError {
    AudioCapture(String),    // Device issues, permissions
    Transcription(String),   // Model loading, processing
    TextInjection(String),   // Permission, target app issues
    Settings(String),        // Configuration problems
}
```

> **Capability JSON** Add `global-shortcut:allow-register` to `src-tauri/capabilities/default.json`
> (see Tauri docs for full schema).

---

## 7. Leptos Front-End (optional)

The Tauri template already wires Trunk + Leptos. A minimal status UI:

```rust
use leptos::*;
use tauri_use::{use_invoke, UseTauri};   // helper hooks

#[component]
pub fn App() -> impl IntoView {
    let UseTauri { trigger: transcribe, .. } = use_invoke::<()>(&"transcribe");
    let (status, set_status) = create_signal("Idle");

    // Listen for status updates from backend
    leptos::window_event_listener("speakr-status", move |evt: String| set_status(evt));

    view! {
        <div class="p-4">
            <h1 class="text-xl font-bold">Speakr</h1>
            <p>{move || format!("Status: {status()}")}</p>
            <button class="mt-4 bg-blue-600 text-white px-3 py-1 rounded"
                    on:click=move |_| transcribe()>
                "Record & Type"
            </button>
        </div>
    }
}
```

`tauri.conf.json` should already contain:

```json
{
  "build": {
    "beforeDevCommand": "trunk serve",
    "beforeBuildCommand": "trunk build --release",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": { "withGlobalTauri": true }
}
```

---

## 8. macOS Permissions

1. **Microphone** – Tauri adds `NSMicrophoneUsageDescription` automatically when you enable audio.
2. **Accessibility** – Ask the user to enable Speakr under _System Settings → Privacy & Security →_
   _Accessibility_ so Enigo keystrokes reach other apps.
3. **Codesign & Notarise** – For distribution run:

```bash
cargo tauri build --target universal-apple-darwin   # produces .app bundle
# then codesign & notarise with `xcrun notarytool`
```

---

## 9. Dev & Release Workflow

```bash
# hot-reload UI + backend
trunk serve &              # terminal 1 – WASM
cargo tauri dev            # terminal 2 – desktop shell

# production
trunk build --release      # build UI assets
cargo tauri build          # build .app or MSI/DEB
```

---

## 10. Performance Levers

| Lever              | Effect                       | Hint                                   |
| ------------------ | ---------------------------- | -------------------------------------- |
| Model size         | Latency vs accuracy          | `tiny.en` ≈ 30 MB loads fastest        |
| `params.set_*`     | Threads / strategy           | Set `set_num_threads(num_cpus::get())` |
| Audio chunk length | Turn-around time             | Push-to-talk (≤ 10 s) keeps UI snappy  |
| VAD (optional)     | Trim silence & hallucination | Add `webrtc-vad` if needed             |

---

## 11. Roadmap Ideas

- Config window for model selection & hot-key change
- Streaming, real-time transcription (partial results)
- Windows/Linux support (replace Enigo backend where needed)
- Auto-punctuation & language detection

---

🎉 You now have a single, coherent guide—merge of all three GPT drafts—ready to get **Speakr**
typing for you on macOS in a weekend
