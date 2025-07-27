# Requirements Document

## Introduction

The Global Hotkey feature enables users to trigger the complete dictation workflow (record →
transcribe → inject) from anywhere in the system using a single keyboard shortcut. This system-wide
hotkey registration allows users to capture speech and inject transcribed text without switching
applications or losing focus, maintaining their workflow and productivity.

## Requirements

### Requirement 1

**User Story:** As a user, I want to register a global hotkey that works system-wide, so that I can
trigger dictation from any application without switching focus.

#### Acceptance Criteria

1. WHEN the application starts THEN the system SHALL register a global hotkey (default ⌥ Option +
   `~`)
2. WHEN the hotkey is registered THEN it SHALL remain active even when Speakr is running in the
   background
3. WHEN there is a hotkey conflict THEN the system SHALL warn the user and provide alternative
   suggestions
4. WHEN the application terminates THEN the system SHALL unregister the global hotkey

### Requirement 2

**User Story:** As a user, I want the hotkey to trigger the complete dictation workflow, so that I
can capture speech and have it transcribed into text with a single keypress.

#### Acceptance Criteria

1. WHEN the global hotkey is pressed THEN the system SHALL initiate audio recording
2. WHEN audio recording completes THEN the system SHALL automatically start transcription
3. WHEN transcription completes THEN the system SHALL inject the transcribed text into the currently
   focused field
4. WHEN any step in the workflow fails THEN the system SHALL provide user feedback and gracefully
   abort

### Requirement 3

**User Story:** As a user, I want to configure the hotkey in Settings, so that I can customize it to
fit my workflow and avoid conflicts with other applications.

#### Acceptance Criteria

1. WHEN I open Settings THEN I SHALL see the current hotkey configuration
2. WHEN I change the hotkey in Settings THEN the system SHALL update the registration immediately
3. WHEN I set a conflicting hotkey THEN the system SHALL warn me and prevent duplicate registrations
4. WHEN I save hotkey changes THEN the new hotkey SHALL be persisted across application restarts

### Requirement 4

**User Story:** As a user, I want reliable hotkey activation with fast response times, so that
dictation feels responsive and doesn't interrupt my workflow.

#### Acceptance Criteria

1. WHEN I press the hotkey THEN the system SHALL respond within 100ms
2. WHEN I use the hotkey repeatedly THEN it SHALL have a 99% activation success rate
3. WHEN I complete a 5-second recording THEN the total time-to-text SHALL be ≤3 seconds on M-series
   Macs
4. WHEN the hotkey is pressed during an active recording THEN the system SHALL stop the current
   recording and start a new one
