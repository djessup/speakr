# FR-2: Audio Capture

Captures microphone input suitable for Whisper transcription.

## Requirement

1. Capture 16 kHz mono audio via the `cpal` crate.
2. Default maximum duration **10 s**; user-configurable up to 30 s.
3. Recording stops automatically when the duration limit is reached or the user presses the hot-key
4. again.
5. Audio is buffered entirely in memory; no files are written to disk.
6. Handle microphone permission prompts gracefully on first run.

## Rationale

Lower sample-rate mono audio minimises processing cost while meeting Whisper’s input requirements.

## Acceptance Criteria

- [x] Recording initialises within 100 ms after hot-key press.
- [x] Audio stream conforms to 16 kHz, 16-bit, mono.
- [x] User can change max duration in Settings; value persists across restarts.
- [x] Recording stops cleanly at limit without crashing or clipping.
- [ ] Permission dialog appears once and records decision.

## Implementation Status

### Completed Features

- **Settings Integration**: Audio duration (1-30 seconds) is loaded from
  `AppSettings.audio_duration_secs`
- **Workflow Integration**: Recording configuration uses settings-based duration via
  `create_recording_config_from_settings()`
- **Validation**: Settings validation ensures duration is within acceptable range (1-30 seconds)
- **Persistence**: Audio duration settings persist across application restarts
- **Testing**: Comprehensive integration tests validate settings loading and workflow integration

### Current Implementation

The audio capture system integrates with the settings system:

```rust
// Settings-based recording configuration
pub async fn create_recording_config_from_settings() -> RecordingConfig {
    let settings = load_settings_internal().await.unwrap_or_default();
    RecordingConfig::new(settings.audio_duration_secs) // 1-30 seconds
}

// Workflow integration
let config = create_recording_config_from_settings().await;
let recorder = AudioRecorder::new(config).await?;
```

### Test Coverage

Integration tests validate the complete settings workflow:

- **Settings Persistence**: Audio duration changes persist across restarts
- **Validation**: Invalid duration values are rejected (0, 31+ seconds)
- **Workflow Integration**: Recording configuration uses settings-based duration
- **Error Handling**: Graceful fallback to defaults when settings loading fails

## Test-Driven Design

Adopt test-driven development: begin by writing failing unit/integration tests that assert each
Acceptance Criterion. Only then implement capture logic until tests pass in CI.

## References

PRD §6 Functional Requirements – FR-2
