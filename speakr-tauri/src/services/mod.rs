//! Services module for managing backend service components.
//!
//! This module contains service implementations for:
//! - Global hotkey management
//! - Backend status tracking
//! - Service component types

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
