# Phase 1: Extract Tests (Low Risk)

**Objective**: Move all tests from `lib.rs` into separate files organized by domain

- **New Structure**:

  ```text
  speakr-tauri/tests/
  ├── settings_tests.rs       # Settings save/load/migration tests
  ├── hotkey_tests.rs         # GlobalHotkeyService tests
  ├── status_tests.rs         # BackendStatusService tests
  ├── audio_tests.rs          # Audio recording/file tests
  ├── commands_tests.rs       # Tauri command tests
  └── integration_tests.rs    # Cross-module integration tests
  ```

- **Note**: Integration tests can access internal modules via `speakr_lib::module_name`

## Task Checklist (Phase 1)

- [x] **Create test directory structure**
  - [x] Create `speakr-tauri/tests/` directory
  - [x] Create `settings_tests.rs` file
  - [x] Create `hotkey_tests.rs` file
  - [x] Create `status_tests.rs` file
  - [x] Create `audio_tests.rs` file
  - [x] Create `commands_tests.rs` file
  - [x] Create `integration_tests.rs` file

- [x] **Move settings-related tests**
  - [x] Extract `test_app_settings_default()` → `settings_tests.rs`
  - [~] Extract `test_save_and_load_settings()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_settings_migration()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_atomic_write_creates_backup()` → ~~SKIPPED: Uses Tauri commands~~
  - [~] Extract `test_corruption_recovery_*()` → ~~SKIPPED: Uses private functions~~
  - [x] Extract `test_settings_serialization()` → `settings_tests.rs`
  - [~] Extract `test_save_settings_tauri_command()` → ~~SKIPPED: Tests Tauri command~~
  - [~] Extract `test_settings_performance()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_settings_directory_permissions()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_isolated_settings_*()` tests → ~~SKIPPED: Uses private functions~~

  - [x] Extract `debug_save_button_functionality()` → `settings_tests.rs`

- [x] **Move hotkey-related tests**
  - [x] Extract `test_validate_hot_key_success()` → `hotkey_tests.rs`
  - [x] Extract `test_validate_hot_key_failures()` → `hotkey_tests.rs`
  - [~] Extract `test_register_hot_key()` → ~~SKIPPED: Tests Tauri command~~
  - [x] Add imports for `GlobalHotkeyService` testing

- [x] **Move status-related tests**
  - [x] Extract `test_backend_status_service_creation()` → `status_tests.rs`
  - [x] Extract `test_backend_status_service_update_single_service()` → `status_tests.rs`
  - [x] Extract `test_backend_status_service_all_services_ready()` → `status_tests.rs`
  - [x] Extract `test_backend_status_service_error_handling()` → `status_tests.rs`
  - [x] Extract `test_backend_status_timestamps()` → `status_tests.rs`
  - [~] Extract `test_get_backend_status_tauri_command()` → ~~SKIPPED: Tests Tauri command~~
  - [~] Extract `test_global_backend_service_initialization()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_global_backend_service_state_updates()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_global_backend_service_thread_safety()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_get_backend_status_command_uses_real_service()`
    → ~~SKIPPED: Tests Tauri command~~
  - [~] Extract `test_backend_service_emits_events_on_state_change()`
    → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_complete_status_communication_flow()` → ~~SKIPPED: Uses private functions~~

- [~] **Move audio-related tests** ~~SKIPPED: All use private functions~~
  - [~] Extract `test_debug_record_audio_to_file_saves_with_timestamp()`
        → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_debug_record_audio_to_file_creates_unique_filenames()`
    → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_save_audio_samples_to_wav_file()` → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_generate_audio_filename_with_timestamp()`
    → ~~SKIPPED: Uses private functions~~
  - [~] Extract `test_debug_real_audio_recording_integration()`
    → ~~SKIPPED: Uses private functions~~
  - [~] Add tests for audio file generation and WAV writing → ~~SKIPPED: Functions are private~~
  - [~] Add tests for debug recording functionality → ~~SKIPPED: Functions are private~~

- [~] **Move command-related tests** ~~SKIPPED: All are Tauri command tests~~
  - [~] Extract `test_check_model_availability()` → ~~SKIPPED: Tests Tauri command~~
  - [~] Extract `test_set_auto_launch()` → ~~SKIPPED: Tests Tauri command~~

- [x] **Update imports and run tests**
  - [x] Updated imports in working test files (`settings_tests.rs`, `status_tests.rs`)
  - [x] Cleaned up unused imports to remove warnings
  - [x] Verified all migrated tests still pass (8 tests: 3 settings + 5 status)
  - [x] Removed successfully migrated test functions from `lib.rs`
  - [x] Run `cargo test --workspace` - all unit tests pass ✅
  - [ ] Remove `mod tests` section from `lib.rs` (will be done in final cleanup)
