// ============================================================================
//! Audio utilities for the Tauri desktop application layer.
//!
//! This sub–crate of **speakr-tauri** provides a thin, *synchronous* façade over
//! the more fully-featured asynchronous audio helpers that live in
//! `speakr-core`.  The goal is to expose just enough functionality for the
//! Tauri command handlers and debug helpers without leaking implementation
//! details to the JavaScript runtime.
//!
//! At a high-level the module is split into two concerns:
//!
//! 1. **File helpers** – Creating `.wav` files from raw `i16` sample buffers
//!    and generating timestamped filenames that match the Speakr naming
//!    conventions.
//! 2. **Recording helpers** – Debug-only utilities that capture audio from the
//!    system microphone and write it to disk so that we have real-world
//!    samples during development.
//!
//! The public API that the rest of **speakr-tauri** should depend on is kept
//! intentionally small and is re-exported at the root of this module for
//! convenience:
//!
//! ```ignore
//! use crate::{
//!     generate_audio_filename_with_timestamp,
//!     save_audio_samples_to_wav_file,
//! };
//!
//! # let samples: Vec<i16> = Vec::new();
//! let filename = generate_audio_filename_with_timestamp();
//! let _ = save_audio_samples_to_wav_file(&samples, &std::path::PathBuf::from(filename));
//! ```
//!
//! The actual implementation lives in child modules so there is no executable
//! logic in this file.
// ============================================================================

// ============================================================================
// Module Declarations
// ============================================================================
pub mod files;
pub mod recording;

// ============================================================================
// Public Re-Exports
// ============================================================================

// --------------------------------------------------------------------------
/// Generate a timestamped filename that can be used for audio recordings.
///
/// See [`files::generate_audio_filename_with_timestamp`] for full
/// documentation.
pub use files::generate_audio_filename_with_timestamp;

// --------------------------------------------------------------------------
/// Write raw `i16` audio samples to a `.wav` file.
///
/// See [`files::save_audio_samples_to_wav_file`] for full documentation.
pub use files::save_audio_samples_to_wav_file;

// --------------------------------------------------------------------------
/// Debug-only helper that records a short chunk of audio and writes it to a
/// temporary file.  Refer to [`recording::debug_record_audio_to_file`] for
/// details.
#[cfg(debug_assertions)]
pub use recording::debug_record_audio_to_file;

// --------------------------------------------------------------------------
/// Debug-only helper that records **real-time** audio—i.e. whatever is passing
/// through the microphone—and writes it to disk.  Refer to
/// [`recording::debug_record_real_audio_to_file`] for details.
#[cfg(debug_assertions)]
pub use recording::debug_record_real_audio_to_file;

// ============================================================================
// End of File
// ============================================================================
