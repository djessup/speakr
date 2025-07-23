# Phase 3: Extract Settings (Medium Risk)

**Objective**: Centralize all settings management into dedicated module

## Task Checklist (Phase 3)

- [x] **Create settings module structure**
  - [x] Create `speakr-tauri/src/settings/` directory
  - [x] Create `settings/mod.rs` with module declarations
  - [x] Create `settings/persistence.rs` for file I/O operations
  - [x] Create `settings/migration.rs` for version migrations
  - [x] Create `settings/validation.rs` for directory validation
  - [x] Create `settings/commands.rs` for Tauri commands

- [x] **Extract path and validation functions**
  - [x] Move `get_settings_path()` → `settings/persistence.rs`
  - [x] Move `get_settings_backup_path()` → `settings/persistence.rs`
  - [x] Move `validate_settings_directory_permissions()` → `settings/validation.rs`
  - [x] Add proper error handling and documentation
  - [x] Make functions `pub(crate)` for module visibility

- [x] **Extract file I/O functions**
  - [x] Move `try_load_settings_file()` → `settings/persistence.rs`
  - [x] Move `save_settings_to_dir()` → `settings/persistence.rs`
  - [x] Move `load_settings_from_dir()` → `settings/persistence.rs`
  - [x] Ensure all atomic write logic is preserved
  - [x] Add proper error handling chains
  - [x] Make private functions `pub(crate)` for module visibility

- [x] **Extract migration logic**
  - [x] Move `migrate_settings()` → `settings/migration.rs`
  - [x] Add version handling logic
  - [x] Document migration strategy for future versions
  - [x] Make function `pub(crate)` for module visibility

- [x] **Extract Tauri commands**
  - [x] Extract `save_settings()` implementation → `settings/commands.rs`
        as `save_settings_internal()`
  - [x] Extract `load_settings()` implementation → `settings/commands.rs`
        as `load_settings_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Ensure internal functions use the extracted helper functions
  - [x] Make internal functions `pub(crate)` for module visibility
  - [x] Maintain same function signatures for compatibility

- [x] **Update module exports and imports**
  - [x] Configure `settings/mod.rs` to re-export public functions
  - [x] Add `mod settings;` to `lib.rs`
  - [x] Update imports in `lib.rs`
  - [x] Remove original settings functions from `lib.rs`

- [x] **Test settings extraction thoroughly**
  - [x] Run isolated settings tests to ensure file I/O works
  - [x] Test corruption recovery scenarios
  - [x] Test migration scenarios with version 0 files
  - [x] Verify atomic write behavior
  - [x] Test with real application settings directory
