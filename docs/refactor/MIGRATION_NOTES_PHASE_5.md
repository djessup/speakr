# Migration Notes: Phase 5 Refactor - Command Organisation

## Overview

Phase 5 of the Speakr Tauri backend refactor extracted remaining commands into dedicated
modules and finalised the cleanup of `lib.rs`. This document provides guidance for developers
working with the new structure.

## What Changed

### Before (Pre-Phase 5)

- All command implementations lived in `lib.rs`
- File was over 1000+ lines with mixed concerns
- Commands, services, and business logic were intermingled
- Testing required testing through Tauri command wrappers

### After (Phase 5 Complete)

- Commands organised into functional modules under `commands/`
- Each command has an `*_internal()` function with business logic
- Tauri command wrappers remain in `lib.rs` for registration
- `lib.rs` reduced to ~400 lines, focused on configuration and integration

## New File Structure

```text
speakr-tauri/src/
├── commands/
│   ├── mod.rs          # Command organisation and documentation
│   ├── validation.rs   # Input validation commands
│   ├── system.rs       # System integration commands
│   └── legacy.rs       # Backward compatibility commands
├── services/           # (From previous phases)
│   ├── mod.rs
│   ├── hotkey.rs
│   ├── status.rs
│   └── types.rs
├── settings/           # (From previous phases)
├── debug/              # (From previous phases)
├── audio/              # (From previous phases)
└── lib.rs              # Tauri integration and command registration
```

## Command Implementation Pattern

### New Pattern (Recommended)

```rust
// In commands/validation.rs
pub async fn validate_hot_key_internal(hot_key: String) -> Result<(), AppError> {
    // Business logic here
    Ok(())
}

// In lib.rs
#[tauri::command]
async fn validate_hot_key(hot_key: String) -> Result<(), AppError> {
    validate_hot_key_internal(hot_key).await
}
```

### Key Benefits

1. **Testability**: Internal functions can be tested without Tauri overhead
2. **Modularity**: Commands grouped by functional domain
3. **Maintainability**: Business logic separated from framework concerns
4. **Documentation**: Each module has focused documentation

## Working with Commands

### Adding a New Command

1. **Choose the appropriate module** (`validation`, `system`, or `legacy`)
2. **Implement the internal function**:

   ```rust
   pub async fn my_command_internal(param: String) -> Result<T, AppError> {
       // Implementation here
   }
   ```

3. **Add Tauri wrapper in `lib.rs`**:

   ```rust
   #[tauri::command]
   async fn my_command(param: String) -> Result<T, AppError> {
       my_command_internal(param).await
   }
   ```

4. **Register in `run()` function**:

   ```rust
   .invoke_handler(tauri::generate_handler![
       // ... existing commands,
       my_command
   ])
   ```

5. **Add comprehensive tests** for the internal function

### Command Module Guidelines

- **`validation.rs`**: Input validation, sanitisation, format checking
- **`system.rs`**: OS integration, file system, auto-launch, model availability
- **`legacy.rs`**: Deprecated or backward-compatibility commands

### Testing Commands

```rust
// Test the internal function directly
#[tokio::test]
async fn test_my_command_internal() {
    let result = my_command_internal("test".to_string()).await;
    assert!(result.is_ok());
}
```

## Breaking Changes

### Import Changes

Commands moved from `crate::*` to `crate::commands::*`:

```rust
// Old (no longer works)
use crate::validate_hot_key_internal;

// New
use crate::commands::validation::validate_hot_key_internal;
```

### Function Visibility

Internal functions changed from `pub(crate)` to `pub` to allow cross-module access:

```rust
// Old
pub(crate) async fn validate_hot_key_internal(...) -> ...

// New
pub async fn validate_hot_key_internal(...) -> ...
```

## Error Handling

### Consistent Error Types

All commands use `speakr_types::AppError` for error handling:

```rust
pub enum AppError {
    HotKey(String),
    Settings(String),
    FileSystem(String),
    // ... other variants
}
```

### Error Context

Add context to errors for better debugging:

```rust
Err(AppError::Settings(format!("Invalid model size: {model_size}")))
```

## Documentation Standards

### Function Documentation

All public functions must have rustdoc comments:

```rust
/// Brief description of what the function does.
///
/// # Arguments
///
/// * `param` - Description of the parameter
///
/// # Returns
///
/// Description of what is returned.
///
/// # Errors
///
/// Conditions that cause errors.
///
/// # Examples
///
/// ```rust,no_run
/// use speakr_lib::commands::validation::validate_hot_key_internal;
/// // Example usage
/// ```
pub async fn my_function_internal(param: String) -> Result<(), AppError> {
    // Implementation
}
```

### Module Documentation

Each module should have comprehensive documentation explaining its purpose and usage patterns.

## Testing Strategy

### Unit Tests

- Test internal functions directly (not through Tauri wrappers)
- Use test isolation patterns for file system operations
- Mock external dependencies where possible

### Test Organisation

Tests live alongside code in `mod tests` blocks:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_function_success() {
        // Test implementation
    }
}
```

## Backward Compatibility

### Legacy Support

Commands in `legacy.rs` maintain backward compatibility but should be considered deprecated
for new development.

### Deprecation Path

When deprecating commands:

1. Move to `legacy.rs`
2. Add deprecation notice in documentation
3. Provide migration path in rustdoc

## Performance Considerations

### Command Overhead

The new pattern adds minimal overhead:

- Internal functions: Direct function calls
- Tauri wrappers: Thin delegation layer

### Memory Usage

- Internal functions can be tested in isolation without Tauri runtime
- Reduced memory usage during testing
- Better compiler optimisations due to cleaner module boundaries

## Common Patterns

### Input Validation

```rust
pub async fn validate_input_internal(input: String) -> Result<(), AppError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(AppError::Settings("Input cannot be empty".to_string()));
    }
    // Additional validation...
    Ok(())
}
```

### File System Operations

```rust
pub async fn check_file_internal(path: String) -> Result<bool, AppError> {
    let path = std::path::Path::new(&path);
    match path.exists() {
        true => Ok(true),
        false => Ok(false),
    }
}
```

### Error Propagation

```rust
pub async fn complex_operation_internal() -> Result<T, AppError> {
    let result = validate_input_internal(input).await?;
    let file_exists = check_file_internal(path).await?;
    // Process results...
    Ok(final_result)
}
```

## Future Development

### Adding New Modules

If the `commands/` directory grows too large, consider:

1. Creating subdirectories for related commands
2. Grouping by feature area rather than technical function
3. Maintaining the `*_internal` + wrapper pattern

### Architectural Evolution

The current pattern supports:

- Easy migration to other frameworks (business logic is framework-agnostic)
- Microservice extraction (internal functions are self-contained)
- Enhanced testing strategies (direct function testing)

## Troubleshooting

### Common Issues

1. **Import errors**: Check if function moved to new module
2. **Visibility errors**: Internal functions are now `pub`, not `pub(crate)`
3. **Test failures**: Update imports in test files
4. **Documentation tests**: Use `speakr_lib` as crate name, not `speakr_tauri`

### Migration Checklist

When updating code that depends on the old structure:

- [x] Update imports to new module paths ✅
- [x] Change function visibility if needed ✅
- [x] Update test imports and assertions ✅
- [x] Fix documentation examples with correct crate name ✅
- [x] Verify error handling uses `AppError` consistently ✅

---

*Last Updated: Phase 5 Complete*
*For questions about this refactor, see the original planning documents in `docs/refactor/`*
