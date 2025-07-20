# Speakr – A Local Dictation Hot-Key Utility (Rust + Tauri + Leptos)

A tiny, **privacy-first** macOS desktop app that listens for a global hot-key, records a short audio clip, transcribes it locally with Whisper, then **types** the text into whatever field currently has focus.  Everything runs on-device; no cloud calls, no Electron bloat.

---

## 1. System Overview

```text
┌──────────────────────────────┐
│        Speakr (UI)           │  ← Leptos + Tauri WebView (optional window / tray)
└───────────────┬──────────────┘
                │ invoke/emit
        Global Shortcut  ▲  Settings (model path, hot-key, …)
                ▼         │
┌─────────────────────────┴──────────────────────────┐
│            speakr-core  (Rust lib)                 │
│                                                    │
│ 1. Audio capture  – **cpal**                       │
│ 2. Transcription  – **whisper-rs** (GGUF models)   │
│ 3. Text inject    – **enigo** (synthetic keys)     │
└────────────────────────────────────────────────────┘
```
*Global shortcut*, *audio*, and *keystroke injection* all live in the backend so Speakr continues to work when the UI window is hidden.

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

> Tip Quantised **small.en.gguf (~30 MB)** loads in ≈ 2 s on Apple Silicon and is usually accurate enough for notes & code comments.

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

## 4. Bootstrapping

### 4.1 Prerequisites
* Rust 1.88.0 + (stable)
* Node 18 + & pnpm/yarn/npm (for Tauri/Trunk helpers)
* Xcode Command-Line Tools (macOS)
* Download a GGUF Whisper model → `models/ggml-small.en.gguf`

### 4.2 Create the workspace
```bash
cargo new --lib speakr-core
cargo tauri init --template leptos speakr-tauri   # generates src-tauri + Leptos wiring
cd speakr-tauri
pnpm tauri add global-shortcut                     # JavaScript guest bindings
```

(Add a sibling `speakr-ui` crate only if you want the UI separate from the template.)

---

## 5. Core Library (speakr-core)

<details>
<summary>Cargo.toml</summary>

```toml
[package]            
name    = "speakr-core"
version = "0.1.0"
edition = "2021"

[dependencies]
cpal        = "0.15"
whisper-rs  = { version = "0.8", features = ["whisper-runtime-cpu"] }
enigo       = "0.1"
tokio       = { version = "1", features = ["rt-multi-thread", "macros"] }
anyhow      = "1"
```
</details>

```rust
use anyhow::*;
use cpal::traits::*;
use enigo::*;
use std::sync::mpsc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

pub struct Speakr {
    whisper: WhisperContext,
    enigo:   Enigo,
}

impl Speakr {
    pub fn new(model_path: &str) -> Result<Self> {
        Ok(Self {
            whisper: WhisperContext::new(model_path)?,
            enigo:   Enigo::new(),
        })
    }

    pub async fn capture_and_type(&mut self, seconds: u32) -> Result<()> {
        // 1️⃣  Capture PCM samples --------------------------------------------------
        let (tx, rx) = mpsc::sync_channel(seconds as usize * 16_000);
        let host = cpal::default_host();
        let dev  = host.default_input_device().context("no input device")?;
        let cfg  = dev.default_input_config()?.into();
        let stream = dev.build_input_stream(
            &cfg,
            move |data: &[f32], _| { for &s in data { let _ = tx.send(s); } },
            move |e| eprintln!("cpal error: {e}"),
            None,
        )?;
        stream.play()?;
        let mut samples = Vec::with_capacity(seconds as usize * 16_000);
        for _ in 0..seconds * 16_000 {
            samples.push(rx.recv()?);
        }
        drop(stream);

        // 2️⃣  Transcribe -----------------------------------------------------------
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        let text = self.whisper.full(params, &samples)?;

        // 3️⃣  Inject ---------------------------------------------------------------
        self.enigo.text(&text);
        Ok(())
    }
}
```

---

## 6. Tauri Backend (speakr-tauri / `src-tauri`)

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

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use speakr_core::Speakr;
use std::sync::Mutex;
use tauri::{Manager, State};

struct AppState(Mutex<Option<Speakr>>);

#[tauri::command]
async fn transcribe(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.0.lock().unwrap();
    guard
        .as_mut()
        .ok_or("model not ready")?
        .capture_and_type(10)        // 10 s max
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::init())
        .manage(AppState(Mutex::new(None)))
        .setup(|app| {
            // Pre-load Whisper model once at startup
            let model = Speakr::new("../models/ggml-small.en.gguf")?;
            *app.state::<AppState>().0.lock().unwrap() = Some(model);

            // Register ⌘⌥Space
            #[cfg(desktop)]
            app.global_shortcut().register("CMD+OPTION+SPACE", move || {
                let handle = app.app_handle();
                tauri::async_runtime::spawn(async move {
                    let _ = handle.invoke("transcribe", &()).await;
                });
            })?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![transcribe])
        .run(tauri::generate_context!())
        .expect("error while running Speakr");
}
```

> **Capability JSON** Add `global-shortcut:allow-register` to `src-tauri/capabilities/default.json` (see Tauri docs for full schema).

---

## 7. Leptos Front-End (optional)
The Tauri template already wires Trunk + Leptos.  A minimal status UI:

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
2. **Accessibility** – Ask the user to enable Speakr under *System Settings → Privacy & Security → Accessibility* so Enigo keystrokes reach other apps.
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
* Config window for model selection & hot-key change
* Streaming, real-time transcription (partial results)
* Windows/Linux support (replace Enigo backend where needed)
* Auto-punctuation & language detection

---

### 🎉 You now have a single, coherent guide—merge of all three GPT drafts—ready to get **Speakr** typing for you on macOS in a weekend. 