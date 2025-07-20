# FR-7: Status Events

Emit real-time status updates for UI overlays and logging.

## Requirement

1. Broadcast status events: `Recording`, `Transcribing`, `Injected`, `Error` (variants).
2. Events emitted over an internal async channel consumable by UI components and log subsystem.
3. Include timestamp and optional payload (e.g. error message).
4. Provide public Rust API `subscribe_status()` for other components.

## Rationale

A decoupled event system lets the overlay and future extensions react without tight coupling to
business logic.

## Acceptance Criteria

- [ ] Overlay reflects status within 50 ms of event emission.
- [ ] Logs capture all events with accurate timestamps.
- [ ] No missed or duplicated events observed in 1-hour monkey test (500 invocations).

## Test-Driven Design

Start with failing tests subscribing to the event channel and asserting delivery guarantees
(latency, ordering, no duplicates). Implement until green.

## References

PRD §6 Functional Requirements – FR-7
