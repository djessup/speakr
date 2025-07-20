# NFR: Compatibility

Operate across supported macOS versions and CPU architectures.

## Requirement

- Support **macOS 13+** on Apple Silicon and Intel Macs.
- Intel Macs may experience doubled latency but must remain functional.

## Rationale

Wider OS support increases addressable market while retaining acceptable performance.

## Acceptance Criteria

- [ ] Manual QA passes on Intel MBP 2020 (macOS 13).
- [ ] Automated smoke test on GitHub Actions Intel runner passes.
- [ ] Latency SLA documented separately for Intel.

## Test-Driven Design

Add failing cross-arch smoke tests to CI runners before porting; success criteria met when tests
 pass on Intel and Apple Silicon.

## References

PRD §7 Non-Functional Requirements – Compatibility
