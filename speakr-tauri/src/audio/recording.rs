// ============================================================================
//! Audio Recording Utilities (Debug-Only)
//!
//! This module contains **debug-only** helper utilities for recording audio and
//! persisting it to disk while running the Speakr Tauri application.  Two
//! flavours of recording are provided:
//!
//! 1. [`debug_record_audio_to_file`] – Generates *synthetic* audio samples (a
//!    440 Hz sine wave) entirely in memory and stores them as a WAV file.  This
//!    helper is useful when you need deterministic, microphone-independent test
//!    data.
//! 2. [`debug_record_real_audio_to_file`] – Invokes the [`speakr_core::audio`]
//!    capture pipeline to record **real** microphone input for a fixed
//!    duration and dumps the result to a WAV file.
//!
//! Both functions are `#[allow(dead_code)]` and are not compiled into release
//! builds; they exist strictly to ease local testing and debugging.  They are
//! therefore *not* part of the public API surface exposed to the frontend.
//!
//! # Usage
//!
//! ```ignore
//! use std::path::Path;
//! use crate::debug_record_real_audio_to_file;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), speakr_types::AppError> {
//! let output = debug_record_real_audio_to_file(Path::new("/tmp"), 3).await?;
//! println!("Recorded to {}", output.display());
//! # Ok(())
//! # }
//! ```
// ============================================================================

// =========================
// External Imports
// =========================

use crate::audio::files::{generate_audio_filename_with_timestamp, save_audio_samples_to_wav_file};
use speakr_core::audio::{AudioRecorder, RecordingConfig};
use speakr_types::AppError;
use std::path::{Path, PathBuf};

// ============================================================================
// Synthetic Audio Recording Helpers
// ============================================================================

// --------------------------------------------------------------------------
/// Generate **synthetic** audio samples and store them as a WAV file on disk.
///
/// The generated signal is a pure 440 Hz sine wave (`A4`) sampled at
/// `16_000 Hz`.  This deterministic data is particularly handy when you want
/// repeatable tests without depending on real hardware.
///
/// # Arguments
/// * `output_dir` – Directory where the WAV file should be created.
/// * `duration_secs` – Desired length of the recording in seconds.
///
/// # Returns
/// The absolute [`PathBuf`] of the file written to disk.
///
/// # Errors
/// Returns [`AppError`] if the samples cannot be written to disk.
///
/// # Internal API
/// This helper is gated behind `#[allow(dead_code)]` and is **not** included in
/// production builds.
#[allow(dead_code)]
pub async fn debug_record_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // Generate an output file path with a timestamp so that multiple runs do
    // not collide.
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    // --------------------------------------------------
    // Synthesise 16-bit mono sine-wave samples
    // --------------------------------------------------
    let sample_rate = 16_000_u32;
    let total_samples = sample_rate * duration_secs;

    let samples: Vec<i16> = (0..total_samples)
        .map(|i| {
            let t = i as f64 / sample_rate as f64;
            let amplitude = 16_000.0; // Safe range for 16-bit audio (±32k)
            (amplitude * (2.0 * std::f64::consts::PI * 440.0 * t).sin()) as i16
        })
        .collect();

    // Persist samples to disk
    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}

// ===========================================================================

// ============================================================================
// Real Microphone Recording Helpers
// ============================================================================

// --------------------------------------------------------------------------
/// Record **real** microphone input for a fixed duration and store it as a WAV
/// file.
///
/// Internally this leverages [`speakr_core::audio::AudioRecorder`] to handle
/// the platform-specific capture mechanics.
///
/// # Arguments
/// * `output_dir` – Directory where the resulting WAV file will be written.
/// * `duration_secs` – Number of seconds to capture audio.
///
/// # Returns
/// The absolute [`PathBuf`] pointing at the newly created WAV file.
///
/// # Errors
/// Any error bubbling up from the underlying audio recorder or filesystem is
/// wrapped into [`AppError`].
///
/// # Internal API
/// Like its synthetic counterpart, this function is only compiled in debug
/// builds.
#[allow(dead_code)]
pub async fn debug_record_real_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // --------------------------------------------------
    // Configure and start the recorder
    // --------------------------------------------------
    let config = RecordingConfig::new(duration_secs);
    let recorder = AudioRecorder::new(config)
        .await
        .map_err(|e| AppError::Settings(format!("Failed to create audio recorder: {e}")))?;

    recorder
        .start_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to start recording: {e}")))?;

    // Block the current task for the desired duration. The rest of the
    // application continues to run because we're inside an async runtime.
    tokio::time::sleep(std::time::Duration::from_secs(duration_secs as u64)).await;

    // --------------------------------------------------
    // Finalise recording and persist to disk
    // --------------------------------------------------
    let result = recorder
        .stop_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to stop recording: {e}")))?;

    let samples = result.samples();

    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}
