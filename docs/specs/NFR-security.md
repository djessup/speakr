# NFR: Security

Prevent unintended data leakage and maintain user privacy.

## Requirement

- No outbound network connections except optional auto-update domain.
- Hardened runtime & proper code-signing for macOS notarisation.
- Microphone access prompt shown once and justification provided.

## Rationale

Privacy-first positioning requires strict control over network activity and OS security policies.

## Acceptance Criteria

- [ ] Static analysis shows no runtime socket creation beyond update URL when enabled.
- [ ] Application passes Apple notarisation & gatekeeper checks.
- [ ] Firewall test (Little Snitch) reveals no unexpected traffic.

## Test-Driven Design

Write security unit tests (e.g., socket mocks) and notarisation validation scripts before code
changes; CI must enforce them.

## References

PRD §7 Non-Functional Requirements – Security
