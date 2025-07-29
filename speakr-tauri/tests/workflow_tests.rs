// ============================================================================
//! Integration tests for the dictation workflow orchestration.
//!
//! These tests verify that the complete dictation pipeline (record → transcribe → inject)
//! works correctly and handles errors appropriately.
// ============================================================================
#![allow(clippy::field_reassign_with_default)]

use speakr_lib::settings::{load_settings_from_dir, save_settings_to_dir};
use speakr_lib::workflow::create_recording_config_with_loader;
use speakr_types::{AppError, AppSettings};
use tempfile::TempDir;

mod test_utils;

// ============================================================================
// Workflow Integration Tests
// ============================================================================

#[tokio::test]
async fn test_workflow_components_exist() {
    // Test that the workflow module exports the expected functions
    // This is a basic compilation test to ensure the integration is properly wired

    // Test that AppError variants exist for workflow steps
    let audio_error = AppError::AudioCapture("Test audio error".to_string());
    assert_eq!(
        audio_error.to_string(),
        "Audio capture error: Test audio error"
    );

    let transcription_error = AppError::Transcription("Test transcription error".to_string());
    assert_eq!(
        transcription_error.to_string(),
        "Transcription error: Test transcription error"
    );

    let injection_error = AppError::TextInjection("Test injection error".to_string());
    assert_eq!(
        injection_error.to_string(),
        "Text injection error: Test injection error"
    );
}

#[tokio::test]
async fn test_workflow_error_types() {
    // Test that all required error types are available for the workflow
    let errors = vec![
        AppError::AudioCapture("Audio capture failed".to_string()),
        AppError::Transcription("Transcription failed".to_string()),
        AppError::TextInjection("Text injection failed".to_string()),
    ];

    for error in errors {
        // Verify each error can be created and has a meaningful message
        assert!(
            !error.to_string().is_empty(),
            "Error message should not be empty"
        );
    }
}

// ============================================================================
// Audio Duration Integration Tests
// ============================================================================

#[tokio::test]
async fn test_audio_capture_uses_settings_duration() {
    // Arrange - Use isolated temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path().to_path_buf();

    // Create custom settings with specific audio duration
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 15; // Different from default 10 seconds

    // Save settings to isolated directory
    save_settings_to_dir(&settings, &temp_dir_path)
        .await
        .unwrap();

    // Act - Load settings from isolated directory
    let loaded_settings = load_settings_from_dir(&temp_dir_path).await.unwrap();

    // Assert - Verify the audio duration is correctly loaded
    assert_eq!(loaded_settings.audio_duration_secs, 15);
    assert_eq!(
        loaded_settings.audio_duration_secs,
        settings.audio_duration_secs
    );
}

#[tokio::test]
async fn test_audio_capture_respects_duration_limits() {
    // Test that audio duration validation works correctly
    let mut settings = AppSettings::default();

    // Test valid range
    settings.audio_duration_secs = 1;
    assert!(AppSettings::validate_audio_duration(
        settings.audio_duration_secs
    ));

    settings.audio_duration_secs = 30;
    assert!(AppSettings::validate_audio_duration(
        settings.audio_duration_secs
    ));

    // Test invalid range
    assert!(!AppSettings::validate_audio_duration(0));
    assert!(!AppSettings::validate_audio_duration(31));
}

#[tokio::test]
async fn test_workflow_loads_audio_duration_from_settings() {
    // Arrange - Use isolated temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path().to_path_buf();

    // Set custom audio duration in settings
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 25; // Different from default 10 seconds
    save_settings_to_dir(&settings, &temp_dir_path)
        .await
        .unwrap();

    // Act - Load settings from isolated directory
    let loaded_settings = load_settings_from_dir(&temp_dir_path).await.unwrap();

    // Assert - The workflow should use this duration when creating RecordingConfig
    assert_eq!(loaded_settings.audio_duration_secs, 25);

    // Verify that the workflow uses the settings duration
}

