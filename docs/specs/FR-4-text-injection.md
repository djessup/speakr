# FR-4: Transcript Injection

Types the transcribed text into the currently focused input field.

## Requirement

1. Use the `enigo` crate to emit synthetic keystrokes that reproduce the transcription exactly as
2. plain text.
3. Injection must preserve line breaks and punctuation.
4. Injection must run on the main UI thread to respect macOS accessibility APIs.
5. Provide feedback event (e.g. `Injected`) to UI overlay/log once complete.

## Rationale

Typing text directly avoids clipboard usage and works in most applications, maintaining illusion of
native typing.

## Acceptance Criteria

- [ ] For a 100-character transcript, injection latency ≤ 300 ms.
- [ ] Typed characters match transcription byte-for-byte.
- [ ] Works in common editors (VS Code, Xcode, Pages, Safari).
- [ ] Emits completion event for downstream UI.

## Test-Driven Design

Write failing integration tests measuring injection latency and correctness across target editors.
Deliver code to satisfy the tests.

## References

PRD §6 Functional Requirements – FR-4
