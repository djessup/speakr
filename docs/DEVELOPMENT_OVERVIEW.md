# Development Overview

## Testing Strategy

Speakr follows **Test-Driven Development (TDD)** principles with comprehensive test coverage across
all components.

### Test Organisation

- **Unit Tests**: Located in `#[cfg(test)]` modules within each source file
- **Integration Tests**: Located in `tests/` directories within each crate
- **Workflow Tests**: End-to-end testing of the complete dictation pipeline

### Key Testing Patterns

#### Settings Integration Testing

The workflow system includes comprehensive tests for settings integration, particularly for audio
capture configuration:

```rust
#[tokio::test]
async fn test_audio_capture_uses_settings_duration() {
    // Test that audio capture respects user-configured duration
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let mut settings = AppSettings::default();
    settings.audio_duration_secs = 15; // Custom duration

    save_settings_to_dir(&settings, &temp_dir.path().to_path_buf()).await.unwrap();
    let loaded_settings = load_settings_from_dir(&temp_dir.path().to_path_buf()).await.unwrap();

    assert_eq!(loaded_settings.audio_duration_secs, 15);
}
```

#### Workflow Integration Testing

The `workflow_tests.rs` module validates the complete dictation pipeline integration:

- **Settings Loading**: Verifies that workflow components load user settings correctly
- **Audio Duration Configuration**: Tests that recording configuration uses settings-based duration
- **Error Handling**: Validates graceful error handling across workflow steps
- **End-to-End Integration**: Tests complete record → transcribe → inject pipeline

### Test Isolation

All tests use isolated environments to prevent interference:

- **Filesystem Tests**: Use `tempfile::TempDir` for isolated temporary directories
- **Settings Tests**: Create isolated settings files that don't affect global configuration
- **Audio Tests**: Mock audio devices and use test data where possible
- **Test Utilities**: The `test_utils.rs` module provides helper functions for creating isolated
  settings environments and managing test dependencies

### Running Tests

```bash
# Run all tests
cargo test --workspace --all-features

# Run specific test module
cargo test --package speakr-tauri workflow_tests

# Run with output
cargo test --package speakr-tauri workflow_tests -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --all-features --out Html
```

## Current Implementation Status

### Completed Components

- ✅ **Settings System**: Persistent configuration with validation and migration support
- ✅ **Global Hotkey**: System-wide hotkey registration with conflict detection and settings
  integration
- ✅ **Audio Capture**: 16kHz mono recording with configurable duration (1-30 seconds, validated
  using named constants)
- ✅ **Workflow Orchestration**: Complete pipeline integration with comprehensive error handling
- ✅ **Testing Infrastructure**: TDD practices with >90% coverage and isolated test environments

### In Progress

- 🚧 **Transcription**: Whisper model integration (placeholder implementation)
- 🚧 **Text Injection**: Synthetic keystroke generation (placeholder implementation)
- 🚧 **UI Components**: Settings interface and status displays

### Planned

- 📋 **Model Management**: Whisper model download and validation
- 📋 **Performance Monitoring**: Latency tracking and optimization
- 📋 **Auto-update**: GitHub releases integration

## Development Workflow

### 1. Test-First Development

Before implementing any feature:

1. **Write failing tests** that describe the expected behaviour
2. **Run tests** to confirm they fail for the right reasons
3. **Implement minimal code** to make tests pass
4. **Refactor** while keeping tests green

### 2. Settings Integration Pattern

When adding new configurable features:

1. **Add field to `AppSettings`** in `speakr-types/src/lib.rs`
2. **Add validation logic** if needed
3. **Update settings persistence** in `speakr-tauri/src/settings/`
4. **Write integration tests** that verify settings loading and usage
5. **Update UI components** to expose the setting

### 3. Workflow Integration Pattern

When adding new workflow steps:

1. **Define error types** in `speakr-types/src/lib.rs`
2. **Implement core logic** in `speakr-core/`
3. **Add workflow orchestration** in `speakr-tauri/src/workflow.rs`
4. **Write integration tests** in `speakr-tauri/tests/workflow_tests.rs`
5. **Add UI feedback** for status and errors

## Code Quality Standards

### Error Handling

- Use `Result<T, AppError>` for all fallible operations
- Provide meaningful error messages with context
- Test both success and failure cases
- Handle errors gracefully without crashing

### Documentation

- Document all public APIs with rustdoc
- Include examples in documentation
- Keep documentation up-to-date with implementation
- Use clear, concise language

### Testing Requirements

- **>90% code coverage** for all new code
- **Test isolation** - no shared state between tests
- **Meaningful test names** that describe behaviour
- **Both positive and negative test cases**
- **TDD compliance** - write tests before implementation
- **Dependency injection** - use traits and mocking for testable code
- **Both positive and negative test cases**