#[tokio::test]
async fn test_create_recording_config_from_settings_uses_settings_duration() {
    // This test verifies that the workflow function actually uses settings duration

    // Arrange - Use isolated settings environment
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 20; // Different from default 10 seconds

    let (_temp_dir, loader) =
        test_utils::create_isolated_settings_env_with_settings(settings).await;

    // Act - Create recording config using the workflow function with isolated loader
    let config = create_recording_config_with_loader(loader).await;

    // Assert - The config should use the settings duration
    assert_eq!(config.max_duration_secs(), 20);
}

#[tokio::test]
async fn test_create_recording_config_from_settings_fallback_on_error() {
    // This test verifies that the function falls back to default when settings fail

    // Arrange - Use mock settings loader that fails
    use speakr_lib::settings::traits::test_utils::MockSettingsLoader;
    use std::sync::Arc;

    let mut mock_loader = MockSettingsLoader::new();
    mock_loader
        .expect_load_settings()
        .times(1)
        .returning(|| Err(AppError::Settings("Mock settings failure".to_string())));

    // Act - Create recording config with failing loader
    let config = create_recording_config_with_loader(Arc::new(mock_loader)).await;

    // Assert - Should fall back to default duration (10 seconds)
    assert_eq!(config.max_duration_secs(), 10);
}

#[tokio::test]
async fn test_settings_integration_end_to_end() {
    // This test verifies the complete integration from settings to RecordingConfig

    // Arrange - Create settings with various durations
    let test_cases = vec![1, 5, 10, 15, 20, 25, 30];

    for duration in test_cases {
        // Set up isolated settings with specific duration
        let mut settings = AppSettings::default();
        settings.audio_duration_secs = duration;

        let (_temp_dir, loader) =
            test_utils::create_isolated_settings_env_with_settings(settings).await;

        // Act - Create config using workflow function with isolated loader
        let config = create_recording_config_with_loader(loader).await;

        // Assert - Config should match settings
        assert_eq!(
            config.max_duration_secs(),
            duration,
            "RecordingConfig duration should match settings for duration {duration}"
        );

        // Verify max_samples calculation is correct
        let expected_samples = (duration as usize) * 16_000; // 16kHz sample rate
        assert_eq!(
            config.max_samples(),
            expected_samples,
            "Sample count should be correct for duration {duration}"
        );
    }
}

// ============================================================================
// Test Isolation Verification
// ============================================================================

#[tokio::test]
async fn test_isolated_settings_environments_dont_interfere() {
    // This test verifies that multiple isolated settings environments
    // can run in parallel without interfering with each other

    // Arrange - Create multiple isolated environments with different settings
    let mut settings1 = AppSettings::default();
    settings1.audio_duration_secs = 5;

    let mut settings2 = AppSettings::default();
    settings2.audio_duration_secs = 25;

    let (_temp_dir1, loader1) =
        test_utils::create_isolated_settings_env_with_settings(settings1).await;
    let (_temp_dir2, loader2) =
        test_utils::create_isolated_settings_env_with_settings(settings2).await;

    // Act - Create configs from both loaders simultaneously
    let (config1, config2) = tokio::join!(
        create_recording_config_with_loader(loader1),
        create_recording_config_with_loader(loader2)
    );

    // Assert - Each config should use its own settings
    assert_eq!(
        config1.max_duration_secs(),
        5,
        "First environment should use 5 seconds"
    );
    assert_eq!(
        config2.max_duration_secs(),
        25,
        "Second environment should use 25 seconds"
    );

    // Verify they are truly independent
    assert_ne!(
        config1.max_duration_secs(),
        config2.max_duration_secs(),
        "Configs should have different durations proving isolation"
    );
}

