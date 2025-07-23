//! Debug command implementations.
//!
//! This module contains the internal implementations of debug Tauri commands
//! for audio recording tests, log management, and debug panel functionality.

use crate::debug::{
    storage::{DEBUG_LOG_MESSAGES, DEBUG_RECORDING_STATE},
    types::{DebugLogLevel, DebugLogMessage},
};
use speakr_core::audio::{AudioRecorder, RecordingConfig};
use speakr_types::AppError;
use std::{fs, path::PathBuf, time::Duration};
use tracing::info;

/// Internal implementation for debug audio recording test
///
/// # Returns
///
/// Returns a success message for testing purposes.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
pub async fn debug_test_audio_recording_internal() -> Result<String, AppError> {
    crate::debug::storage::add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        "Starting audio recording test",
    );

    // Simulate some processing time
    tokio::time::sleep(Duration::from_millis(500)).await;

    crate::debug::storage::add_debug_log(
        DebugLogLevel::Debug,
        "speakr-core",
        "Mock audio recording completed",
    );

    // Return a mock success result
    Ok("Audio recording test completed successfully! (Mock implementation)".to_string())
}

/// Gets the default output directory for debug audio recordings
///
/// # Returns
///
/// Returns the path to the user's Documents/Speakr/debug_recordings/ directory.
///
/// # Errors
///
/// Returns `AppError` if the directory cannot be created.
pub fn get_debug_recordings_directory() -> Result<PathBuf, AppError> {
    let documents_dir = dirs::document_dir()
        .ok_or_else(|| AppError::Settings("Could not find Documents directory".to_string()))?;

    let debug_dir = documents_dir.join("Speakr").join("debug_recordings");

    // Create directory if it doesn't exist
    if !debug_dir.exists() {
        fs::create_dir_all(&debug_dir).map_err(|e| {
            AppError::FileSystem(format!("Failed to create debug recordings dir: {e}"))
        })?;
    }

    Ok(debug_dir)
}

/// Internal implementation for debug start recording command
///
/// # Returns
///
/// Returns a success message indicating recording has started.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
pub async fn debug_start_recording_internal() -> Result<String, AppError> {
    info!("🎙️ Debug: Starting real push-to-talk recording");
    crate::debug::storage::add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        "Real push-to-talk recording started",
    );

    // Check if already recording
    {
        let state = DEBUG_RECORDING_STATE.lock().unwrap();
        if state.recorder.is_some() {
            return Ok("Recording already in progress".to_string());
        }
    }

    // Create audio recorder with 30 second max duration (push-to-talk should be shorter)
    let config = RecordingConfig::new(30);
    let recorder = AudioRecorder::new(config)
        .await
        .map_err(|e| AppError::Settings(format!("Failed to create audio recorder: {e}")))?;

    // Start recording
    recorder
        .start_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to start recording: {e}")))?;

    // Store recorder in global state
    {
        let mut state = DEBUG_RECORDING_STATE.lock().unwrap();
        state.recorder = Some(recorder);
        state.start_time = Some(std::time::Instant::now());
    }

    crate::debug::storage::add_debug_log(
        DebugLogLevel::Info,
        "speakr-core",
        "Real audio recording started successfully",
    );

    Ok("🎙️ Real recording started! Release button to stop and save.".to_string())
}

/// Internal implementation for debug stop recording command
///
/// # Returns
///
/// Returns a message with the file path where audio was saved.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
pub async fn debug_stop_recording_internal() -> Result<String, AppError> {
    info!("⏹️ Debug: Stopping real push-to-talk recording and saving to disk");

    // Get recorder from global state
    let (recorder, start_time) = {
        let mut state = DEBUG_RECORDING_STATE.lock().unwrap();
        let recorder = state.recorder.take();
        let start_time = state.start_time.take();
        (recorder, start_time)
    };

    let Some(recorder) = recorder else {
        crate::debug::storage::add_debug_log(
            DebugLogLevel::Warn,
            "speakr-debug",
            "No active recording to stop",
        );
        return Ok("No recording was active".to_string());
    };

    // Stop recording and get samples
    let result = recorder
        .stop_recording()
        .await
        .map_err(|e| AppError::Settings(format!("Failed to stop recording: {e}")))?;

    let samples = result.samples();
    let duration = start_time.map(|t| t.elapsed()).unwrap_or_default();

    crate::debug::storage::add_debug_log(
        DebugLogLevel::Info,
        "speakr-debug",
        &format!(
            "Recording stopped, captured {} samples in {:.2}s",
            samples.len(),
            duration.as_secs_f64()
        ),
    );

    // Save to file in debug recordings directory
    let output_dir = get_debug_recordings_directory()?;
    let filename = crate::audio::files::generate_audio_filename_with_timestamp();
    let output_path = output_dir.join(filename);

    crate::audio::files::save_audio_samples_to_wav_file(&samples, &output_path).await?;

    let success_message = format!(
        "⏹️ Recording saved! {} samples ({:.2}s) → {}",
        samples.len(),
        duration.as_secs_f64(),
        output_path.display()
    );

    crate::debug::storage::add_debug_log(DebugLogLevel::Info, "speakr-debug", &success_message);

    info!("{}", success_message);
    Ok(success_message)
}

/// Internal implementation for getting log messages
///
/// # Returns
///
/// Returns a vector of log messages.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
pub async fn debug_get_log_messages_internal() -> Result<Vec<DebugLogMessage>, AppError> {
    if let Ok(logs) = DEBUG_LOG_MESSAGES.lock() {
        Ok(logs.iter().cloned().collect())
    } else {
        Err(AppError::Settings(
            "Failed to access log messages".to_string(),
        ))
    }
}

/// Internal implementation for clearing log messages
///
/// # Returns
///
/// Returns `Ok(())` on success.
///
/// # Errors
///
/// Returns `AppError` if the operation fails.
pub async fn debug_clear_log_messages_internal() -> Result<(), AppError> {
    if let Ok(mut logs) = DEBUG_LOG_MESSAGES.lock() {
        logs.clear();
        crate::debug::storage::add_debug_log(
            DebugLogLevel::Info,
            "speakr-debug",
            "Log messages cleared",
        );
        Ok(())
    } else {
        Err(AppError::Settings(
            "Failed to clear log messages".to_string(),
        ))
    }
}
