# Requirements Document

## Introduction

The Text Injection feature provides synthetic keyboard input to type transcribed text into the
currently focused application. Using the `enigo` crate, the system simulates natural typing to
inject text directly into any input field, maintaining the illusion of native user input while
preserving formatting and special characters. The implementation must work across all macOS
applications while respecting accessibility APIs and performance requirements.

## Requirements

### Requirement 1

**User Story:** As a user, I want transcribed text to be typed directly into my current application,
so that I can seamlessly continue working without manual copy-paste operations.

#### Acceptance Criteria

1. WHEN transcription completes THEN the text SHALL be injected into the currently focused input
   field
2. WHEN text is injected THEN it SHALL use synthetic keystrokes via the `enigo` crate
3. WHEN injection occurs THEN it SHALL work in any macOS application with text input
4. WHEN injection completes THEN the cursor SHALL be positioned at the end of the injected text

### Requirement 2

**User Story:** As a user, I want injected text to preserve all formatting and special characters,
so that the transcribed content appears exactly as intended.

#### Acceptance Criteria

1. WHEN text contains line breaks THEN they SHALL be preserved in the injected output
2. WHEN text contains punctuation THEN it SHALL be reproduced byte-for-byte accurately
3. WHEN text contains special characters THEN they SHALL be properly encoded and injected
4. WHEN text contains Unicode characters THEN they SHALL be handled correctly across applications

### Requirement 3

**User Story:** As a user, I want fast text injection that doesn't interrupt my workflow, so that
dictation feels responsive and natural.

#### Acceptance Criteria

1. WHEN injecting 100 characters THEN the operation SHALL complete within 300 milliseconds
2. WHEN injection starts THEN it SHALL begin immediately after transcription completes
3. WHEN injection is in progress THEN it SHALL not block other application functions
4. WHEN injection completes THEN feedback SHALL be provided to indicate completion

### Requirement 4

**User Story:** As a user, I want text injection to work reliably across different applications, so
that I can use dictation in any context.

#### Acceptance Criteria

1. WHEN using VS Code THEN text injection SHALL work in all editor contexts
2. WHEN using Xcode THEN text injection SHALL work in code editors and text fields
3. WHEN using Pages THEN text injection SHALL work in document editing
4. WHEN using Safari THEN text injection SHALL work in web forms and text areas

### Requirement 5

**User Story:** As a developer, I want proper error handling and feedback for text injection, so
that I can diagnose and resolve issues when injection fails.

#### Acceptance Criteria

1. WHEN injection fails THEN the error SHALL be logged with sufficient detail for debugging
2. WHEN injection completes THEN a completion event SHALL be emitted for UI feedback
3. WHEN accessibility permissions are missing THEN clear guidance SHALL be provided
4. WHEN injection is blocked THEN fallback options SHALL be suggested to the user

### Requirement 6

**User Story:** As a macOS user, I want text injection to respect system accessibility APIs, so that
it works properly with system security and accessibility features.

#### Acceptance Criteria

1. WHEN injection runs THEN it SHALL execute on the main UI thread as required by macOS
2. WHEN accessibility permissions are required THEN the system SHALL request them appropriately
3. WHEN system security blocks injection THEN clear error messages SHALL be provided
4. WHEN injection occurs THEN it SHALL respect macOS input method and keyboard layout settings
