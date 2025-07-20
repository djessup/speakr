# NFR: Latency

Ensure low end-to-end latency from hot-key activation to text injection.

## Requirement

- 95th percentile **time-to-text ≤ 3 s** for a 5-second audio clip on Apple Silicon (M1) using the
- small Whisper model.
- Latency measured in release (optimised) builds with all background services running.

## Rationale

Sub-3-second latency preserves conversational flow and competitive advantage over cloud dictation.

## Acceptance Criteria

- [ ] Automated telemetry logs latency for every invocation.
- [ ] CI latency test passes on GitHub Actions M1 runner.
- [ ] Performance regression test fails build if P95 > 3 s.

## Test-Driven Design

Create automated performance tests that measure P95 latency; commit them before optimising the code.

## References

PRD §7 Non-Functional Requirements – Latency
