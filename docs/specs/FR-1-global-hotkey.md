# FR-1: Global Hot-key

Registers a system-wide hot-key at application start that toggles the
**record → transcribe → inject** flow.

## Requirement

1. The application must register a global hot-key (default **⌘ Cmd + ⌥ Option + Space**,
   configurable via settings).
2. Must be active even when Speakr is running in the background.
3. The hot-key configuration is loaded from persisted settings at startup, with fallback to default
   if loading fails.
4. Pressing the hot-key initiates, in order:
   1. Audio recording
   2. Transcription
   3. Text injection into the current focused field.
5. The hot-key must be configurable in Settings and warn on conflicts.

## Rationale

A single keyboard shortcut lets users capture ideas without context-switching, maintaining focus and
flow.

## Acceptance Criteria

- [x] Hot-key can be triggered from any application on macOS 13+.
- [x] Hot-key configuration is loaded from persisted settings at startup.
- [x] Fallback to default hot-key when settings loading fails.
- [ ] 95th percentile **time-to-text ≤ 3 s** for 5 s recordings on M-series Macs.
- [ ] 99 % activation success rate in telemetry.
- [ ] Changing the hot-key in Settings updates the registration immediately and prevents duplicates.

## Test-Driven Design

Follow TDD: write failing automated tests for every case in **Test Cases** (formerly Acceptance
Criteria) before implementation. CI should pass only when the new tests turn green.

## References

PRD §6 Functional Requirements – FR-1
