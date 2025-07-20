# FR-6: Settings UI

Provides a graphical interface (tray or window) for user configuration.

## Requirement

1. Expose configuration for:
   - Global hot-key picker
   - Model selector (small, medium, large GGUF)
   - Auto-launch on login toggle
2. Implemented as a Tauri window accessible from the menu bar/tray.
3. Validate hot-key conflicts and model availability.
4. Preference changes take effect without restarting the app.

## Rationale

A minimal settings UI keeps the main workflow keyboard-first while allowing deeper configuration
when needed.

## Acceptance Criteria

- [ ] Opening Settings from tray displays window within 200 ms.
- [ ] Changing options updates behaviour immediately (e.g. new hot-key active).
- [ ] Invalid configurations (missing model file) display inline errors.
- [ ] Settings persist after app restart.

## Test-Driven Design

Define unit/UI tests for each settings control and validation rule before coding. Implementation is
complete when all tests pass.

## References

PRD §6 Functional Requirements – FR-6
