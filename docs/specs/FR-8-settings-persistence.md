# FR-8: Settings Persistence

Persist user preferences locally without cloud sync.

## Requirement

1. Store settings in a JSON file located in the platform-appropriate app data directory
2. (`$HOME/Library/Application Support/Speakr/settings.json`).
3. Write changes atomically to avoid corruption.
4. Migration framework supports future schema evolution with versioning.
5. No data leaves the device.

## Rationale

Local persistence offers instant access, privacy, and offline capability.

## Acceptance Criteria

- [ ] Settings file created on first launch with defaults.
- [ ] Modifying settings updates file within 100 ms.
- [ ] Corrupt settings file triggers automatic recovery to defaults.
- [ ] Unit tests cover load/save error paths.

## Test-Driven Design

Write failing unit tests for load/save, corruption recovery, and migration before implementation;
pass them in CI.

## References

PRD §6 Functional Requirements – FR-8
