//! Settings management module
//!
//! This module handles all settings-related functionality including:
//! - File I/O operations and persistence
//! - Version migrations
//! - Directory validation
//! - Tauri command implementations

pub mod commands;
pub mod migration;
pub mod persistence;
pub mod validation;

// Re-export functions needed by lib.rs and tests
pub use commands::{load_settings_internal, save_settings_internal};
pub use migration::migrate_settings;
pub use persistence::{
    get_settings_backup_path, get_settings_path, load_settings_from_dir, save_settings_to_dir,
    try_load_settings_file,
};
pub use validation::validate_settings_directory_permissions;
