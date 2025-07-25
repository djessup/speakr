// ============================================================================
//! Model Abstractions & Utilities
//!
//! This *umbrella* module groups together everything related to **model
//! management** inside *Speakr* – that is model **metadata**, the canonical
//! **model list**, and the convenience helpers that keep that list up-to-date.
//!
//! Sub-modules:
//! - `metadata` – `ModelMetadata` plus filename ↔︎ enum helpers
//! - `list`   – `Model` enum with a variant for every officially supported
//!   model
//! - `list_updater` – fetch and merge the latest model index at runtime
//!
//! In the public API we re-export the most commonly used items so that callers
//! can simply `use speakr_core::model::*` without having to care about the
//! internal file layout.
//!
//! # Quick Example
//! ```no_run
//! use speakr_core::model::Model;
//!
//! let url = Model::LargeV3TurboQuantizedQ8_0.url();
//! println!("Download from: {url}");
//! ```
// ============================================================================

mod list;
mod list_updater;
mod metadata;

//
// Re-exports
//
pub use list::Model;
pub use list_updater::ModelListUpdater;
pub use metadata::{filename_to_variant_name, ModelMetadata};

// Only load the test file during testing
#[cfg(test)]
mod list_tests;
