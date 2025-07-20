# NFR: Reliability

Maintain stability across heavy usage.

## Requirement

- Application must run **1-hour monkey test (500 invocations)** with zero crashes.
- Recover gracefully from errors (audio device unavailable, model missing).

## Rationale

High reliability builds user trust and reduces support overhead.

## Acceptance Criteria

- [ ] CI integration test simulates 500 sequential hot-key invocations without crash.
- [ ] Error conditions logged and surfaced via Status Events.

## Test-Driven Design

Introduce a failing soak-test (500 invocations) in CI first; stabilise code until it passes
 consistently.

## References

PRD §7 Non-Functional Requirements – Reliability
