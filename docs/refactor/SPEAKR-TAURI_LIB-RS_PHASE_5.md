# Phase 5: Extract Commands and Finalize (Low Risk)

**Objective**: Group remaining commands and clean up lib.rs to final state

## Task Checklist (Phase 5)

- [x] **Create commands module structure**
  - [x] Create `speakr-tauri/src/commands/` directory
  - [x] Create `commands/mod.rs` with command registration logic
  - [x] Create `commands/validation.rs` for input validation
  - [x] Create `commands/system.rs` for system-related commands
  - [x] Create `commands/legacy.rs` for backward compatibility

- [x] **Extract validation commands**
  - [x] Move `validate_hot_key_internal()` → `commands/validation.rs` (already extracted from
        implementation)
  - [x] Keep `validate_hot_key()` wrapper in `lib.rs` (already implemented)
  - [x] Make internal function `pub(crate)` for module visibility
  - [x] Add comprehensive hotkey format validation
  - [x] Add input sanitization and error handling
  - [x] Document validation rules and supported formats

- [x] **Extract system commands**
  - [x] Extract `check_model_availability()` implementation → `commands/system.rs` as
        `check_model_availability_internal()`
  - [x] Extract `set_auto_launch()` implementation → `commands/system.rs` as
        `set_auto_launch_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Make internal functions `pub(crate)` for module visibility
  - [x] Add proper file system checks and error handling
  - [x] Add system integration placeholders for auto-launch

- [x] **Extract legacy commands**
  - [x] Extract `register_hot_key()` implementation → `commands/legacy.rs` as
        `register_hot_key_internal()`
  - [x] Extract `greet()` implementation → `commands/legacy.rs` as `greet_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Make internal functions `pub(crate)` for module visibility
  - [x] Add deprecation warnings if appropriate
  - [x] Document backward compatibility requirements

- [x] **Create centralized command registration**
  - [x] Maintain existing command registration pattern in `run()` function
  - [x] Keep conditional compilation for debug vs release builds
  - [x] Ensure all `#[tauri::command]` wrappers remain in `lib.rs` for registration
  - [x] Create helper functions in `commands/mod.rs` for command organization if needed
  - [x] Document command registration requirements for future additions

- [x] **Finalize lib.rs cleanup**
  - [x] Remove all extracted function implementations (`*_internal` functions moved to modules)
  - [x] Keep all `#[tauri::command]` wrappers that delegate to `*_internal` functions
  - [x] Note: `validate_hot_key_internal()` already exists and just needs to be moved
  - [x] Keep only essential imports
  - [x] Clean up module declarations
  - [x] Update `run()` function command registration as needed
  - [x] Add comprehensive module documentation

- [x] **Final testing and validation**
  - [x] Run full test suite: `cargo test --workspace --all-features`
  - [x] Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
  - [x] Run formatting check: `cargo fmt --all -- --check`
  - [x] Test application startup and hotkey registration
  - [x] Test settings save/load functionality
  - [x] Test debug panel (in debug mode)
  - [x] Verify all Tauri commands are accessible from frontend
  - [x] Check final line count of `lib.rs` (400 lines - reduced from ~1000+ original)

- [x] **Documentation updates**
  - [x] Update module documentation in each new file
  - [x] Add rustdoc examples where appropriate
  - [x] Fix failing doctests with correct crate names
  - [x] Update architectural documentation
  - [x] Create migration notes for future developers
