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
  - [ ] Move `register_global_hotkey()` command → `services/hotkey.rs`
  - [ ] Move `unregister_global_hotkey()` command → `services/hotkey.rs`
  - [ ] Add `pub` visibility to service and methods as needed

- [ ] **Extract BackendStatusService**
  - [ ] Move `BackendStatusService` struct → `services/status.rs`
  - [ ] Move all impl blocks and methods
  - [ ] Move `get_backend_status()` command → `services/status.rs`
  - [ ] Add necessary imports for Tauri AppHandle, etc.
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
