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
  - [ ] Move `validate_hot_key()` → `commands/validation.rs`
  - [ ] Add comprehensive hotkey format validation
  - [ ] Add input sanitization and error handling
  - [ ] Document validation rules and supported formats

- [ ] **Extract system commands**
  - [ ] Move `check_model_availability()` → `commands/system.rs`
  - [ ] Move `set_auto_launch()` → `commands/system.rs`
  - [ ] Add proper file system checks and error handling
  - [ ] Add system integration placeholders for auto-launch

- [ ] **Extract legacy commands**
  - [ ] Move `register_hot_key()` → `commands/legacy.rs`
  - [ ] Move `greet()` → `commands/legacy.rs`
  - [ ] Add deprecation warnings if appropriate
  - [ ] Document backward compatibility requirements

- [ ] **Create centralized command registration**
  - [ ] Create command registration function in `commands/mod.rs`
  - [ ] Group commands by conditional compilation (debug vs release)
  - [ ] Create macro or helper for command handler generation
  - [ ] Ensure all commands are properly registered

- [ ] **Finalize lib.rs cleanup**
  - [ ] Remove all extracted functions and types
  - [ ] Keep only essential imports
  - [ ] Clean up module declarations
  - [ ] Update `run()` function to use command registration helper
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
