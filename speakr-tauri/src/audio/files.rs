// ============================================================================
//! Audio File Helpers (`audio::files`)
//!
//! This module provides a minimal set of helper utilities for working with
//! audio files inside the **Speakr** Tauri application.  The helpers focus on
//! WAV-formatted *16-bit, mono, 16 kHz* files which are the preferred format
//! for downstream speech-recognition services used by Speakr.
//!
//! The public API purposely remains very small and is **only** intended for
//! *internal* use by the surrounding `audio` sub-crate and tests.  No item in
//! this module is currently exposed to the JavaScript layer through Tauri
//! commands.
//!
//! # Provided Functionality
//! 1. `generate_audio_filename_with_timestamp` – Creates a unique, timestamped
//!    filename suitable for recordings.
//! 2. `save_audio_samples_to_wav_file` – Persists an in-memory slice of `i16`
//!    samples to an on-disk WAV file using the optimal recording spec.
// ============================================================================

// =========================
// External Imports
// =========================
use hound::{WavSpec, WavWriter};
use speakr_types::AppError;
use std::path::PathBuf;

// ============================================================================
// Filename Utilities
// ============================================================================

// --------------------------------------------------------------------------
/// Generate a filename for a new audio recording.
///
/// The filename is built from the current UTC timestamp with millisecond
/// precision so that multiple recordings in the same second never collide.
/// The resulting string follows the pattern:
///
/// ```text
/// recording_YYYY-MM-DD_HH-MM-SS.mmm.wav
/// ```
///
/// # Returns
/// A `String` containing the formatted filename **without** any directory
/// component – callers are expected to prepend their desired output path.
///
/// # Examples
/// ```ignore
/// use crate::generate_audio_filename_with_timestamp;
/// let fname = generate_audio_filename_with_timestamp();
/// assert!(fname.starts_with("recording_"));
/// assert!(fname.ends_with(".wav"));
/// ```
///
/// # Internal API
/// This helper is kept pub(crate) to allow sharing across the `audio` module
/// and its tests – it **must not** be exposed to the frontend.
pub fn generate_audio_filename_with_timestamp() -> String {
    let now = chrono::Utc::now();
    format!("recording_{}.wav", now.format("%Y-%m-%d_%H-%M-%S%.3f"))
}

// ============================================================================
// WAV Persistence Helpers
// ============================================================================

// --------------------------------------------------------------------------
/// Persist raw PCM samples (`i16`) to a 16-bit mono WAV file on disk.
///
/// # Arguments
/// * `samples`      – The in-memory audio samples to write.
/// * `output_path`  – Full filesystem path (including filename) where the WAV
///   file should be created.
///
/// # Returns
/// `Ok(())` on success.
///
/// # Errors
/// Returns an `AppError::FileSystem` if:
/// * The parent directory does not exist.
/// * The file cannot be created or written to.
/// * Finalising the WAV writer fails.
///
/// # WAV Configuration
/// The produced file always adheres to the following audio spec:
/// * **Channels**: 1 (mono)
/// * **Sample rate**: 16 kHz
/// * **Bit depth**: 16-bit signed integers
///
/// # Internal API
/// Just like `generate_audio_filename_with_timestamp`, this function is
/// intended for *internal* consumption only and is not wired up to any Tauri
/// command.
pub async fn save_audio_samples_to_wav_file(
    samples: &[i16],
    output_path: &PathBuf,
) -> Result<(), AppError> {
    // WAV specification optimised for speech recognition
    let spec = WavSpec {
        channels: 1,         // Mono audio
        sample_rate: 16_000, // 16 kHz sample rate (common for speech)
        bits_per_sample: 16, // 16-bit depth
        sample_format: hound::SampleFormat::Int,
    };

    // Validate output path directory exists
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            return Err(AppError::FileSystem(format!(
                "Output directory does not exist: {}",
                parent.display()
            )));
        }
    }

    let mut writer = WavWriter::create(output_path, spec)
        .map_err(|e| AppError::FileSystem(format!("Failed to create WAV file: {e}")))?;

    // Write all samples to the file
    for &sample in samples {
        writer
            .write_sample(sample)
            .map_err(|e| AppError::FileSystem(format!("Failed to write audio sample: {e}")))?;
    }

    writer
        .finalize()
        .map_err(|e| AppError::FileSystem(format!("Failed to finalize WAV file: {e}")))?;

    Ok(())
}

// ===========================================================================
// End of File
// ===========================================================================
