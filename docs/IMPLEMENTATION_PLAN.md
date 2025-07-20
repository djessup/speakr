# Implementation Plan – Speakr

A step-by-step roadmap to deliver the Speakr application using the **test-driven, multi-crate**
approach defined in the specification set under `docs/specs/`.

---

## 1. Repository Scaffold

Reference: [INIT-01 Project Scaffold](specs/INIT-01-project-scaffold.md)

1. Execute the migration steps to create the Cargo workspace (`speakr-core`, `speakr-tauri`,
2. optional `speakr-ui`).
3. Commit and open a draft PR; CI should fail until tests are added.
4. Add baseline CI workflows (lint, build, placeholder tests) that currently fail.

## 2. Core Library (`speakr-core`)

| Order | Spec                                     | Task                                                                                                        |
| ----- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| 2.1   | [FR-2](specs/FR-2-audio-capture.md)      | Implement audio capture (`cpal`). Begin with failing unit test asserting 16-kHz mono stream & duration cap. |
| 2.2   | [FR-3](specs/FR-3-transcription.md)      | Implement transcription (`whisper-rs`). Add latency test harness.                                           |
| 2.3   | [FR-4](specs/FR-4-text-injection.md)     | Implement text injection (`enigo`). Integration tests across editors via mock window focus.                 |
| 2.4   | [FR-5](specs/FR-5-injection-fallback.md) | Implement clipboard fallback; write secure-field simulation tests.                                          |
| 2.5   | [FR-7](specs/FR-7-status-events.md)      | Emit status events; test channel delivery & ordering.                                                       |

Merge each sub-task when its tests pass and CI is green.

## 3. Tauri Backend (`speakr-tauri`)

| Order | Spec                                       | Task                                                                                                   |
| ----- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| 3.1   | [FR-1](specs/FR-1-global-hotkey.md)        | Register global hot-key via `tauri-plugin-global-shortcut`; write E2E test with headless Tauri window. |
| 3.2   | —                                          | Wire hot-key → async call into `speakr-core` pipeline; ensure status events are forwarded via `emit`.  |
| 3.3   | [FR-8](specs/FR-8-settings-persistence.md) | Add settings persistence (JSON). Unit tests for load/save & corruption recovery.                       |

## 4. Front-End (Leptos)

| Order | Spec                                            | Task                                                                                     |
| ----- | ----------------------------------------------- | ---------------------------------------------------------------------------------------- |
| 4.1   | [FR-6](specs/FR-6-settings-ui.md)               | Build Settings & Status overlay UI; write component tests with Leptos testing utilities. |
| 4.2   | [NFR-accessibility](specs/NFR-accessibility.md) | Add automated axe-core & VoiceOver tests.                                                |

## 5. Cross-Cutting Non-Functional Work

| Spec                                            | Focus                                                                 |
| ----------------------------------------------- | --------------------------------------------------------------------- |
| [NFR-latency](specs/NFR-latency.md)             | Optimise model loading & thread usage; ensure performance tests pass. |
| [NFR-footprint](specs/NFR-footprint.md)         | Strip symbols, enable `lto`, audit memory.                            |
| [NFR-reliability](specs/NFR-reliability.md)     | Add monkey-test CI job (500 invocations).                             |
| [NFR-security](specs/NFR-security.md)           | Socket-mock tests, Hardened Runtime flags, notarisation script.       |
| [NFR-compatibility](specs/NFR-compatibility.md) | Add Intel macOS runner to CI.                                         |

## 6. Auto-Update

Reference: [FR-9 Auto-update](specs/FR-9-auto-update.md)

1. Integrate update check using `tauri-plugin-updater` (or custom).
2. Write integration tests mocking GitHub Releases API & download validation.

## 7. Documentation & Release

1. Update `docs/book/` with usage & contribution guide.
2. Ensure `mdbook` build passes in CI.
3. Produce signed DMG via CI; attach to GitHub Release.

---

## Progress Checklist

- [ ] 0. Preparation complete
- [ ] 1. Repository scaffold merged ([INIT-01](specs/INIT-01-project-scaffold.md))
- [ ] 2.1 Audio capture ([FR-2](specs/FR-2-audio-capture.md)) implemented & tested
- [ ] 2.2 Transcription ([FR-3](specs/FR-3-transcription.md)) implemented & tested
- [ ] 2.3 Text injection ([FR-4](specs/FR-4-text-injection.md)) implemented & tested
- [ ] 2.4 Injection fallback ([FR-5](specs/FR-5-injection-fallback.md)) implemented & tested
- [ ] 2.5 Status events ([FR-7](specs/FR-7-status-events.md)) implemented & tested
- [ ] 3.1 Global hot-key ([FR-1](specs/FR-1-global-hotkey.md)) registered & tested
- [ ] 3.2 Backend pipeline wired
- [ ] 3.3 Settings persistence ([FR-8](specs/FR-8-settings-persistence.md)) implemented & tested
- [ ] 4.1 Settings UI ([FR-6](specs/FR-6-settings-ui.md)) implemented & tested
- [ ] 4.2 Accessibility audits ([NFR-accessibility](specs/NFR-accessibility.md)) passing
- [ ] Non-functional targets ([Latency](specs/NFR-latency.md), [Footprint](specs/NFR-footprint.md),
      [Reliability](specs/NFR-reliability.md), [Security](specs/NFR-security.md),
      [Compatibility](specs/NFR-compatibility.md)) met
- [ ] Auto-update ([FR-9](specs/FR-9-auto-update.md)) implemented & tested
- [ ] Docs & Release pipeline finished

Tick each box as the corresponding PR merges with passing CI.

## Recent Progress (2025-07-20)

- Scaffolded `speakr-core` library crate and added it to the workspace manifest.
- Added stub implementation (`record_to_vec`) and constants in `speakr-core::audio`.
- Committed **failing** unit test `audio_capture.rs` verifying 16 kHz mono stream and placeholders.
- Workspace compiles; test fails as expected, ready for implementation phase.
