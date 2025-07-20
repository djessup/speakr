# NFR: Footprint

Constrain binary size and runtime memory usage.

## Requirement

- Universal macOS binary size ≤ **20 MB** (excluding model files).
- Peak RSS ≤ **400 MB** including model during standard transcription workload.

## Rationale

A lightweight application reduces download size, disk usage and keeps memory pressure low on older
devices.

## Acceptance Criteria

- [ ] `du -h` on release DMG shows ≤ 20 MB binary.
- [ ] Runtime memory measured via Activity Monitor stays ≤ 400 MB during 30 s monkey test.

## Test-Driven Design

Add failing size and memory regression tests into CI before implementation tweaks.

## References

PRD §7 Non-Functional Requirements – Footprint
