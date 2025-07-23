// ============================================================================
//! Core library for Speakr's audio capture, transcription, and text injection pipeline.
//!
//! This crate provides the fundamental building blocks for privacy-first voice-to-text
//! functionality on macOS, with support for:
//!
//! - **High-quality audio capture**: 16kHz mono sampling optimised for Whisper models
//! - **Configurable recording duration**: 1-30 seconds with intelligent timeout handling
//! - **In-memory buffering**: No temporary files for enhanced privacy
//! - **Comprehensive error handling**: Robust error propagation with custom error types
//! - **Async/await support**: Built on tokio for non-blocking operations
//! - **Model management**: Download, validate, and load Whisper GGUF models
//!
//! # Architecture
//!
//! The crate is organised into two main modules:
//! - [`audio`] - Audio capture and recording functionality
//! - [`model`] - Whisper model management and metadata handling
//!
//! # Usage
//!
//! Basic audio recording workflow:
//!
//! ```no_run
//! use speakr_core::audio::{AudioRecorder, RecordingConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create recorder with 5-second limit
//!     let config = RecordingConfig::new(5);
//!     let recorder = AudioRecorder::new(config).await?;
//!
//!     // Start recording
//!     recorder.start_recording().await?;
//!
//!     // Wait for user input or timeout
//!     tokio::time::sleep(std::time::Duration::from_secs(3)).await;
//!
//!     // Stop and get samples
//!     let result = recorder.stop_recording().await?;
//!     let samples = result.samples();
//!
//!     println!("Captured {} audio samples", samples.len());
//!     Ok(())
//! }
//! ```
//!
//! # Privacy and Security
//!
//! This crate is designed with privacy as the primary concern:
//! - All audio processing happens in-memory
//! - No network requests or external data transmission
//! - Models are validated with checksums before use
//! - Temporary files are avoided wherever possible
//!
//! # Performance Considerations
//!
//! - Audio samples are captured at 16kHz mono for optimal balance of quality and size
//! - Recording buffers are pre-allocated to minimise runtime allocations
//! - Model loading is performed once at startup to reduce latency
//!
//! # Error Handling
//!
//! All public functions return `Result` types with descriptive error messages.
//! Refer to individual module documentation for specific error conditions.
// ============================================================================

// =========================
// Module Declarations
// =========================

/// Audio capture and recording functionality.
///
/// Provides high-quality audio recording optimised for speech recognition,
/// with configurable duration limits and in-memory buffering for privacy.
pub mod audio;

/// Whisper model management and metadata handling.
///
/// Handles downloading, validation, and loading of Whisper GGUF models
/// with proper checksum verification and metadata extraction.
pub mod model;

// ===========================================================================
