# Phase 2: Extract Services (Medium Risk)

**Objective**: Move service structs and related functionality to dedicated modules

## Task Checklist (Phase 2)

- [ ] **Create services module structure**
  - [ ] Create `speakr-tauri/src/services/` directory
  - [ ] Create `services/mod.rs` with module declarations
  - [ ] Create `services/types.rs` for shared enums
  - [ ] Create `services/hotkey.rs` for GlobalHotkeyService
  - [ ] Create `services/status.rs` for BackendStatusService

- [ ] **Extract ServiceComponent enum**
  - [ ] Move `ServiceComponent` enum → `services/types.rs`
  - [ ] Add appropriate derives and documentation
  - [ ] Re-export from `services/mod.rs`

- [ ] **Extract GlobalHotkeyService**
  - [ ] Move entire `GlobalHotkeyService` struct → `services/hotkey.rs`
  - [ ] Move all impl blocks and methods
  - [ ] Add necessary imports (tauri, tracing, etc.)
  - [ ] Extract `register_global_hotkey()` implementation → `services/hotkey.rs` as
        `register_global_hotkey_internal()`
  - [ ] Extract `unregister_global_hotkey()` implementation → `services/hotkey.rs` as
        `unregister_global_hotkey_internal()`
  - [ ] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [ ] Make service and methods `pub(crate)` for module visibility

- [ ] **Extract BackendStatusService**
  - [ ] Move `BackendStatusService` struct → `services/status.rs`
  - [ ] Move all impl blocks and methods
  - [ ] Move `GLOBAL_BACKEND_SERVICE` static → `services/status.rs`
  - [ ] Move `get_global_backend_service()` helper → `services/status.rs`
  - [ ] Move `update_global_service_status()` helper → `services/status.rs`
  - [ ] Extract `get_backend_status()` implementation → `services/status.rs` as
        `get_backend_status_internal()`
  - [ ] Extract `update_service_status()` implementation → `services/status.rs` as
        `update_service_status_internal()`
  - [ ] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [ ] Add necessary imports for Tauri AppHandle, etc.
  - [ ] Make all functions `pub(crate)` for module visibility
  - [ ] Add `Default` implementation

- [ ] **Update lib.rs imports and exports**
  - [ ] Add `mod services;` to `lib.rs`
  - [ ] Add `use services::*;` or specific imports
  - [ ] Remove original service implementations from `lib.rs`
  - [ ] Update command registration in `run()` function

- [ ] **Test service extraction**
  - [ ] Run `cargo check` to verify compilation
  - [ ] Run `cargo test --workspace` to ensure tests pass
  - [ ] Test hotkey registration functionality manually
  - [ ] Test status service functionality
