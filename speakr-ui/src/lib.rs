// ============================================================================
//! Speakr Web-UI (Leptos + WASM)
//!
//! This crate delivers the browser-based user-interface for the **Speakr** project.
//! It is compiled to WebAssembly and embedded in the Tauri frontend, but can also
//! be served standalone during development.
//!
//! High-level responsibilities:
//! 1. Render the root [`App`] component.
//! 2. Provide settings forms and debug panels (conditionally compiled).
//! 3. Bootstrap the WASM runtime and mount the virtual-DOM into the document
//!    body.
//!
//! The file follows the documentation layout defined in
//! `docs/refactor/RUST_DOCUMENTATION_TRACKING.md`:
//!
//! ```text
//! // ============================================================================  ← file header (you are here)
//! // Module Declarations
//! // External Imports
//! // WASM Boot-strap function
//! // ============================================================================
//! ```
// ============================================================================

// =========================
// Module Declarations
// =========================
mod app;
mod settings;

// Debug-only UI panels
#[cfg(debug_assertions)]
mod debug;

// =========================
// External Imports
// =========================
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// Re-export root component so integration tests can mount it directly
pub use app::App;

// ============================================================================
// WASM Boot-strap Function
// ============================================================================
/// Entry point invoked automatically by the JavaScript loader that initialises
/// the WASM module. It:
/// 1. Installs a panic-hook so Rust panics are forwarded to the browser console.
/// 2. Mounts the [`App`] component to the document body.
///
/// # Panics
/// This function should never panic itself—any panic inside Leptos will be
/// caught by `console_error_panic_hook` and forwarded to the browser console
/// for easier debugging.
#[wasm_bindgen(start)]
pub fn main() {
    // Better error messages when a Rust panic bubbles up to JS.
    console_error_panic_hook::set_once();

    // Mount the root component into the HTML <body> element.
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

// ============================================================================
