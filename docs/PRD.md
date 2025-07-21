---
title: Product Requirements Document – Speakr
version: 2025-07-20
status: Draft
authors: David Jessup
---
# Product Requirements Document – Speakr

- [1. Purpose / Vision](#1-purpose--vision)
- [2. Problem Statement](#2-problem-statement)
- [3. Goals \& Non-Goals](#3-goals--non-goals)
  - [3.1 Goals](#31-goals)
  - [3.2 Non-Goals](#32-non-goals)
- [4. Personas](#4-personas)
- [5. User Stories](#5-user-stories)
- [6. Functional Requirements](#6-functional-requirements)
- [7. Non-Functional Requirements](#7-non-functional-requirements)
- [8. Metrics / KPIs](#8-metrics--kpis)
- [9. Milestones](#9-milestones)
- [10. Open Questions](#10-open-questions)
- [11. Appendix – Stakeholders \& Review](#11-appendix--stakeholders--review)

## 1. Purpose / Vision

Speakr is a **privacy-first dictation hot-key utility** for macOS (Windows/Linux later). In a single
keystroke, users can record speech, **transcribe entirely on-device**, and have the text typed
directly into any active input field. Speakr aims to be the fastest way for developers, writers,
and power-users to turn fleeting thoughts into code or prose without breaking flow, and
**without sending audio to the cloud**.

## 2. Problem Statement

1. Switching to dedicated dictation apps breaks focus and incurs network latency.
2. Many corporate or offline environments forbid cloud speech services for privacy reasons.
3. OS-level dictation is unreliable for code, lacks custom hot-keys, and has high latency on older
   hardware.

**Opportunity**: A lightweight, keyboard-driven tool that works anywhere text can be typed, requires
no network, and respects user privacy.

## 3. Goals & Non-Goals

### 3.1 Goals

1. **<= 3 s** end-to-end latency for 5-second recordings on Apple Silicon (M-series).
2. **100% offline** – no external network calls.
3. Global hot-key works in background apps.
4. Support customisable models & hot-keys via UI.
5. Ship notarised universal macOS binary < 20 MB (excluding model).
6. Provide a clean upgrade path to Windows & Linux.

### 3.2 Non-Goals

- Real-time streaming (v1 may paste only after stop).
- Mobile platforms.
- Full grammar / punctuation correction.
- Server-side sync or accounts.

## 4. Personas

| Persona               | Needs / Pain-points                                                 |
| --------------------- | ------------------------------------------------------------------- |
| **Dev Dana**          | Insert comments/code quickly without losing keyboard context.       |
| **Writer Will**       | Draft snippets into any text editor without toggling apps.          |
| **Privacy Peter**  | Dictate confidential material offline, no data leaves device.       |
| **Accessibility Ava** | Replace or augment typing due to RSI, keep workflow keyboard-first. |

## 5. User Stories

> _MoSCoW method: Must, Should, Could, Won’t (for now)_

| Priority | Description |
| --- | --- |
| **Must** | _“As a user, I press `<Opt>` + `~` and my spoken words (≤30 s) are typed into the active field within ~3 s.”_ |
| **Must** | _“As a user, the app asks for mic + Accessibility permissions on first run and explains why.”_ |
| **Must** | _“As a user, I can change the hot-key in settings and be warned of conflicts.”_ |
| **Should** | _“As a user, I can pick a smaller/faster model if my machine is slow.”_ |
| **Should** | _“As a user, a subtle overlay shows ‘Recording… / Transcribing…’ states.”_ |
| **Could** | _“As an advanced user, I can turn on auto-punctuation.”_ |
| **Could** | _“As an advanced user, I can add bespoke words to the dictionary.”_ |
| **Won’t (v1)** | _Live transcript shown word-by-word while speaking._ |

## 6. Functional Requirements

| FR | Description |
| --- | --- |
| FR-1 | Global hot-key registers at app start and triggers record/transcribe/inject flow. |
| FR-2 | Audio capture uses 16 kHz mono via `cpal`, max configurable duration (default 10 s). |
| FR-3 | Transcription runs through Whisper (GGUF) via `whisper-rs`; language default EN. |
| FR-4 | Transcript is injected via synthetic keystrokes (`enigo`) into current focus. |
| FR-5 | If injection fails (secure field), fallback to clipboard-paste with user warning. |
| FR-6 | UI (tray or window) exposes: hot-key picker, model selector, auto-launch toggle. |
| FR-7 | App emits status events for UI overlay and logs (Recording, Transcribing, Error). |
| FR-8 | Settings persist locally (JSON in AppData,  no cloud). |
| FR-9 | App auto-updates via GitHub Releases (optional in v1). |

## 7. Non-Functional Requirements

| Category          | Requirement                                                      | Metric / Acceptance                                |
| ----------------- | ---------------------------------------------------------------- | -------------------------------------------------- |
| **Latency**       | End-to-end ≤ 3 s (M1, 5 s audio, small model)                    | 95th percentile measured in telemetry log (local). |
| **Footprint**     | Binary ≤ 20 MB; RAM ≤ 400 MB including model.                    | `du -sh` and Activity Monitor/smoke tests.         |
| **Reliability**   | No crashes in 1-hour monkey test (500 invocations).              | CI integration test + manual QA.                   |
| **Security**      | No outbound network sockets except auto-update domain (opt-out). | Static analysis + firewall test.                   |
| **Compatibility** | macOS 13+. Intel macs may see doubled latency but functional.    | QA on Intel MBP (2020) & M1.                       |
| **Accessibility** | Follows macOS VoiceOver / high-contrast guidelines.              | Apple Accessibility Inspector score ≥ 85.          |

## 8. Metrics / KPIs

| Metric | Target |
| --- | --- |
| **Time-to-text (P95)** | ≤ 3 s. |
| **Activation success rate** | ≥ 99% (hot-key triggers & types). |
| **Crash-free sessions** | > 99.5%. |
| **Daily active users (DAU)** | post-launch target: 1 k. |
| **% of transcripts requiring manual fix** | < 15% (optional feedback prompt). |

## 9. Milestones

| Milestone                 | Scope                                       |
| ------------------------- | ----------- |
| M0  – Prototype spike     | Hot-key → record → transcribe → paste (CLI) |
| M1  – MVP macOS app       | Tauri shell, settings window, notarised DMG |
| M2  – Public beta         | Auto-update, error logs, model manager      |
| M3  – Windows/Linux alpha | Replace injection backend, install bundles  |
| M4  – v1.0 GA             | Streaming (optional), website + docs        |

## 10. Open Questions

1. Should we bundle a small GGUF model or trigger a first-run download wizard?
2. How to handle non-Latin languages (auto-detect vs user-select)?
3. Do we sandbox the app on macOS or rely on hardened runtime?
4. Which licence (MIT vs GPL) given we embed Whisper weights?
5. Accept user telemetry opt-in for latency metrics?

## 11. Appendix – Stakeholders & Review

- **Product Lead** – @PM
- **Engineering Lead** – @TechLead
- **Design** – @UX
- **Security** – @Sec
- **QA** – @QA

_Reviews_: Architecture (Tech), Security (Sec), Accessibility (UX).
