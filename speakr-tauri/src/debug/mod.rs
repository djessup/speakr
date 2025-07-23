//! Debug functionality module.
//!
//! This module provides debugging utilities for the Speakr application,
//! including log message storage, audio recording test commands, and
//! debug console functionality. All debug code is conditionally compiled
//! and only available in debug builds.

#[cfg(debug_assertions)]
pub mod commands;
#[cfg(debug_assertions)]
pub mod storage;
#[cfg(debug_assertions)]
pub mod types;

// Re-export types for lib.rs to use
#[cfg(debug_assertions)]
pub use types::{DebugLogLevel, DebugLogMessage};

// Re-export functions that lib.rs needs to access
#[cfg(debug_assertions)]
pub use commands::{
    debug_clear_log_messages_internal, debug_get_log_messages_internal,
    debug_start_recording_internal, debug_stop_recording_internal,
    debug_test_audio_recording_internal,
};
#[cfg(debug_assertions)]
pub use storage::add_debug_log;
