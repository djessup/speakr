//! Audio file operations and WAV file utilities.

use hound::{WavSpec, WavWriter};
use speakr_types::AppError;
use std::path::PathBuf;

/// Generates an audio filename with current timestamp
///
/// # Returns
///
/// A filename string in the format "recording_YYYY-MM-DD_HH-MM-SS.wav"
///
/// # Internal API
/// This function is only intended for internal use and testing.
pub fn generate_audio_filename_with_timestamp() -> String {
    let now = chrono::Utc::now();
    format!("recording_{}.wav", now.format("%Y-%m-%d_%H-%M-%S%.3f"))
}

/// Saves audio samples to a WAV file
///
/// # Arguments
///
/// * `samples` - The audio samples to save (16-bit mono)
/// * `output_path` - The path where the WAV file should be saved
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the file cannot be written.
///
/// # WAV Configuration
///
/// The WAV file is configured with:
/// - Channels: 1 (Mono)
/// - Sample rate: 16,000 Hz (16 kHz)
/// - Bits per sample: 16-bit
/// - Sample format: Signed integer
///
/// # Internal API
/// This function is only intended for internal use and testing.
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
