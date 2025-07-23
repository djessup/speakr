# Phase 2: Extract Services (Medium Risk)

**Objective**: Move service structs and related functionality to dedicated modules

## Task Checklist (Phase 2)

- [x] **Create services module structure**
  - [x] Create `speakr-tauri/src/services/` directory
  - [x] Create `services/mod.rs` with module declarations
  - [x] Create `services/types.rs` for shared enums
  - [x] Create `services/hotkey.rs` for GlobalHotkeyService
  - [x] Create `services/status.rs` for BackendStatusService

- [x] **Extract ServiceComponent enum**
  - [x] Move `ServiceComponent` enum → `services/types.rs`
  - [x] Add appropriate derives and documentation
  - [x] Re-export from `services/mod.rs`

- [x] **Extract GlobalHotkeyService**
  - [x] Move entire `GlobalHotkeyService` struct → `services/hotkey.rs`
  - [x] Move all impl blocks and methods
  - [x] Add necessary imports (tauri, tracing, etc.)
  - [x] Extract `register_global_hotkey()` implementation → `services/hotkey.rs` as
        `register_global_hotkey_internal()`
  - [x] Extract `unregister_global_hotkey()` implementation → `services/hotkey.rs` as
        `unregister_global_hotkey_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Make service and methods `pub(crate)` for module visibility

- [x] **Extract BackendStatusService**
  - [x] Move `BackendStatusService` struct → `services/status.rs`
  - [x] Move all impl blocks and methods
  - [x] Move `GLOBAL_BACKEND_SERVICE` static → `services/status.rs`
  - [x] Move `get_global_backend_service()` helper → `services/status.rs`
  - [x] Move `update_global_service_status()` helper → `services/status.rs`
  - [x] Extract `get_backend_status()` implementation → `services/status.rs` as
        `get_backend_status_internal()`
  - [x] Extract `update_service_status()` implementation → `services/status.rs` as
        `update_service_status_internal()`
  - [x] Keep `#[tauri::command]` wrappers in `lib.rs` that call `_internal` functions
  - [x] Add necessary imports for Tauri AppHandle, etc.
  - [x] Make all functions `pub(crate)` for module visibility
  - [x] Add `Default` implementation

- [x] **Update lib.rs imports and exports**
  - [x] Add `mod services;` to `lib.rs`
  - [x] Add `use services::*;` or specific imports
  - [x] Remove original service implementations from `lib.rs`
  - [x] Update command registration in `run()` function

- [x] **Test service extraction**
  - [x] Run `cargo check` to verify compilation
  - [x] Run `cargo test --workspace` to ensure tests pass
  - [x] Test hotkey registration functionality manually
  - [x] Test status service functionality
