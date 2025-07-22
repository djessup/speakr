# Debug Panel Documentation

The Speakr debug panel is a development-only interface that provides debugging tools and testing capabilities. It's designed to help developers test features, monitor system behaviour, and troubleshoot issues during development.

## Overview

The debug panel is only available in debug builds (`cargo tauri dev`) and is completely excluded from release builds for security and performance reasons. It provides a comprehensive debugging interface with real-time logging, feature testing, and system monitoring capabilities.

## Accessing the Debug Panel

### Availability

- **Debug builds only**: The panel is conditionally compiled using `#[cfg(debug_assertions)]`
- **Toggle button**: A red "🛠️ Debug" button appears in the header (debug builds only)
- **Visual indicator**: The panel shows a "DEBUG BUILD" badge to remind developers of the build type

### Navigation

1. Start the application in debug mode: `cargo tauri dev`
2. Look for the "🛠️ Debug" button in the top-right corner of the header
3. Click to toggle between the settings panel and debug panel
4. The button text changes to "🛠️ Hide Debug" when the panel is active

## Features

### 1. Audio Testing

#### Legacy Test Button

- **Purpose**: Basic audio system testing
- **Behaviour**: Click to run a simple audio recording test
- **Feedback**: Shows progress in the debug output area

#### Push-to-Talk Recording

- **Purpose**: Test real-time audio recording with push-to-talk interaction
- **Behaviour**:
  - Hold the button to start recording
  - Release to stop recording
  - Supports both mouse and touch events
- **Visual feedback**:
  - Button changes colour and shows pulsing animation when recording
  - Text updates to show current state
  - Recording state is displayed in system info

### 2. Logging Console

#### Real-time Log Display

- **Scrolling console**: Shows recent log messages from the backend
- **Auto-scroll**: Automatically scrolls to show newest messages (toggleable)
- **Timestamp**: Each message includes precise timestamp
- **Source tracking**: Shows which component generated each log message

#### Log Level Filtering

- **Dropdown filter**: Filter by specific log levels (TRACE, DEBUG, INFO, WARN, ERROR)
- **Visual indicators**: Each level has distinct emoji icons and colours
- **Level-specific styling**: Error and warning messages have highlighted backgrounds

#### Console Controls

- **Refresh**: Manually refresh log messages from backend
- **Clear**: Clear all log messages from display and backend storage
- **Auto-scroll toggle**: Enable/disable automatic scrolling to newest messages

### 3. System Information

Real-time display of:

- **Build type**: Always shows "Debug" in debug panel
- **Environment**: Shows "Development"
- **Recording state**: Live status of audio recording (Active/Inactive)

## Technical Implementation

### Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Frontend      │    │     Backend      │    │  Log Storage    │
│   (Leptos)      │    │    (Tauri)       │    │   (Memory)      │
├─────────────────┤    ├──────────────────┤    ├─────────────────┤
│ DebugPanel      │◄──►│ debug_* commands │◄──►│ DEBUG_LOG_      │
│ LoggingConsole  │    │ add_debug_log()  │    │ MESSAGES        │
│ Push-to-talk UI │    │ Log collection   │    │ (VecDeque)      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Conditional Compilation

The debug panel uses Rust's conditional compilation to ensure it's only included in debug builds:

```rust
#[cfg(debug_assertions)]
mod debug;

#[cfg(debug_assertions)]
use crate::debug::DebugPanel;
```

### Backend Commands

All debug commands are prefixed with `debug_` and conditionally compiled:

- `debug_test_audio_recording()` - Legacy audio test
- `debug_start_recording()` - Start push-to-talk recording
- `debug_stop_recording()` - Stop push-to-talk recording
- `debug_get_log_messages()` - Retrieve stored log messages
- `debug_clear_log_messages()` - Clear log message storage

### Log Message Storage

Debug logs are stored in memory using a thread-safe circular buffer:

```rust
static DEBUG_LOG_MESSAGES: LazyLock<Arc<Mutex<VecDeque<DebugLogMessage>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(VecDeque::with_capacity(1000))));
```

**Key characteristics:**

- **Capacity**: Limited to 1000 messages (prevents memory bloat)
- **Thread-safe**: Uses `Arc<Mutex<>>` for concurrent access
- **Circular buffer**: Automatically removes old messages when capacity is reached
- **Structured data**: Each message includes timestamp, level, target, and content

### Event Handling

