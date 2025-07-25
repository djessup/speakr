// ============================================================================
//! Service Layer – Module Root
// ============================================================================
//!
//! This module contains service implementations for:
//! - **Global hotkey management** - Handles system-wide keyboard shortcuts
//! - **Backend status tracking** - Monitors service component health and readiness
//! - **Service component types** - Shared enums and types across services
//!
//! # Service Architecture
//!
//! Services in Speakr follow a singleton pattern with global state management:
//! 1. Each service has a `*Service` struct for managing state
//! 2. Services are accessed through `*_internal()` functions
//! 3. Global state is protected by mutex guards for thread safety
//! 4. Services emit status updates through the Tauri event system
//!
//! # Thread Safety
//!
//! All services are designed to be thread-safe and can be accessed from
//! multiple contexts (frontend events, background tasks, tests) without
//! data races or corruption.

pub mod hotkey;
pub mod status;
pub mod types;

// Re-export types that need to be public across modules
pub use types::ServiceComponent;

// Re-export status functions needed by lib.rs and tests
pub use status::{
    get_backend_status_internal, get_global_backend_service, update_global_service_status,
    update_service_status_internal, BackendStatusService,
};

// Re-export reset function for tests
#[cfg(any(test, debug_assertions))]
pub use status::reset_global_backend_service;
