# Implementation Plan

## Current Implementation Status

The global hotkey system has been largely implemented with the following key components:

### Complete

- ✅ **Core Service**: `GlobalHotkeyService` with registration/unregistration
- ✅ **Event Emission**: Hotkey triggers emit `hotkey-triggered` events
- ✅ **Tauri Commands**: `register_global_hotkey` and `unregister_global_hotkey`
- ✅ **Startup Integration**: Default hotkey registration with fallback
- ✅ **Validation**: Basic hotkey format validation
- ✅ **Testing**: Comprehensive validation tests

### Incomplete

- ❌ **Event Handling**: No listener for `hotkey-triggered` events
- ❌ **Settings Integration**: Hardcoded defaults, no user settings loading
- ❌ **Runtime Updates**: No UI integration for changing hotkeys

## Remaining Tasks

- [x] 1. Complete core hotkey workflow integration

  - Wire the `hotkey-triggered` event to the complete dictation pipeline (record → transcribe →
    inject)
  - Implement event handler that calls into `speakr-core` for audio capture
  - Add proper error handling and user feedback for workflow failures
  - _Requirements: 2.1, 2.2_

- [x] 2. Implement settings integration for hotkey configuration

  - [x] 2.1 Load user-defined hotkey from persisted settings at startup

    - ✅ Modified `register_default_hotkey` function in `lib.rs` to load hotkey from
      `AppSettings.hot_key` using `load_settings_internal()`
    - ✅ Updated startup sequence to attempt settings loading before hotkey registration
    - ✅ Maintained existing fallback behaviour when settings load fails (uses
      "CmdOrCtrl+Alt+Space")
    - ✅ Added proper logging for both successful settings load and fallback scenarios
    - _Requirements: 3.1, 3.4_

  - [x] 2.2 Create Tauri command for runtime hotkey updates
    - Implement `update_global_hotkey` Tauri command that re-registers hotkey at runtime
    - Integrate with existing `GlobalHotkeyService` for registration/unregistration
    - Replace UI `GlobalShortcutManager` JavaScript calls with backend service calls
    - _Requirements: 3.2, 3.3_

- [x] 3. Basic hotkey service implementation

  - [x] 3.1 Core GlobalHotkeyService structure

    - `GlobalHotkeyService` struct implemented with registration/unregistration methods
    - Thread-safe state management using Arc<Mutex<>> for current shortcut tracking
    - Integration with `tauri-plugin-global-shortcut` for system-level registration
    - _Requirements: 1.1, 1.2_

  - [x] 3.2 Event emission on hotkey trigger

    - Hotkey triggers emit `hotkey-triggered` event with empty payload
    - Event emission happens in shortcut callback handler
    - TODO comment indicates workflow integration needed in next step
    - _Requirements: 2.1_

  - [x] 3.3 Basic conflict detection and error handling

    - `HotkeyError` enum with `RegistrationFailed`, `ConflictDetected`, `NotFound` variants
    - Registration failures return appropriate error types with descriptive messages
    - Automatic unregistration of existing shortcuts before registering new ones
    - _Requirements: 1.3, 3.3_

  - [x] 3.4 Tauri command integration
    - `register_global_hotkey` and `unregister_global_hotkey` commands implemented
    - Commands accept `HotkeyConfig` struct with shortcut string and enabled flag
    - Internal functions separate business logic from Tauri command wrappers
    - _Requirements: 3.2_

- [x] 4. Application startup integration

  - [x] 4.1 Default hotkey registration at startup

    - Application registers `CmdOrCtrl+Alt+Space` as default hotkey on startup
    - Registration happens asynchronously in setup function
    - Success/failure logged with appropriate debug information
    - _Requirements: 1.1, 1.2_

  - [x] 4.2 Fallback hotkey mechanism
    - Falls back to `CmdOrCtrl+Alt+F2` if default hotkey registration fails
    - Provides user feedback through logging when fallback is used
    - Graceful degradation when both default and fallback fail
    - _Requirements: 1.3_

- [x] 5. Basic validation and type system

  - [x] 5.1 HotkeyConfig data structure

    - `HotkeyConfig` struct with shortcut string and enabled boolean
    - Default implementation using `DEFAULT_HOTKEY` constant
    - Serialization support for settings persistence
    - _Requirements: 3.1, 3.4_

  - [x] 5.2 Hotkey validation using Tauri parsing
    - Basic validation implemented in `validate_hot_key_internal` function
    - Checks for required modifiers and proper format structure
    - Additional validation using Tauri's native parsing in hotkey service
    - _Requirements: 3.2, 3.3_

- [x] 6. Constants and defaults updated

  - [x] 6.1 Default hotkey constant
    - `DEFAULT_HOTKEY` set to `CmdOrCtrl+Alt+F1` in `speakr-types`
    - Avoids conflicts with common system shortcuts
    - Used consistently across application startup and settings
    - _Requirements: 1.1_

- [x] 7. Complete workflow integration

  - [x] 7.1 Event listener for hotkey-triggered events
    - Add event listener in application setup to handle `hotkey-triggered` events
    - Connect event handler to `speakr-core` audio capture functionality
    - Implement complete record → transcribe → inject pipeline
    - _Requirements: 2.1, 2.2_

- [ ] 8. Performance monitoring and optimization

  - [ ] 8.1 Add response time measurement

    - Implement timing measurement from hotkey press to event emission
    - Add performance logging to track response times
    - Ensure 100ms response time requirement is met
    - _Requirements: 4.1_

  - [ ] 8.2 Add end-to-end latency tracking
    - Implement sequence ID generation for tracking complete workflows
    - Add timing measurement for complete record → transcribe → inject pipeline
    - Create performance metrics collection for 3-second latency requirement
    - _Requirements: 4.3_

- [ ] 9. Enhanced error handling and recovery

  - [ ] 9.1 Add graceful workflow abortion on failures

    - Implement proper cleanup when any step in the dictation workflow fails
    - Add user notification system for workflow failures
    - Ensure system remains responsive after workflow errors
    - _Requirements: 2.4_

  - [ ] 9.2 Add hotkey registration recovery
    - Implement automatic re-registration on system wake/resume
    - Add periodic health checks for hotkey registration status
    - Create recovery mechanisms for lost hotkey registrations
    - _Requirements: 1.1, 1.4_

- [ ] 10. Comprehensive testing coverage

  - [ ] 10.1 Create integration tests for hotkey workflow

    - Write tests for complete hotkey → dictation → injection workflow
    - Add tests for error scenarios and recovery mechanisms
    - Create performance tests for response time requirements
    - _Requirements: 4.1, 4.2, 4.3_

  - [x] 10.2 Basic hotkey validation tests
    - Comprehensive test coverage for hotkey format validation in `hotkey_tests.rs`
    - Tests for function keys, special keys, numeric keys, and symbols
    - Tests for invalid hotkey formats and edge cases
    - Tests implemented in `commands/validation.rs` module tests
    - _Requirements: 3.2, 3.3_

- [x] 11. UI enhancements for hotkey management

  - [x] 11.1 Create hotkey configuration interface

    - Add hotkey input field with real-time validation in Settings UI
    - Implement hotkey conflict warnings in the UI
    - Add hotkey testing functionality (press to test)
    - _Requirements: 3.1, 3.2, 3.3_

  - [x] 11.2 Add hotkey status indicators
    - Display current hotkey registration status in UI
    - Show conflict warnings and resolution suggestions
    - Add visual feedback for successful hotkey changes
    - _Requirements: 3.3, 3.4_
