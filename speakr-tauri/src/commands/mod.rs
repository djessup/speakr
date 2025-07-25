// ============================================================================
//! Tauri Command Management
// ============================================================================
//!
//! This module organises all Tauri commands by functional domain:
//! - `validation` - Input validation commands
//! - `system` - System integration commands
//! - `legacy` - Backward-compatibility commands
//!
//! # Architecture
//!
//! Each command is implemented in two parts:
//! 1. `*_internal()` function in the appropriate module (contains the actual logic)
//! 2. `#[tauri::command]` wrapper in `lib.rs` (for Tauri registration)
//!
//! This separation allows for easy testing of command logic without Tauri overhead.
//!
//! # Adding New Commands
//!
//! To add a new command:
//! 1. Create the `*_internal()` function in the appropriate module
//! 2. Add a `#[tauri::command]` wrapper in `lib.rs` that calls the internal function
//! 3. Register the command in the `run()` function's `.invoke_handler()` call
//! 4. Add comprehensive tests for the internal function
//!
//! # Example Structure
//!
//! ```rust,no_run
//! use speakr_types::AppError;
//!
//! // In commands/validation.rs
//! pub async fn validate_input_internal(input: String) -> Result<(), AppError> {
//!     // Validation logic here
//!     Ok(())
//! }
//!
//! // In lib.rs
//! #[tauri::command]
//! async fn validate_input(input: String) -> Result<(), AppError> {
//!     validate_input_internal(input).await
//! }
//! ```

// =========================
// Module Declarations
// =========================

/// Backward-compatibility command implementations that will be removed in a
/// future major release. Keep additions here minimal and prefer creating new
/// commands in their own dedicated modules.
pub mod legacy;

/// Commands that interact with the underlying operating system, for example
/// reading the file-system or configuring auto-launch settings.
pub mod system;

/// Pure validation helpers that perform synchronous checks and return
/// structured `AppError`s without touching the outside world. These helpers
/// are units of business-logic and can be reused from e.g. settings services.
pub mod validation;

// ============================================================================
// End of File
// ============================================================================
