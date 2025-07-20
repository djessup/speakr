# FR-9: Auto-update

Provide optional self-update via GitHub Releases.

## Requirement

1. When enabled, periodically (daily) check GitHub Releases for a newer version tag.
2. Use secure download (HTTPS) and verify code signature / hash before install.
3. Prompt user with *Release Notes* and require confirmation before applying update.
4. Allow users to disable auto-update in Settings.
5. Feature optional in v1; must degrade gracefully when disabled.

## Rationale

Easy updates encourage users to stay on latest version, reducing support burden and delivering
security fixes.

## Acceptance Criteria

- [ ] Update check runs off main thread; no UI freeze.
- [ ] Failed update check logs but does not crash application.
- [ ] Downloaded binary passes macOS notarisation verification.
- [ ] User can opt-out entirely; no network calls when disabled.

## Test-Driven Design

Begin with failing integration tests that simulate update availability, download verification,

## References

PRD §6 Functional Requirements – FR-9
