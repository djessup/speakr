//! Core library for Speakr's audio capture, transcription, and text injection pipeline.
//!
//! This crate provides the fundamental building blocks for privacy-first voice-to-text
//! functionality on macOS, with support for:
//!
//! - High-quality audio capture (16kHz mono, optimized for Whisper)
//! - Configurable recording duration (1-30 seconds)
//! - In-memory buffering (no temporary files)
//! - Comprehensive error handling
//! - Async/await support with tokio
//!
//! # Examples
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

//
// Modules
//
pub mod audio;
pub mod model;
