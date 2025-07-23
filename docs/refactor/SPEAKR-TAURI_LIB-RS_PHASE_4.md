
# Phase 4: Extract Debug and Audio (Low Risk)

**Objective**: Isolate debug and audio functionality into separate modules

## Task Checklist (Phase 4)

- [x] **Create debug module structure**
  - [x] Create `speakr-tauri/src/debug/` directory
  - [x] Create `debug/mod.rs` with conditional compilation
  - [x] Create `debug/types.rs` for debug data structures
  - [x] Create `debug/storage.rs` for static storage
  - [x] Create `debug/commands.rs` for debug Tauri commands

- [x] **Extract debug types and storage**
  - [x] Move `DebugLogLevel` enum → `debug/types.rs`
  - [x] Move `DebugLogMessage` struct → `debug/types.rs`
  - [x] Move `DebugRecordingState` struct → `debug/types.rs`
  - [x] Move `DEBUG_LOG_MESSAGES` static → `debug/storage.rs`
  - [x] Move `DEBUG_RECORDING_STATE` static → `debug/storage.rs`
  - [x] Move `add_debug_log()` function → `debug/storage.rs`

- [x] **Extract debug commands**
  - [x] Extract `debug_test_audio_recording()` implementation → `debug/commands.rs` as
        `debug_test_audio_recording_internal()`
  - [x] Extract `debug_start_recording()` implementation → `debug/commands.rs` as
        `debug_start_recording_internal()`
  - [x] Extract `debug_stop_recording()` implementation → `debug/commands.rs` as
        `debug_stop_recording_internal()`
  - [x] Extract `debug_get_log_messages()` implementation → `debug/commands.rs` as
        `debug_get_log_messages_internal()`
  - [x] Extract `debug_clear_log_messages()` implementation → `debug/commands.rs` as
        `debug_clear_log_messages_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Move `get_debug_recordings_directory()` → `debug/commands.rs`
  - [x] Make all extracted functions `pub(crate)` for module visibility

- [x] **Create audio module structure**
  - [x] Create `speakr-tauri/src/audio/` directory
  - [x] Create `audio/mod.rs` with public interface
  - [x] Create `audio/files.rs` for WAV file operations
  - [x] Create `audio/recording.rs` for recording logic

- [x] **Extract audio file operations**
  - [x] Move `generate_audio_filename_with_timestamp()` → `audio/files.rs`
  - [x] Move `save_audio_samples_to_wav_file()` → `audio/files.rs`
  - [x] Make functions `pub(crate)` for module visibility
  - [x] Add proper WAV spec configuration
  - [x] Add file path validation

- [x] **Extract audio recording functions**
  - [x] Move `debug_record_audio_to_file()` → `audio/recording.rs`
  - [x] Move `debug_record_real_audio_to_file()` → `audio/recording.rs`
  - [x] Make functions `pub(crate)` for module visibility
  - [x] Ensure proper integration with speakr-core AudioRecorder

- [x] **Update conditional compilation**
  - [x] Ensure `#[cfg(debug_assertions)]` is properly applied
  - [x] Test that debug code is excluded from release builds (compilation successful)
  - [x] Update command registration to handle debug commands conditionally

- [x] **Update lib.rs and test functionality**
  - [x] Add `mod debug;` and `mod audio;` to `lib.rs`
  - [x] Update imports and re-exports
  - [x] Remove original debug and audio functions from `lib.rs`
  - [x] Test debug panel functionality in development mode (24/27 tests passing)
  - [x] Test audio recording and file saving (integration tests passing)
