//! Audio recording functionality and test utilities.

use crate::audio::files::{generate_audio_filename_with_timestamp, save_audio_samples_to_wav_file};
use speakr_core::audio::{AudioRecorder, RecordingConfig};
use speakr_types::AppError;
use std::path::{Path, PathBuf};

/// Records mock audio data to a file for testing
///
/// Creates synthetic audio data (sine wave) for testing purposes
/// without requiring actual microphone access.
///
/// # Arguments
///
/// * `output_dir` - Directory where the audio file should be saved
/// * `duration_secs` - Recording duration in seconds
///
/// # Returns
///
/// Returns the path to the created audio file.
///
/// # Errors
///
/// Returns `AppError` if recording or file saving fails.
///
/// # Internal API
/// This function is only intended for internal use and testing.
#[allow(dead_code)] // Used only in tests
pub async fn debug_record_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // Generate filename with timestamp
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    // Create mock audio samples for testing (simple sine wave at 440Hz - A note)
    let sample_rate = 16_000;
    let samples: Vec<i16> = (0..sample_rate * duration_secs)
        .map(|i| {
            let t = (i as f64) / (sample_rate as f64);
            let frequency = 440.0; // A note frequency
            let amplitude = 16000.0; // Safe amplitude for 16-bit audio
            (amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin()) as i16
        })
        .collect();

    // Save to WAV file
    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}

/// Records real audio data to a file using speakr-core AudioRecorder
///
/// This function performs actual audio recording using the system microphone
/// and saves the results to a WAV file.
///
/// # Arguments
///
/// * `output_dir` - Directory where the audio file should be saved
/// * `duration_secs` - Recording duration in seconds
///
/// # Returns
///
/// Returns the path to the created audio file.
///
/// # Errors
///
/// Returns `AppError` if recording or file saving fails.
///
/// # Internal API
/// This function is only intended for internal use and testing.
#[allow(dead_code)] // Used only in tests
pub async fn debug_record_real_audio_to_file(
    output_dir: &Path,
    duration_secs: u32,
) -> Result<PathBuf, AppError> {
    // Create recorder with the specified duration
    let config = RecordingConfig::new(duration_secs);
    let recorder = AudioRecorder::new(config)
        .await
        .map_err(|e| AppError::Settings(format!("Failed to create audio recorder: {e}")))?;

    // Start recording
    recorder
        .start_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to start recording: {e}")))?;

    // Wait for the recording duration
    tokio::time::sleep(std::time::Duration::from_secs(duration_secs as u64)).await;

    // Stop recording and get samples
    let result = recorder
        .stop_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to stop recording: {e}")))?;

    let samples = result.samples();

    // Generate filename with timestamp and save to file
    let filename = generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    save_audio_samples_to_wav_file(&samples, &output_path).await?;

    Ok(output_path)
}