Push-to-talk functionality uses multiple event handlers for robust interaction:

```rust
on:mousedown=move |_| start_recording()
on:mouseup=move |_| stop_recording()
on:mouseleave=move |_| stop_recording()  // Handles mouse leaving button area
on:touchstart=move |_| start_recording()
on:touchend=move |_| stop_recording()
```

## Development Patterns

### Adding New Debug Features

1. **Backend Command**:

   ```rust
   #[cfg(debug_assertions)]
   #[tauri::command]
   async fn debug_your_feature() -> Result<String, AppError> {
       add_debug_log(DebugLogLevel::Info, "your-component", "Feature tested");
       // Your implementation
       Ok("Success message".to_string())
   }
   ```

2. **Frontend Integration**:

   ```rust
   impl DebugManager {
       pub async fn test_your_feature() -> Result<String, String> {
           tauri_invoke_no_args("debug_your_feature")
               .await
               .map_err(|e| format!("Failed to test feature: {e}"))
       }
   }
   ```

3. **UI Component**:

   ```rust
   <button
       class="debug-btn-primary"
       on:click=move |_| test_your_feature()
   >
       "Test Your Feature"
   </button>
   ```

4. **Register Command**:

   ```rust
   // Add to debug build handler list
   debug_your_feature,
   ```

### Logging Best Practices

1. **Use appropriate log levels**:
   - `Trace`: Detailed execution flow
   - `Debug`: Development information
   - `Info`: General information
   - `Warn`: Potential issues
   - `Error`: Actual errors

2. **Include context**:

   ```rust
   add_debug_log(
       DebugLogLevel::Info,
       "component-name",
       &format!("Action completed with result: {}", result)
   );
   ```

3. **Target naming**:
   - Use consistent component names
   - Follow pattern: `speakr-{component}` (e.g., `speakr-core`, `speakr-tauri`)

### Testing Debug Features

Debug features should be tested like any other code:

```rust
#[test]
fn test_debug_manager_methods_exist() {
    // Compile-time test for method signatures
    let _fn: fn() -> _ = DebugManager::test_your_feature;
    assert!(true, "Debug method exists and compiles");
}
```

## Security Considerations

### Build-time Exclusion

- Debug panel code is completely removed from release builds
- No performance impact on production builds
- No security surface area in release builds

### Development-only Data

- Log messages are stored only in memory
- No persistent storage of debug information
- Automatic cleanup when application closes

### Safe Defaults

- Mock implementations prevent accidental system access
- All debug commands return safe, predictable responses
- Clear visual indicators remind developers of debug mode

## Troubleshooting

### Debug Panel Not Visible

- **Check build type**: Ensure you're running `cargo tauri dev`, not a release build
- **Look for button**: The toggle button appears in the header, not as a separate window
- **Browser cache**: If using `trunk serve`, clear browser cache and reload

### Log Messages Not Appearing

- **Click refresh**: Use the "🔄 Refresh" button to manually fetch logs
- **Check backend**: Ensure debug commands are registered in the invoke handler
- **Memory limit**: Log storage is limited to 1000 messages; older messages are automatically removed

### Push-to-Talk Not Working

- **Hold, don't click**: The button requires holding down, not just clicking
- **Check events**: Ensure mouse/touch events are properly handled
- **Visual feedback**: Look for button colour change and pulsing animation during recording

## Future Enhancements

Potential additions to the debug panel:

1. **Performance Monitoring**: CPU, memory usage graphs
2. **Network Activity**: Mock API call testing
3. **State Inspection**: Real-time application state viewer
4. **Configuration Testing**: Dynamic settings modification
5. **Export Functionality**: Save debug logs to file
6. **Remote Debugging**: WebSocket connection for external debugging tools

## Related Files

- **Frontend**: `speakr-ui/src/debug.rs` - Main debug panel implementation
- **Backend**: `speakr-tauri/src/lib.rs` - Debug commands and log storage
- **Styles**: `speakr-ui/styles.css` - Debug panel CSS styles
- **Types**: Log message types and enums
- **Tests**: Unit tests for debug functionality

## Contributing

When adding debug features:

1. Follow the established patterns for conditional compilation
2. Add appropriate logging with meaningful messages
3. Include tests for new functionality
4. Update this documentation with new features
5. Ensure features work in both desktop and mobile layouts

The debug panel is a powerful development tool that should enhance the development experience while maintaining security and performance in production builds.
