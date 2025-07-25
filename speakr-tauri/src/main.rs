// ============================================================================
//! Speakr – Application entry point.
//!
//! This binary launches the Speakr backend implemented in [`speakr_lib`].  It is
//! intentionally lightweight: all heavy lifting (Tauri window creation, plugin
//! initialisation, command registration, logging, etc.) is delegated to
//! [`speakr_lib::run`].  Keeping the `main.rs` minimal ensures that the library
//! API remains the single authority for starting the application in both
//! desktop and (potentially) mobile contexts.
//!
//! # Platform notes
//! * **Windows release builds** – The
//!   `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`
//!   attribute disables the console subsystem so that no additional console
//!   window is shown to end-users.
//!
//! # Exit behaviour
//! The process exits when the Tauri event loop ends. Any panics or unrecoverable
//! errors inside the backend will propagate and terminate the process with a
//! non-zero exit code.
// ============================================================================

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// =========================
// External Imports
// =========================

// ============================================================================
// Application Entry Point
// ============================================================================

// --------------------------------------------------------------------------
/// Launches the Speakr backend.
///
/// This function simply calls [`speakr_lib::run`]. It does not return under
/// normal circumstances because the Tauri runtime blocks the current thread
/// until the application exits.
fn main() {
    speakr_lib::run();
}

// ===========================================================================
