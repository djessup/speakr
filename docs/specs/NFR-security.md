# NFR: Security

Prevent unintended data leakage and maintain user privacy through comprehensive security controls.

## Requirements

### Network Security

- No outbound network connections except optional auto-update domain.
- Hardened runtime & proper code-signing for macOS notarisation.
- Microphone access prompt shown once and justification provided.

### Data Security & Input Validation

- Settings files must be protected against DoS attacks via size limits.
- All deserializable data structures must reject unknown fields.
- File path inputs must be validated against path traversal attacks.
- JSON parsing errors must provide detailed diagnostics without information leakage.
- Configuration persistence must use atomic operations with backup/recovery.

## Rationale

Privacy-first positioning requires strict control over network activity, OS security policies, and
robust protection against malicious input that could compromise system integrity or user data.

## Acceptance Criteria

### Network Security

- [ ] Static analysis shows no runtime socket creation beyond update URL when enabled.
- [ ] Application passes Apple notarisation & gatekeeper checks.
- [ ] Firewall test (Little Snitch) reveals no unexpected traffic.

### Data Security

- [x] Settings files are limited to 64KB to prevent DoS attacks.
- [x] All deserializable structs use `#[serde(deny_unknown_fields)]` to reject malicious JSON.
- [x] File path validation rejects `..`, absolute paths, and control characters.
- [x] Enhanced JSON parsing errors with `serde_path_to_error` provide field-level diagnostics.
- [x] Atomic file operations with backup/recovery mechanisms for settings persistence.

## Implementation

### Serde Security Measures (Completed)

**File Size Protection:**

```rust
pub const MAX_SETTINGS_FILE_SIZE: usize = 64 * 1024; // 64KB limit

// Check file size before reading
let metadata = fs::metadata(path)?;
if metadata.len() > MAX_SETTINGS_FILE_SIZE as u64 {
    return Err("Settings file too large");
}
```

**Schema Validation:**

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // Reject malicious extra fields
pub struct AppSettings {
    // ... fields
}
```

**Path Traversal Protection:**

```rust
pub fn validate_file_path(path: &str) -> bool {
    // Reject paths with traversal attempts, absolute paths, etc.
    !path.contains("..") && !path.starts_with('/') && !path.chars().any(|c| c.is_control())
}
```

## Test-Driven Design

Write security unit tests (e.g., socket mocks, malicious input tests) and notarisation validation
scripts before code changes; CI must enforce them.

## References

PRD §7 Non-Functional Requirements – Security