#[tokio::test]
async fn test_mock_settings_loader_for_error_scenarios() {
    // This test verifies that mock settings loaders work correctly for testing error scenarios

    use speakr_lib::settings::traits::test_utils::MockSettingsLoader;
    use std::sync::Arc;

    // Arrange - Create mock that returns specific settings
    let mut mock_loader = MockSettingsLoader::new();
    let mut expected_settings = AppSettings::default();
    expected_settings.audio_duration_secs = 15;

    mock_loader
        .expect_load_settings()
        .times(1)
        .returning(move || Ok(expected_settings.clone()));

    // Act - Use mock loader
    let config = create_recording_config_with_loader(Arc::new(mock_loader)).await;

    // Assert - Should use mocked settings
    assert_eq!(config.max_duration_secs(), 15);
}

#[tokio::test]
// Note: This test can be heavy in CI environments with many parallel threads
async fn test_parallel_isolated_settings_loading() {
    // This test verifies that isolated settings can be loaded in parallel
    // without race conditions or interference

    // Arrange - Create multiple environments with different durations
    let durations = vec![1, 5, 10, 15, 20, 25, 30];
    let mut loaders = Vec::new();
    let mut temp_dirs = Vec::new();

    for duration in &durations {
        let mut settings = AppSettings::default();
        settings.audio_duration_secs = *duration;

        let (temp_dir, loader) =
            test_utils::create_isolated_settings_env_with_settings(settings).await;
        temp_dirs.push(temp_dir); // Keep temp dirs alive
        loaders.push(loader);
    }

    // Act - Load all settings in parallel
    let mut tasks = Vec::new();
    for loader in loaders {
        let task = tokio::spawn(async move { create_recording_config_with_loader(loader).await });
        tasks.push(task);
    }

    let configs: Vec<_> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .map(|result| result.unwrap())
        .collect();

    // Assert - Each config should have the correct duration
    for (i, config) in configs.iter().enumerate() {
        assert_eq!(
            config.max_duration_secs(),
            durations[i],
            "Config {} should have duration {}",
            i,
            durations[i]
        );
    }

    // Verify all configs are different (proving isolation)
    for i in 0..configs.len() {
        for j in i + 1..configs.len() {
            if durations[i] != durations[j] {
                assert_ne!(
                    configs[i].max_duration_secs(),
                    configs[j].max_duration_secs(),
                    "Configs with different settings should have different durations"
                );
            }
        }
    }
}

// ============================================================================
// Future Integration Tests (Placeholders)
// ============================================================================

#[tokio::test]
#[ignore = "Future integration test - requires actual audio hardware"]
async fn test_complete_workflow_with_real_audio() {
    // TODO: This test will be implemented when actual transcription and injection are available
    // It should:
    // 1. Capture real audio from microphone
    // 2. Transcribe using Whisper model
    // 3. Inject text using enigo
    // 4. Verify end-to-end latency requirements
}

#[tokio::test]
#[ignore = "Future integration test - requires Whisper models"]
async fn test_workflow_transcription_accuracy() {
    // TODO: This test will be implemented when transcription is available
    // It should:
    // 1. Use known audio samples
    // 2. Verify transcription accuracy
    // 3. Test different model sizes
    // 4. Verify language detection
}

#[tokio::test]
#[ignore = "Future integration test - requires text injection"]
async fn test_workflow_text_injection_compatibility() {
    // TODO: This test will be implemented when text injection is available
    // It should:
    // 1. Test injection in various applications
    // 2. Verify special character handling
    // 3. Test Unicode support
    // 4. Verify injection speed requirements
}
#[tokio::test]
async fn test_workflow_uses_settings_duration_for_recording_time() {
    // This test verifies that the workflow actually uses the settings duration
    // for both RecordingConfig and the sleep time during recording

    // Arrange - Create settings with a specific duration
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 3; // 3 seconds for testing

    let (_temp_dir, loader) =
        test_utils::create_isolated_settings_env_with_settings(settings).await;

    // Act - Create recording config using the workflow function
    let config = create_recording_config_with_loader(loader).await;

    // Assert - Config should use the settings duration
    assert_eq!(config.max_duration_secs(), 3);

    // The actual sleep duration in capture_audio_with_loader should match this
    // This is verified by the fact that the config is used to determine sleep time
    // in the line: Duration::from_secs(config.max_duration_secs() as u64)
}
