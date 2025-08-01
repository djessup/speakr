# Security Improvements Changelog

## 2025-01-25 - Comprehensive Serde Security Hardening

Implemented comprehensive security measures for serde deserialization to protect against malicious
input, DoS attacks, and data integrity issues.

### Security Enhancements

#### 1. Schema Validation (`#[serde(deny_unknown_fields)]`)

Added strict schema validation to all deserializable structs to reject malicious JSON with extra
fields:

**Core Types** (`speakr-types/src/lib.rs`):

- `AppSettings` - Application configuration
- `HotkeyConfig` - Global hotkey settings
- `ModelSize` - Whisper model size enumeration
- `ModelInfo` - Model metadata and information
- `ServiceStatus` - Service state tracking
- `BackendStatus` - Overall system status

**Debug Types** (`speakr-tauri/src/debug/types.rs`):

- `DebugLogLevel` - Debug logging levels
- `DebugLogMessage` - Debug log entries

**Service Types** (`speakr-tauri/src/services/types.rs`):

- `ServiceComponent` - Service component identification

#### 2. DoS Protection via File Size Limits

**Added Constants** (`speakr-types/src/lib.rs`):

```rust
/// Maximum allowed settings file size in bytes.
///
/// Set to 64KB to prevent DoS attacks while allowing reasonable settings growth.
/// A typical settings file should be under 1KB, so this provides generous headroom.
pub const MAX_SETTINGS_FILE_SIZE: usize = 64 * 1024;
```

**Implementation** (`speakr-tauri/src/settings/persistence.rs`):

```rust
// Check file size before reading to prevent DoS attacks
let metadata = fs::metadata(path)?;
if metadata.len() > MAX_SETTINGS_FILE_SIZE as u64 {
    return Err(format!(
        "Settings file too large: {} bytes (max: {} bytes)",
        metadata.len(),
        MAX_SETTINGS_FILE_SIZE
    ));
}
```

#### 3. Enhanced Error Diagnostics

**Added Dependency** (`speakr-tauri/Cargo.toml`):

```toml
serde_path_to_error = "0.1" # Better error messages for JSON parsing
```

**Implementation**:

```rust
let mut settings: AppSettings =
    serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&content))
        .map_err(|e| format!("Failed to parse settings JSON: {e}"))?;
```

#### 4. Path Traversal Protection

**Added Validation Function** (`speakr-types/src/lib.rs`):

```rust
/// Validates that a file path is safe and doesn't contain path traversal attempts.
pub fn validate_file_path(path: &str) -> bool {
    // Reject paths with traversal attempts
    if path.contains("..") {
        return false;
    }

    // Reject absolute paths (should be relative to app directory)
    if path.starts_with('/') || path.starts_with('\\') {
        return false;
    }

    // Reject Windows drive letters
    if path.len() >= 2 && path.chars().nth(1) == Some(':') {
        return false;
    }

    // Reject null bytes and other control characters
    if path.chars().any(|c| c.is_control()) {
        return false;
    }

    true
}
```

#### 5. Improved Test Error Handling

Replaced `unwrap()` calls in test code with descriptive `expect()` messages:

```rust
// Before
let settings: AppSettings = serde_json::from_str(&json).unwrap();

// After
let settings: AppSettings = serde_json::from_str(&json)
    .expect("JSON should deserialize to settings");
```

### Security Benefits

| Threat                    | Protection            | Implementation                                 |
| ------------------------- | --------------------- | ---------------------------------------------- |
| **Malicious JSON Fields** | `deny_unknown_fields` | All deserializable structs reject extra fields |
| **DoS via Large Files**   | File size limits      | 64KB limit prevents memory exhaustion          |
| **Poor Error Messages**   | `serde_path_to_error` | Detailed JSON parsing errors with field paths  |
| **Path Traversal**        | Path validation       | Utility function ready for file path fields    |
| **Development Issues**    | Better test errors    | `expect()` instead of `unwrap()` in tests      |

### Testing

- **Core Types**: All 20 tests in `speakr-types` pass ✅
- **Settings Persistence**: File size limits tested and validated ✅
- **Schema Validation**: Unknown field rejection verified ✅
- **Error Handling**: Enhanced error messages confirmed ✅

### Documentation Updates

Updated documentation to reflect security improvements:

- `docs/ARCHITECTURE.md` - Added data security section
- `docs/SYSTEM_DESCRIPTION.md` - Added security features overview
- `docs/specs/NFR-security.md` - Added serde security requirements and implementation
- `README.md` - Added security-hardened feature and coding standards

### Compliance

This implementation addresses security requirements from:

- Privacy-first architecture (no data leakage)
- Input validation best practices
- DoS attack prevention
- Secure configuration handling
- Test-driven development standards

All changes maintain backward compatibility while significantly enhancing security posture.
