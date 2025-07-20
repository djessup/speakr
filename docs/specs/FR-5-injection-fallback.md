# FR-5: Injection Fallback

Clipboard-paste fallback when keystroke injection is blocked.

## Requirement

1. Detect secure text fields or injection failure (e.g. `enigo` error).
2. Copy transcript to clipboard and simulate ⌘V paste as fallback.
3. Display transient warning overlay: *“Secure field detected – text pasted via clipboard.”*
4. Restore previous clipboard contents after paste to respect user data.

## Rationale

Some password or secure fields block synthetic keystrokes. A controlled clipboard fallback ensures
functionality while informing the user.

## Acceptance Criteria

- [ ] 100 % success rate pasting into macOS secure text fields (Safari password prompt as test).
- [ ] Previous clipboard restored within 500 ms after paste.
- [ ] Warning overlay disappears automatically after 3 s.
- [ ] No sensitive transcript retained on clipboard after restore.

## Test-Driven Design

Craft failing tests for secure-field detection, clipboard restoration, and overlay timing. Implement
 fallback logic until tests succeed.

## References

PRD §6 Functional Requirements – FR-5
