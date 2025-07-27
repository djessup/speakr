# Requirements Document

## Introduction

The Injection Fallback feature provides a clipboard-based alternative when direct text injection fails or is blocked by secure input fields. The system detects when synthetic keystrokes cannot be delivered and automatically falls back to clipboard paste operations while preserving user data and providing appropriate feedback. This ensures universal compatibility across all macOS applications, including those with enhanced security measures.

## Requirements

### Requirement 1

**User Story:** As a user, I want dictation to work in secure text fields, so that I can use voice input even when applications block synthetic keystrokes.

#### Acceptance Criteria

1. WHEN text injection fails due to security restrictions THEN the system SHALL detect the failure
2. WHEN a secure text field is detected THEN the system SHALL automatically switch to clipboard fallback
3. WHEN clipboard fallback is used THEN it SHALL achieve 100% success rate in secure fields
4. WHEN fallback occurs THEN the user SHALL be notified that clipboard was used

### Requirement 2

**User Story:** As a user, I want my existing clipboard contents preserved, so that fallback operations don't interfere with my workflow.

#### Acceptance Criteria

1. WHEN clipboard fallback is initiated THEN the current clipboard contents SHALL be saved
2. WHEN the paste operation completes THEN the original clipboard contents SHALL be restored
3. WHEN clipboard restoration occurs THEN it SHALL complete within 500 milliseconds
4. WHEN the process completes THEN no sensitive transcript data SHALL remain on the clipboard

### Requirement 3

**User Story:** As a user, I want clear feedback when fallback is used, so that I understand why the behavior is different from normal injection.

#### Acceptance Criteria

1. WHEN clipboard fallback is triggered THEN a warning overlay SHALL be displayed
2. WHEN the overlay appears THEN it SHALL show the message "Secure field detected – text pasted via clipboard"
3. WHEN the overlay is shown THEN it SHALL disappear automatically after 3 seconds
4. WHEN fallback occurs THEN the event SHALL be logged for debugging purposes

### Requirement 4

**User Story:** As a developer, I want reliable detection of injection failures, so that fallback is triggered appropriately without false positives.

#### Acceptance Criteria

1. WHEN enigo returns an error THEN the system SHALL trigger fallback mode
2. WHEN injection timeout occurs THEN the system SHALL attempt fallback
3. WHEN accessibility permissions are denied THEN fallback SHALL be used as alternative
4. WHEN fallback detection occurs THEN it SHALL not interfere with normal injection operations

### Requirement 5

**User Story:** As a user, I want fallback to work seamlessly with the clipboard paste operation, so that text appears correctly in the target application.

#### Acceptance Criteria

1. WHEN clipboard fallback is used THEN the system SHALL copy text to clipboard
2. WHEN text is copied THEN the system SHALL simulate ⌘V paste command
3. WHEN paste command is sent THEN it SHALL work in the currently focused application
4. WHEN paste completes THEN the text SHALL appear correctly formatted in the target field

### Requirement 6

**User Story:** As a security-conscious user, I want assurance that sensitive data is handled properly during fallback operations.

#### Acceptance Criteria

1. WHEN sensitive text is processed THEN it SHALL only remain on clipboard for the minimum necessary time
2. WHEN clipboard restoration occurs THEN all traces of the transcript SHALL be removed
3. WHEN fallback completes THEN no sensitive data SHALL be logged or persisted
4. WHEN errors occur during fallback THEN sensitive content SHALL still be properly cleaned up