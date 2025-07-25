---
inclusion: always
---

# Specification Guidelines

## Spec Creation Rules

When creating or updating specifications:

1. **Review existing code first** - Check current implementation in relevant crates
2. **Follow established patterns** - Use existing specs in `docs/specs/` and `.kiro/specs/`
3. **Respect architecture boundaries** - Align with 4-crate workspace structure
4. **Include file references** - Use `#[[file:path]]` syntax for code/config references

## Specification Types & Structure

### Functional Requirements (FR-*)

**Naming**: `FR-{number}-{feature-name}.md` (e.g., `FR-1-global-hotkey.md`)

**Required sections**:

- **Requirements**: What the feature must do
- **Design**: Architecture and component interactions
- **Implementation**: Specific code changes and file locations
- **Testing**: Unit, integration, and acceptance test strategies
- **Acceptance Criteria**: Measurable success conditions

### Non-Functional Requirements (NFR-*)

**Naming**: `NFR-{category}.md` (e.g., `NFR-latency.md`)

**Must include**:

- **Metrics**: Specific, measurable targets (≤3s latency, ≤20MB binary)
- **Quality gates**: Pass/fail criteria (>99.5% crash-free sessions)
- **Platform constraints**: macOS versions, hardware requirements

### Testing Strategy

- **Unit tests**: `#[cfg(test)]` modules for business logic
- **Integration tests**: `tests/` directory for cross-module functionality
- **Test isolation**: Use `tempfile::TempDir` for filesystem tests
- **Mock dependencies**: Audio devices, system APIs

## Specification Validation Checklist

Before finalizing specs:

- [ ] Check for existing similar functionality
- [ ] Verify crate boundary alignment
- [ ] Assess performance impact
- [ ] Define testing approach
- [ ] Plan user documentation

## File Reference Syntax

Use `#[[file:relative_path]]` to reference:

- Implementation files
- Configuration schemas
- Test files
- API definitions

This keeps specs synchronized with actual code changes.
