# Phase 3: Extract Settings (Medium Risk)

**Objective**: Centralize all settings management into dedicated module

## Task Checklist (Phase 3)

- [ ] **Create settings module structure**
  - [ ] Create `speakr-tauri/src/settings/` directory
  - [ ] Create `settings/mod.rs` with module declarations
  - [ ] Create `settings/persistence.rs` for file I/O operations
  - [ ] Create `settings/migration.rs` for version migrations
  - [ ] Create `settings/validation.rs` for directory validation
  - [ ] Create `settings/commands.rs` for Tauri commands

- [ ] **Extract path and validation functions**
  - [ ] Move `get_settings_path()` → `settings/persistence.rs`
  - [ ] Move `get_settings_backup_path()` → `settings/persistence.rs`
  - [ ] Move `validate_settings_directory_permissions()` → `settings/validation.rs`
  - [ ] Add proper error handling and documentation

- [ ] **Extract file I/O functions**
  - [ ] Move `try_load_settings_file()` → `settings/persistence.rs`
  - [ ] Move `save_settings_to_dir()` → `settings/persistence.rs`
  - [ ] Move `load_settings_from_dir()` → `settings/persistence.rs`
  - [ ] Ensure all atomic write logic is preserved
  - [ ] Add proper error handling chains

- [ ] **Extract migration logic**
  - [ ] Move `migrate_settings()` → `settings/migration.rs`
  - [ ] Add version handling logic
  - [ ] Document migration strategy for future versions

- [ ] **Extract Tauri commands**
  - [ ] Move `save_settings()` command → `settings/commands.rs`
  - [ ] Move `load_settings()` command → `settings/commands.rs`
  - [ ] Ensure commands use the extracted helper functions
  - [ ] Maintain same function signatures for compatibility

- [ ] **Update module exports and imports**
  - [ ] Configure `settings/mod.rs` to re-export public functions
  - [ ] Add `mod settings;` to `lib.rs`
  - [ ] Update imports in `lib.rs`
  - [ ] Remove original settings functions from `lib.rs`

- [ ] **Test settings extraction thoroughly**
  - [ ] Run isolated settings tests to ensure file I/O works
  - [ ] Test corruption recovery scenarios
  - [ ] Test migration scenarios with version 0 files
  - [ ] Verify atomic write behavior
  - [ ] Test with real application settings directory
