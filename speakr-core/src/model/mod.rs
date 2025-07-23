//!
//! Model module
//!
//! This module provides the model types and functionality for the Speakr application.
//!
//! It includes the following submodules:
//!
//! - `metadata`: Model metadata types
//! - `list`: Model list types
//! - `list_updater`: updater functions for refreshing the model list
//!
//! # Examples
//!
//! ```rust
//! use speakr_core::model::Model;
//! println!("{}", Model::LargeV3TurboQuantizedQ8_0.url());
//! ```

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
