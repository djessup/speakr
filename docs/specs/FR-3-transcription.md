# FR-3: Transcription

Offline transcription of recorded audio to text using Whisper.

## Requirement

1. Use `whisper-rs` to run Whisper (GGUF) models entirely on-device.
2. Default language: English (en). Allow user language selection in Settings.
3. Transcription must complete within **≤ 3 s** (95th percentile) for 5-second recordings on Apple
4. Silicon with the small model.
5. Support user-selectable model sizes for latency/accuracy trade-off.
6. No external network calls during transcription.

## Rationale

On-device inference preserves privacy and removes network latency, achieving the product’s
privacy-first promise.

## Acceptance Criteria

- [ ] Transcription completes within latency budget on M1 and Intel reference machines.
- [ ] Selecting a different model in Settings updates the engine without restart.
- [ ] No outbound network traffic observed via packet capture.
- [ ] Errors (e.g. model missing) surface in UI overlay/log with actionable message.

## Test-Driven Design

Begin with failing automated tests for latency, language selection, and network isolation.
Implement transcription until all tests pass, following TDD.

## References

PRD §6 Functional Requirements – FR-3
