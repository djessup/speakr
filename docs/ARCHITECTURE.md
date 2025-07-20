---
title: Technical Architecture – Speakr
version: 2025-07-20
status: Draft
---
# Speakr – Technical Architecture

- [1. Purpose](#1-purpose)
- [2. High-Level Architecture](#2-high-level-architecture)
- [3. Crate \& Directory Layout](#3-crate--directory-layout)
- [4. Runtime Flow (Happy Path)](#4-runtime-flow-happy-path)
- [5. Concurrency \& Safety](#5-concurrency--safety)
- [6. Security \& Permissions](#6-security--permissions)
- [7. Build \& Packaging](#7-build--packaging)
- [8. Extensibility Points](#8-extensibility-points)
- [9. Risks \& Mitigations](#9-risks--mitigations)
- [10. Future Roadmap](#10-future-roadmap)

## 1. Purpose
Speakr is a **privacy-first hot-key dictation utility** for macOS (with Windows/Linux on the roadmap). When the user presses a global shortcut, it records a short audio segment, runs an **on-device Whisper model**, and synthesises keystrokes to **type** the transcript into the currently-focused application – all in under a few seconds.

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Tauri Shell
        direction TB
        GlobalShortcut["Global Shortcut<br/><i>tauri-plugin-global-shortcut</i>"]
        IPC["IPC Bridge<br/><i>tauri invoke / emit</i>"]
        Tray["System Tray / UI<br/><i>Leptos + WASM</i>"]
    end

    subgraph Core Library
        direction TB
        Recorder["Audio Recorder<br/><i>cpal</i>"]
        STT["Speech-to-Text<br/><i>whisper-rs</i>"]
        Injector["Text Injector<br/><i>enigo</i>"]
    end

    GlobalShortcut -- "hot-key pressed" --> Recorder
    Recorder -- "PCM samples" --> STT
    STT -- "transcript" --> Injector
    Injector -- "keystrokes" --> FocusApp(["Focused Application"])

    %% UI flow
    Recorder -- "status events" --- IPC
    STT ---- IPC
    Injector --- IPC
    IPC ==> Tray
```

Key points:
1. **All heavy-weight logic lives in pure Rust** (`speakr-core`). The UI may be hidden without affecting functionality.
2. **No network access** – Whisper runs entirely on-device.
3. **Plugin isolation** – Optional features (auto-start, clipboard, etc.) are added via Tauri plugins with explicit capability JSON.

---

## 3. Crate & Directory Layout

| Layer    | Crate / Path              | Main Responsibilities                                                             |
| -------- | ------------------------- | --------------------------------------------------------------------------------- |
| Core     | `speakr-core/`            | Record audio (cpal) ➜ transcribe (whisper-rs) ➜ inject text (enigo)               |
| Backend  | `speakr-tauri/src-tauri/` | Registers global hot-key, exposes `#[tauri::command]` wrappers, persists settings |
| Frontend | `speakr-ui/` (optional)   | Leptos WASM UI for tray, preferences, status overlay                              |
| Assets   | `models/`                 | GGUF Whisper models downloaded post-install                                       |

All crates live in a single **Cargo workspace** to guarantee compatible dependency versions.

---

## 4. Runtime Flow (Happy Path)

| Step | Thread/Task            | Action                                                           | Typical Latency                 |
| ---- | ---------------------- | ---------------------------------------------------------------- | ------------------------------- |
| 1    | Main (OS)              | User presses ⌘⌥Space                                             | –                               |
| 2    | Tauri shortcut handler | Spawns async task `transcribe()`                                 | < 1 ms                          |
| 3    | Tokio worker           | `cpal::Stream` captures 16-kHz mono PCM into ring-buffer         | 0–10 s (configurable)           |
| 4    | Same task              | PCM fed into `whisper_rs::full()`                                | ~1 s per 10 s audio on M-series |
| 5    | Same task              | Transcript returned → `enigo.text()` synthesises keystrokes      | ≤ 300 ms                        |
| 6    | UI task                | Frontend receives status events via `emit()` and updates overlay | realtime                        |

Failure cases (no mic, model missing, permission denied) surface via error events and native notifications.

---

## 5. Concurrency & Safety
* **Tokio** multi-thread runtime drives asynchronous recording and Whisper inference.
* The `AppState(Mutex<Option<Speakr>>)` guards the singleton Whisper context; loading occurs once at app start.
* Hot-key handler offloads work to the runtime to keep the UI thread non-blocking.
* Audio buffer uses a bounded `sync_channel` to avoid unbounded RAM growth.

---

## 6. Security & Permissions
| Platform | Permission        | Why                       | Request Mechanism                                     |
| -------- | ----------------- | ------------------------- | ----------------------------------------------------- |
| macOS    | Microphone access | Record audio              | `NSMicrophoneUsageDescription` (Info.plist)           |
| macOS    | Accessibility     | Send synthetic keystrokes | User enables app in *System Settings ▸ Accessibility* |
| All      | Global shortcut   | Register hot-key          | `global-shortcut:allow-register` capability           |

The app runs **offline**; no data leaves the device.

---

## 7. Build & Packaging
1. **Dev**: `trunk serve &` (frontend) + `cargo tauri dev` (backend)
2. **Release**: `trunk build --release` ➜ `cargo tauri build`
3. macOS notarisation: `xcrun notarytool submit --wait` after codesign.
4. Universal binary size ≈ 15 MB (+ model).

---

## 8. Extensibility Points
* **Voice Activity Detection**: plug-in `webrtc-vad` before Whisper to auto-stop on silence.
* **Streaming transcripts**: call `whisper_rs::full_partial()` and enqueue keystrokes incrementally.
* **Multi-language**: set `params.set_language(None)` for auto-detect.
* **Cross-platform**: replace `enigo` backend with `send_input` (Win) or `xdo` (X11) while keeping public API.

---

## 9. Risks & Mitigations
| Risk                                         | Mitigation                                             |
| -------------------------------------------- | ------------------------------------------------------ |
| Keystroke injection blocked in secure fields | Fallback to clipboard-paste mode with warning          |
| Whisper latency on older CPUs                | Offer `tiny.en.gguf` and shorter max record time       |
| Shortcut clashes                             | UI lets user redefine hot-key and validates uniqueness |
| Model file missing/corrupt                   | Verify checksum on load and show error dialogue        |

---

## 10. Future Roadmap
1. **Settings sync** via `tauri-plugin-store` (JSON in AppData).  
2. **Auto-start on login** (`tauri-plugin-autostart`).  
3. **GPU inference** when Whisper Metal backend stabilises.  
4. **Installer bundles** (DMG/MSI/DEB) with model downloader.

---

_This document replaces the previous placeholder `docs/ARCHITECTURE.md` and should be kept up-to-date with all architectural changes._