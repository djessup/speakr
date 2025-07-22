# Phase 5: Extract Commands and Finalize (Low Risk)

**Objective**: Group remaining commands and clean up lib.rs to final state

## Task Checklist (Phase 5)

- [ ] **Create commands module structure**
  - [ ] Create `speakr-tauri/src/commands/` directory
  - [ ] Create `commands/mod.rs` with command registration logic
  - [ ] Create `commands/validation.rs` for input validation
  - [ ] Create `commands/system.rs` for system-related commands
  - [ ] Create `commands/legacy.rs` for backward compatibility

- [ ] **Extract validation commands**
  - [ ] Move `validate_hot_key_internal()` → `commands/validation.rs` (already extracted from
        implementation)
  - [ ] Keep `validate_hot_key()` wrapper in `lib.rs` (already implemented)
  - [ ] Make internal function `pub(crate)` for module visibility
  - [ ] Add comprehensive hotkey format validation
  - [ ] Add input sanitization and error handling
  - [ ] Document validation rules and supported formats

- [ ] **Extract system commands**
  - [ ] Extract `check_model_availability()` implementation → `commands/system.rs` as
        `check_model_availability_internal()`
  - [ ] Extract `set_auto_launch()` implementation → `commands/system.rs` as
        `set_auto_launch_internal()`
  - [ ] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [ ] Make internal functions `pub(crate)` for module visibility
  - [ ] Add proper file system checks and error handling
  - [ ] Add system integration placeholders for auto-launch

- [ ] **Extract legacy commands**
  - [ ] Extract `register_hot_key()` implementation → `commands/legacy.rs` as
        `register_hot_key_internal()`
  - [ ] Extract `greet()` implementation → `commands/legacy.rs` as `greet_internal()`
  - [ ] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [ ] Make internal functions `pub(crate)` for module visibility
  - [ ] Add deprecation warnings if appropriate
  - [ ] Document backward compatibility requirements

- [ ] **Create centralized command registration**
  - [ ] Maintain existing command registration pattern in `run()` function
  - [ ] Keep conditional compilation for debug vs release builds
  - [ ] Ensure all `#[tauri::command]` wrappers remain in `lib.rs` for registration
  - [ ] Create helper functions in `commands/mod.rs` for command organization if needed
  - [ ] Document command registration requirements for future additions

- [ ] **Finalize lib.rs cleanup**
  - [ ] Remove all extracted function implementations (`*_internal` functions moved to modules)
  - [ ] Keep all `#[tauri::command]` wrappers that delegate to `*_internal` functions
  - [ ] Note: `validate_hot_key_internal()` already exists and just needs to be moved
  - [ ] Keep only essential imports
  - [ ] Clean up module declarations
  - [ ] Update `run()` function command registration as needed
  - [ ] Add comprehensive module documentation

- [ ] **Final testing and validation**
  - [ ] Run full test suite: `cargo test --workspace --all-features`
  - [ ] Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
  - [ ] Run formatting check: `cargo fmt --all -- --check`
  - [ ] Test application startup and hotkey registration
  - [ ] Test settings save/load functionality
  - [ ] Test debug panel (in debug mode)
  - [ ] Verify all Tauri commands are accessible from frontend
  - [ ] Check final line count of `lib.rs` (~150-200 lines target)

  - [ ] **Documentation updates**
    - [ ] Update module documentation in each new file
    - [ ] Add rustdoc examples where appropriate
    - [ ] Update any architectural documentation
    - [ ] Create migration notes for future developers
