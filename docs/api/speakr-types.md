# Shared Types (`speakr-types`)

Shared, serialisable types used across the backend (Tauri) and the UI (Leptos/WASM).

## Constants

- `DEFAULT_SCHEMA_VERSION: u32` – settings schema version for migration.
- `DEFAULT_HOTKEY: &str` – default global hot-key string (Tauri format).
- `DEFAULT_MODEL_SIZE: &str` – default Whisper model size, e.g. "medium".
- `DEFAULT_AUTO_LAUNCH: bool` – default auto-launch preference.
- `MIN_AUDIO_DURATION_SECS: u32` – min recording seconds (1).
- `MAX_AUDIO_DURATION_SECS: u32` – max recording seconds (30).
- `DEFAULT_AUDIO_DURATION_SECS: u32` – default recording seconds (10).
- `MAX_SETTINGS_FILE_SIZE: usize` – soft cap for settings file size (64KiB).

## Error Types

### `AppError`
Unified backend error type covering settings, filesystem, hot-key, command, audio, transcription and injection errors. Stringifies to user-friendly messages.

```rust
use speakr_types::AppError;
let e = AppError::Settings("Invalid hotkey format".to_string());
assert_eq!(e.to_string(), "Settings error: Invalid hotkey format");
```

### `HotkeyError`
Fine-grained hot-key errors: `RegistrationFailed`, `ConflictDetected`, `NotFound`.

### `TranscriptionError`
Transcription-related failures: `ModelNotFound`, `ModelLoadingFailed`, `ProcessingFailed`, `InsufficientMemory`, `InvalidAudioFormat`, `UnsupportedLanguage`, `DownloadFailed`.

```rust
use speakr_types::{TranscriptionError, ModelSize};
let err = TranscriptionError::ModelNotFound { model_size: ModelSize::Large };
assert!(err.user_message().contains("not available"));
```

## Settings Types

### `HotkeyConfig`
Tauri-format shortcut string plus enabled flag. Default uses `DEFAULT_HOTKEY`.

```rust
use speakr_types::HotkeyConfig;
let cfg = HotkeyConfig::default();
assert!(cfg.enabled);
```

### `AppSettings`
Single source of truth for app preferences. Includes `version`, `hot_key`, `model_size`, `auto_launch`, optional `language`, `performance_mode`, `audio_duration_secs`.

```rust
use speakr_types::AppSettings;
let mut s = AppSettings::default();
s.hot_key = "CmdOrCtrl+Alt+Space".into();
s.audio_duration_secs = 10;
assert!(s.validate().is_ok());
```

## Transcription Types

- `ModelSize` – `Small | Medium | Large` with helpers `display_name`, `to_string_value`, `from_string`, `all`.
- `ModelInfo` – derived info for a `ModelSize` (filename, size, display name, description).
- `PerformanceMode` – `Speed | Balanced | Accuracy`.
- `TranscriptionConfig` – `model_size`, `language`, `auto_detect_language`, `performance_mode`.
- `TranscriptionSegment` – per-segment text with timings and confidence.
- `TranscriptionResult` – full result with text, language, confidence, processing time, memory delta, model used, and segments.

```rust
use speakr_types::{TranscriptionConfig, ModelSize, PerformanceMode};
let cfg = TranscriptionConfig { model_size: ModelSize::Medium, ..Default::default() };
```

## Status Types

- `ServiceStatus` – `Ready | Starting | Error(String) | Unavailable` with helpers `display_name`, `is_ready`.
- `BackendStatus` – all service states plus timestamp with helpers `new_ready`, `new_starting`, `is_ready`.
- `StatusUpdate` – type alias for `BackendStatus`.

```rust
use speakr_types::BackendStatus;
let s = BackendStatus::new_ready();
assert!(s.is_ready());
```
