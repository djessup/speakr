
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
  - [ ] Move `debug_test_audio_recording()` → `debug/commands.rs`
  - [ ] Move `debug_start_recording()` → `debug/commands.rs`
  - [ ] Move `debug_stop_recording()` → `debug/commands.rs`
  - [ ] Move `debug_get_log_messages()` → `debug/commands.rs`
  - [ ] Move `debug_clear_log_messages()` → `debug/commands.rs`
  - [ ] Move `get_debug_recordings_directory()` → `debug/commands.rs`

- [ ] **Create audio module structure**
  - [ ] Create `speakr-tauri/src/audio/` directory
  - [ ] Create `audio/mod.rs` with public interface
  - [ ] Create `audio/files.rs` for WAV file operations
  - [ ] Create `audio/recording.rs` for recording logic
  - [ ] Create `audio/commands.rs` for audio Tauri commands

- [ ] **Extract audio file operations**
  - [ ] Move `generate_audio_filename_with_timestamp()` → `audio/files.rs`
  - [ ] Move `save_audio_samples_to_wav_file()` → `audio/files.rs`
  - [ ] Add proper WAV spec configuration
  - [ ] Add file path validation

- [ ] **Extract audio recording functions**
  - [ ] Move `debug_record_audio_to_file()` → `audio/recording.rs`
  - [ ] Move `debug_record_real_audio_to_file()` → `audio/recording.rs`
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
