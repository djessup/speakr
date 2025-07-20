# NFR: Accessibility

Comply with macOS accessibility guidelines.

## Requirement

- UI elements (overlay, settings) must be VoiceOver readable.
- Support high-contrast mode and respect user font scaling preferences.
- Achieve Apple Accessibility Inspector score ≥ **85**.

## Rationale

Ensures inclusivity for users with visual impairments or other accessibility needs.

## Acceptance Criteria

- [ ] VoiceOver reads overlay status changes accurately.
- [ ] High-contrast mode renders UI with sufficient contrast ratios (> 4.5:1).
- [ ] Automated accessibility audit (axe-core) passes with no critical violations.

## Test-Driven Design

Introduce automated accessibility audits (axe-core, VoiceOver scripts) in CI before fixing
violations.

## References

PRD §7 Non-Functional Requirements – Accessibility
