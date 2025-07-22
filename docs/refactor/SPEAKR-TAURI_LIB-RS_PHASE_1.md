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

- **Note**: Integration tests can access internal modules via `speakr_tauri::module_name`

## Task Checklist (Phase 1)

- [ ] **Create test directory structure**
  - [ ] Create `speakr-tauri/tests/` directory
  - [ ] Create `settings_tests.rs` file
  - [ ] Create `hotkey_tests.rs` file
  - [ ] Create `status_tests.rs` file
  - [ ] Create `audio_tests.rs` file
  - [ ] Create `commands_tests.rs` file
  - [ ] Create `integration_tests.rs` file

- [ ] **Move settings-related tests**
  - [ ] Extract `test_app_settings_default()` → `settings_tests.rs`
  - [ ] Extract `test_save_and_load_settings()` → `settings_tests.rs`
  - [ ] Extract `test_settings_migration()` → `settings_tests.rs`
  - [ ] Extract `test_atomic_write_creates_backup()` → `settings_tests.rs`
  - [ ] Extract `test_corruption_recovery_*()` tests → `settings_tests.rs`
  - [ ] Extract `test_settings_serialization()` → `settings_tests.rs`
  - [ ] Extract `test_settings_performance()` → `settings_tests.rs`
  - [ ] Extract `test_settings_directory_permissions()` → `settings_tests.rs`
  - [ ] Extract `test_isolated_settings_*()` tests → `settings_tests.rs`

- [ ] **Move hotkey-related tests**
  - [ ] Extract `test_validate_hot_key_success()` → `hotkey_tests.rs`
  - [ ] Extract `test_validate_hot_key_failures()` → `hotkey_tests.rs`
  - [ ] Extract `test_register_hot_key()` → `hotkey_tests.rs`
  - [ ] Add imports for `GlobalHotkeyService` testing

- [ ] **Move status-related tests**
  - [ ] Extract `test_backend_status_service_creation()` → `status_tests.rs`
  - [ ] Extract `test_backend_status_service_update_single_service()` → `status_tests.rs`
  - [ ] Extract `test_backend_status_service_all_services_ready()` → `status_tests.rs`
  - [ ] Extract `test_backend_status_service_error_handling()` → `status_tests.rs`
  - [ ] Extract `test_backend_status_timestamps()` → `status_tests.rs`
  - [ ] Extract `test_get_backend_status_tauri_command()` → `status_tests.rs`

- [ ] **Move audio-related tests**
  - [ ] Extract `debug_save_button_functionality()` → `audio_tests.rs`
  - [ ] Add tests for audio file generation and WAV writing
  - [ ] Add tests for debug recording functionality

- [ ] **Move command-related tests**
  - [ ] Extract `test_check_model_availability()` → `commands_tests.rs`
  - [ ] Extract `test_set_auto_launch()` → `commands_tests.rs`
  - [ ] Extract `test_save_settings_tauri_command()` → `commands_tests.rs`

- [ ] **Update imports and run tests**
  - [ ] Add `use speakr_tauri::*;` imports to all test files
  - [ ] Add `use tempfile::TempDir;` where needed
  - [ ] Add `use tokio::test` attributes
  - [ ] Run `cargo test --workspace` to ensure all tests pass
  - [ ] Remove `mod tests` section from `lib.rs`
