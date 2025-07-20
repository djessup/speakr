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

- [ ] Recording initialises within 100 ms after hot-key press.
- [ ] Audio stream conforms to 16 kHz, 16-bit, mono.
- [ ] User can change max duration in Settings; value persists across restarts.
- [ ] Recording stops cleanly at limit without crashing or clipping.
- [ ] Permission dialog appears once and records decision.

## Test-Driven Design

Adopt test-driven development: begin by writing failing unit/integration tests that assert each
Acceptance Criterion. Only then implement capture logic until tests pass in CI.

## References

PRD §6 Functional Requirements – FR-2
