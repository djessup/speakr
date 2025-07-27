# Requirements Document

## Introduction

The Transcription feature provides offline speech-to-text conversion using Whisper models running
entirely on-device. The system processes audio samples from the Audio Capture module and converts
them to text using locally-stored GGUF models, ensuring complete privacy and eliminating network
dependencies. The implementation must meet strict performance requirements while supporting multiple
model sizes and languages.

## Requirements

### Requirement 1

**User Story:** As a privacy-conscious user, I want all transcription to happen locally on my
device, so that my speech data never leaves my computer.

#### Acceptance Criteria

1. WHEN transcription is performed THEN it SHALL use only locally-stored Whisper models
2. WHEN transcription runs THEN no network requests SHALL be made during the process
3. WHEN models are needed THEN they SHALL be downloaded once and cached locally
4. WHEN the application runs THEN packet capture SHALL show no outbound traffic during transcription

### Requirement 2

**User Story:** As a user, I want fast transcription that doesn't interrupt my workflow, so that I
can continue working while speech is being processed.

#### Acceptance Criteria

1. WHEN transcribing a 5-second recording THEN it SHALL complete within 3 seconds on Apple Silicon
2. WHEN transcription starts THEN it SHALL initialize within 500 milliseconds
3. WHEN using the small model THEN 95th percentile latency SHALL be ≤3 seconds
4. WHEN transcription runs THEN it SHALL not block the UI or other application functions

### Requirement 3

**User Story:** As a user, I want to choose different model sizes, so that I can balance
transcription speed and accuracy based on my needs.

#### Acceptance Criteria

1. WHEN I access settings THEN I SHALL see available model size options (tiny, small, medium, large)
2. WHEN I select a different model THEN the transcription engine SHALL update without requiring
   restart
3. WHEN I choose a larger model THEN I SHALL get better accuracy at the cost of slower processing
4. WHEN I choose a smaller model THEN I SHALL get faster processing with acceptable accuracy

### Requirement 4

**User Story:** As a multilingual user, I want to transcribe speech in different languages, so that
I can use the application regardless of what language I'm speaking.

#### Acceptance Criteria

1. WHEN I access settings THEN I SHALL see language selection options
2. WHEN no language is specified THEN the system SHALL auto-detect the language
3. WHEN I select a specific language THEN transcription SHALL be optimized for that language
4. WHEN using English-only models THEN they SHALL provide better performance for English speech

### Requirement 5

**User Story:** As a user, I want clear error handling when transcription fails, so that I
understand what went wrong and how to fix it.

#### Acceptance Criteria

1. WHEN a model file is missing THEN the system SHALL display an actionable error message
2. WHEN transcription fails THEN the error SHALL be logged with sufficient detail for debugging
3. WHEN model loading fails THEN the system SHALL suggest downloading or repairing the model
4. WHEN insufficient memory is available THEN the system SHALL suggest using a smaller model

### Requirement 6

**User Story:** As a developer, I want the transcription system to integrate cleanly with the audio
capture and text injection pipeline, so that the complete workflow operates seamlessly.

#### Acceptance Criteria

1. WHEN audio capture completes THEN the audio samples SHALL be passed directly to transcription
2. WHEN transcription completes THEN the text result SHALL be passed to the text injection system
3. WHEN any step fails THEN the error SHALL be propagated with appropriate context
4. WHEN the pipeline runs THEN status updates SHALL be emitted for UI feedback
