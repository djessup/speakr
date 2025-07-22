
# Phase 4: Extract Debug and Audio (Low Risk)

**Objective**: Isolate debug and audio functionality into separate modules

## Task Checklist (Phase 4)

- [ ] **Create debug module structure**
  - [ ] Create `speakr-tauri/src/debug/` directory
  - [ ] Create `debug/mod.rs` with conditional compilation
  - [ ] Create `debug/types.rs` for debug data structures
  - [ ] Create `debug/storage.rs` for static storage
  - [ ] Create `debug/commands.rs` for debug Tauri commands

- [ ] **Extract debug types and storage**
  - [ ] Move `DebugLogLevel` enum → `debug/types.rs`
  - [ ] Move `DebugLogMessage` struct → `debug/types.rs`
  - [ ] Move `DebugRecordingState` struct → `debug/types.rs`
  - [ ] Move `DEBUG_LOG_MESSAGES` static → `debug/storage.rs`
  - [ ] Move `DEBUG_RECORDING_STATE` static → `debug/storage.rs`
  - [ ] Move `add_debug_log()` function → `debug/storage.rs`

- [ ] **Extract debug commands**
  - [ ] Extract `debug_test_audio_recording()` implementation → `debug/commands.rs` as
        `debug_test_audio_recording_internal()`
  - [ ] Extract `debug_start_recording()` implementation → `debug/commands.rs` as
        `debug_start_recording_internal()`
  - [ ] Extract `debug_stop_recording()` implementation → `debug/commands.rs` as
        `debug_stop_recording_internal()`
  - [ ] Extract `debug_get_log_messages()` implementation → `debug/commands.rs` as
        `debug_get_log_messages_internal()`
  - [ ] Extract `debug_clear_log_messages()` implementation → `debug/commands.rs` as
        `debug_clear_log_messages_internal()`
  - [ ] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [ ] Move `get_debug_recordings_directory()` → `debug/commands.rs`
  - [ ] Make all extracted functions `pub(crate)` for module visibility

- [ ] **Create audio module structure**
  - [ ] Create `speakr-tauri/src/audio/` directory
  - [ ] Create `audio/mod.rs` with public interface
  - [ ] Create `audio/files.rs` for WAV file operations
  - [ ] Create `audio/recording.rs` for recording logic

- [ ] **Extract audio file operations**
  - [ ] Move `generate_audio_filename_with_timestamp()` → `audio/files.rs`
  - [ ] Move `save_audio_samples_to_wav_file()` → `audio/files.rs`
  - [ ] Make functions `pub(crate)` for module visibility
  - [ ] Add proper WAV spec configuration
  - [ ] Add file path validation

- [ ] **Extract audio recording functions**
  - [ ] Move `debug_record_audio_to_file()` → `audio/recording.rs`
  - [ ] Move `debug_record_real_audio_to_file()` → `audio/recording.rs`
  - [ ] Make functions `pub(crate)` for module visibility
  - [ ] Ensure proper integration with speakr-core AudioRecorder

- [ ] **Update conditional compilation**
  - [ ] Ensure `#[cfg(debug_assertions)]` is properly applied
  - [ ] Test that debug code is excluded from release builds
  - [ ] Update command registration to handle debug commands conditionally

- [ ] **Update lib.rs and test functionality**
  - [ ] Add `mod debug;` and `mod audio;` to `lib.rs`
  - [ ] Update imports and re-exports
  - [ ] Remove original debug and audio functions from `lib.rs`
  - [ ] Test debug panel functionality in development mode
  - [ ] Test audio recording and file saving
