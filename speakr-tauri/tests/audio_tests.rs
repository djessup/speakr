// Audio-related tests extracted from lib.rs

use tempfile::TempDir;

// Import audio functions from their new module locations
use speakr_lib::audio::files::{
    generate_audio_filename_with_timestamp, save_audio_samples_to_wav_file,
};
use speakr_lib::audio::recording::{debug_record_audio_to_file, debug_record_real_audio_to_file};

#[tokio::test]
async fn test_debug_record_audio_to_file_saves_with_timestamp() {
    // Verifies that the helper creates a timestamped filename and non-empty WAV file
    use std::time::SystemTime;

    // Arrange
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let output_dir = temp_dir.path();

    let before_recording = SystemTime::now();

    // Act
    let file_path = debug_record_audio_to_file(output_dir, 2)
        .await
        .expect("Should record audio to file");

    let after_recording = SystemTime::now();

    // Assert
    assert!(file_path.exists(), "Audio file should be created");
    assert!(
        file_path.extension().unwrap_or_default() == "wav",
        "Should create WAV file"
    );

    // Check timestamp in filename
    let filename = file_path.file_name().unwrap().to_string_lossy();
    assert!(
        filename.starts_with("recording_"),
        "Filename should start with 'recording_'"
    );

    // File should contain actual audio data (not just empty)
    let metadata = std::fs::metadata(&file_path).expect("Should get file metadata");
    assert!(
        metadata.len() > 44,
        "WAV file should be larger than header (44 bytes)"
    ); // WAV header is 44 bytes

    // Verify file timestamp is between before/after recording
    let file_time = metadata.modified().expect("Should get modified time");
    assert!(
        file_time >= before_recording && file_time <= after_recording,
        "File timestamp should be within recording window"
    );
}

#[tokio::test]
async fn test_debug_record_audio_to_file_creates_unique_filenames() {
    // Ensures successive recordings generate unique filenames
    use tempfile::TempDir;

    // Arrange
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let output_dir = temp_dir.path();

    // Act - record two files quickly
    let file1 = debug_record_audio_to_file(output_dir, 1)
        .await
        .expect("Should record first audio file");

    // Small delay to ensure different timestamp
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    let file2 = debug_record_audio_to_file(output_dir, 1)
        .await
        .expect("Should record second audio file");

    // Assert
    assert_ne!(file1, file2, "Should create files with unique names");
    assert!(file1.exists(), "First file should exist");
    assert!(file2.exists(), "Second file should exist");
}

#[tokio::test]
async fn test_save_audio_samples_to_wav_file() {
    // Confirms WAV encoder writes a valid RIFF/WAVE header and data section
    use tempfile::TempDir;

    // Arrange
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let output_path = temp_dir.path().join("test_audio.wav");

    // Create test samples (simple sine wave pattern)
    let sample_rate = 16_000;
    let duration_secs = 2;
    let samples: Vec<i16> = (0..sample_rate * duration_secs)
        .map(|i| {
            ((((i as f64) * 2.0 * std::f64::consts::PI * 440.0) / (sample_rate as f64)).sin()
                * 16000.0) as i16
        })
        .collect();

    // Act
    save_audio_samples_to_wav_file(&samples, &output_path)
        .await
        .expect("Should save audio samples to WAV file");

    // Assert
    assert!(output_path.exists(), "WAV file should be created");

    let metadata = std::fs::metadata(&output_path).expect("Should get file metadata");
    assert!(metadata.len() > 44, "WAV file should be larger than header");

    // Check WAV header (first 4 bytes should be "RIFF")
    let file_content = std::fs::read(&output_path).expect("Should read file");
    assert_eq!(&file_content[0..4], b"RIFF", "Should have RIFF header");
    assert_eq!(&file_content[8..12], b"WAVE", "Should have WAVE format");
}

#[tokio::test]
async fn test_generate_audio_filename_with_timestamp() {
    // Validates timestamp components and uniqueness over time
    use std::time::SystemTime;

    // Act
    let _before = SystemTime::now();
    let filename = generate_audio_filename_with_timestamp();
    let _after = SystemTime::now();

    // Assert
    assert!(
        filename.starts_with("recording_"),
        "Should start with 'recording_'"
    );
    assert!(filename.ends_with(".wav"), "Should end with '.wav'");

    // Should contain timestamp components (year, month, day, hour, minute, second)
    assert!(
        filename.contains("2024") || filename.contains("2025"),
        "Should contain year"
    );

    // Generate another filename and ensure they're different
    let filename2 = generate_audio_filename_with_timestamp();
    if filename == filename2 {
        // If they're the same, wait a bit and try again
        tokio::time::sleep(std::time::Duration::from_millis(1001)).await;
        let filename3 = generate_audio_filename_with_timestamp();
        assert_ne!(filename, filename3, "Filenames should be unique over time");
    }
}

#[tokio::test]
#[ignore = "Requires audio hardware access - run manually with 'cargo test -- --ignored'"]
async fn test_debug_real_audio_recording_integration() {
    // Integration test for real audio capture – ignored by default because it needs hardware
    use tempfile::TempDir;

    // Arrange
    let temp_dir = TempDir::new().expect("Should create temp dir");
    let output_dir = temp_dir.path();

    // Test that we can record real audio and save to file
    // This would use the actual AudioRecorder from speakr-core

    // Act - This should do a real recording for 1 second
    let file_path = debug_record_real_audio_to_file(output_dir, 1)
        .await
        .expect("Should record real audio to file");

    // Assert
    assert!(file_path.exists(), "Real audio file should be created");

    let metadata = std::fs::metadata(&file_path).expect("Should get file metadata");
    // For 1 second of 16kHz mono audio, expect roughly:
    // 44 bytes (WAV header) + (16000 samples * 2 bytes/sample) = ~32044 bytes
    assert!(
        metadata.len() > 1000,
        "Real audio file should contain substantial data"
    );
    assert!(
        metadata.len() < 100_000,
        "File size should be reasonable for 1 second"
    );
}
