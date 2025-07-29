# Implementation Plan

- [x] 1. Complete settings integration for audio duration configuration

  - [x] 1.1 Add audio duration field to AppSettings structure

    - ✅ Added `audio_duration_secs: u32` field to `AppSettings` in `speakr-types/src/lib.rs`
    - ✅ Set default value to 10 seconds matching current behaviour
    - ✅ Added validation to ensure duration is between 1-30 seconds via `validate_audio_duration()`
    - ✅ Added `MIN_AUDIO_DURATION_SECS` and `MAX_AUDIO_DURATION_SECS` constants to eliminate magic
      numbers
    - _Requirements: 2.2, 2.4_

  - [x] 1.2 Integrate audio duration with RecordingConfig

    - ✅ Created `create_recording_config_from_settings()` function in workflow module
    - ✅ Modified workflow to load duration from settings instead of hardcoded values
    - ✅ Updated `AudioRecorder::new()` calls to use settings-based duration
    - ✅ Ensured settings changes are reflected in new recordings
    - ✅ Added comprehensive integration tests validating settings workflow
    - _Requirements: 2.1, 2.4_

  - [x] 1.3 Implement proper test isolation for settings-dependent tests
    - ✅ Analyzed existing test patterns and found consistent use of `TempDir` for isolation
    - ✅ Created `SettingsLoader` trait with `GlobalSettingsLoader` and `IsolatedSettingsLoader`
      implementations
    - ✅ Modified workflow functions to accept settings loader via dependency injection
    - ✅ Added `MockSettingsLoader` using mockall for error scenario testing
    - ✅ Created comprehensive test utilities in `test_utils.rs` for isolated settings environments
    - ✅ Updated all workflow tests to use isolated temporary directories
    - ✅ Verified tests can run in parallel without interfering with each other
    - ✅ Added comprehensive test coverage for isolation scenarios and parallel execution
    - _Requirements: 2.4_

- [x] 2. Implement macOS permission handling

  - [x] 2.1 Add permission request flow for first-time microphone access

    - Create user-friendly permission request dialog or guidance
    - Handle macOS permission prompts gracefully
    - Provide clear instructions when permissions are denied
    - _Requirements: 1.4, 3.4_

  - [x] 2.2 Add permission status checking
    - Implement function to check current microphone permission status
    - Add Tauri command to query permission status from UI
    - Display permission status in settings or debug panel
    - _Requirements: 1.4, 3.4, 5.1_

- [ ] 2. Implement graceful permission handling

  - [ ] 2.1 Add permission request flow for first-time microphone access

    - Create user-friendly permission request dialog or guidance
    - Handle macOS permission prompts gracefully
    - Provide clear instructions when permissions are denied
    - _Requirements: 1.4, 3.4_

  - [ ] 2.2 Add permission status checking
    - Implement function to check current microphone permission status
    - Add Tauri command to query permission status from UI
    - Display permission status in settings or debug panel
    - _Requirements: 1.4_

- [x] 3. Complete hotkey integration for production use

  - [x] 3.1 Wire hotkey events to audio capture

    - Connect `hotkey-triggered` event to `AudioRecorder::start_recording()`
    - Implement proper error handling when audio capture fails during hotkey press
    - Add user feedback for audio capture errors (device unavailable, permissions, etc.)
    - _Requirements: 3.1, 3.3, 4.1_

  - [ ] 3.2 Implement hotkey-based recording control
    - Handle second hotkey press to stop active recording
    - Add visual/audio feedback for recording start/stop states
    - Ensure clean recording termination on hotkey stop
    - _Requirements: 4.1, 4.2_

- [ ] 4. Add settings UI for audio configuration

  - [ ] 4.1 Create audio settings section in Settings UI

    - Add duration slider/input field (1-30 seconds range)
    - Display current audio device information
    - Add test recording functionality to verify audio setup
    - _Requirements: 2.2, 2.4_

  - [ ] 4.2 Add audio device selection interface
    - Display list of available input devices using `list_input_devices()`
    - Allow user to select preferred input device
    - Show device status (available, default, etc.)
    - _Requirements: 6.4_

- [ ] 5. Enhance error handling and user feedback

  - [ ] 5.1 Improve error messages for common scenarios

    - Add user-friendly error messages for `AudioCaptureError` variants
    - Create error recovery suggestions (check permissions, reconnect device, etc.)
    - Add error logging with appropriate detail levels
    - _Requirements: 3.3, 3.4_

  - [ ] 5.2 Add audio system health monitoring
    - Implement periodic checks for audio device availability
    - Add recovery mechanisms for lost audio devices
    - Create system status indicators for audio subsystem health
    - _Requirements: 6.3_

- [x] 6. Optimize performance and memory usage

  - [x] 6.1 Verify initialization performance requirements

    - Add performance monitoring to ensure <100ms initialization time
    - Profile memory usage during recording to stay within limits
    - Optimize sample buffer management for large recordings
    - _Requirements: 3.1_

  - [x] 6.2 Add memory cleanup verification
    - Ensure audio buffers are properly released after transcription
    - Add memory usage monitoring and alerts for excessive usage
    - Implement automatic cleanup on application shutdown
    - _Requirements: 5.3, 5.4_

- [x] 7. Expand test coverage for integration scenarios

  - [x] 7.1 Add integration tests for settings persistence

    - ✅ Added comprehensive workflow integration tests in `workflow_tests.rs`
    - ✅ Test that audio duration changes persist across app restarts
    - ✅ Verify settings validation prevents invalid duration values (0, 31+ seconds)
    - ✅ Test settings migration when audio settings are added
    - ✅ Added end-to-end testing from settings to RecordingConfig
    - _Requirements: 2.4_

  - [ ] 7.2 Add integration tests for hotkey workflow
    - Test complete hotkey → audio capture → transcription pipeline
    - Verify error handling when audio capture fails during hotkey press
    - Test concurrent hotkey presses and recording state management
    - _Requirements: 4.1, 4.4_

- [x] 8. Add audio quality and format validation

  - [x] 8.1 Verify Whisper format compatibility

    - Add tests to confirm 16 kHz mono output matches Whisper requirements
    - Validate sample format conversion accuracy (F32, I16, U16 → I16)
    - Test audio quality with various input devices and formats
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [ ] 8.2 Add audio quality monitoring
    - Implement basic audio level detection to warn about silent recordings
    - Add sample rate verification to ensure proper conversion
    - Create audio quality metrics for debugging and optimization
    - _Requirements: 1.1, 1.2_

- [ ] 9. Implement advanced recording features

  - [ ] 9.1 Add recording state indicators

    - Create visual indicators for recording active/inactive states
    - Add audio feedback (beep) for recording start/stop (optional)
    - Implement recording timer display in UI
    - _Requirements: 4.2_

  - [ ] 9.2 Add recording history and debugging
    - Store recent recording metadata (duration, sample count, timestamp)
    - Add debug panel showing recording statistics and device info
    - Implement recording quality analysis for troubleshooting
    - _Requirements: 6.3, 6.4_
