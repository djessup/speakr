# Requirements Document

## Introduction

The Audio Capture feature provides high-quality microphone input recording optimized for speech
transcription using the `cpal` crate. The system captures audio in Whisper-compatible format (16 kHz
mono) with configurable duration limits and in-memory processing for privacy. The implementation
uses a trait-based architecture enabling dependency injection and comprehensive testing.

## Requirements

### Requirement 1

**User Story:** As a user, I want the system to capture high-quality audio from my microphone, so
that speech transcription will be accurate and reliable.

#### Acceptance Criteria

1. WHEN audio capture is initiated THEN the system SHALL capture audio at 16 kHz sample rate
2. WHEN audio is captured THEN it SHALL be recorded in mono (single channel) format
3. WHEN audio is captured THEN it SHALL use 16-bit sample depth for quality
4. WHEN different audio formats are encountered THEN the system SHALL convert them to 16-bit signed
   integers

### Requirement 2

**User Story:** As a user, I want to control how long audio recordings can be, so that I can capture
the right amount of speech for my needs while avoiding excessive memory usage.

#### Acceptance Criteria

1. WHEN no custom duration is set THEN the system SHALL use a configurable default duration
2. WHEN I configure a custom duration THEN the system SHALL allow between 1-30 seconds
3. WHEN the duration limit is reached THEN the system SHALL stop recording automatically
4. WHEN I change the duration setting THEN it SHALL persist across application restarts

### Requirement 3

**User Story:** As a user, I want responsive audio capture that starts quickly, so that I don't lose
the beginning of my speech while waiting for recording to begin.

#### Acceptance Criteria

1. WHEN the hotkey is pressed THEN audio recording SHALL initialize within 100 milliseconds
2. WHEN recording starts THEN the system SHALL immediately begin capturing audio samples
3. WHEN recording initialization fails THEN the system SHALL provide clear error feedback
4. WHEN the microphone is unavailable THEN the system SHALL handle the error gracefully

### Requirement 4

**User Story:** As a user, I want to control recording manually, so that I can capture exactly the
speech I intend without unwanted audio.

#### Acceptance Criteria

1. WHEN I press the hotkey during recording THEN the system SHALL stop the current recording
2. WHEN recording stops THEN the system SHALL cleanly finalize the audio buffer without clipping
3. WHEN recording stops THEN the captured samples SHALL be immediately available as `Vec<i16>`
4. WHEN multiple recordings are attempted THEN the system SHALL prevent concurrent recordings

### Requirement 5

**User Story:** As a privacy-conscious user, I want audio to be processed entirely in memory, so
that no sensitive speech data is written to disk unnecessarily.

#### Acceptance Criteria

1. WHEN audio is captured THEN it SHALL be buffered entirely in memory using `Arc<Mutex<Vec<i16>>>`
2. WHEN recording completes THEN no temporary audio files SHALL be written to disk
3. WHEN the application terminates THEN all audio buffers SHALL be cleared from memory
4. WHEN transcription completes THEN the audio buffer SHALL be released immediately

### Requirement 6

**User Story:** As a developer, I want the audio system to be testable and maintainable, so that I
can ensure reliability and add new features safely.

#### Acceptance Criteria

1. WHEN testing audio functionality THEN the system SHALL support mock implementations via traits
2. WHEN running tests THEN they SHALL not require actual audio hardware
3. WHEN errors occur THEN the system SHALL provide detailed error information via AudioCaptureError
4. WHEN listing audio devices THEN the system SHALL return available input devices
