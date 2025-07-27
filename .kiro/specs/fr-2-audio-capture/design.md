# Design Document

## Overview

The Audio Capture feature provides high-quality microphone input recording optimized for Whisper
transcription. Built around the `cpal` crate, it implements a trait-based architecture that enables
dependency injection for testing while maintaining robust error handling and performance
requirements. The system captures 16 kHz mono audio entirely in memory with configurable duration
limits.

## Architecture

The audio capture system follows a layered architecture with clear separation between the public
API, trait abstractions, and platform-specific implementations:

- **AudioRecorder**: High-level API that orchestrates recording lifecycle and timeout management
- **AudioSystem Trait**: Abstraction layer enabling testing and future platform implementations
- **CpalAudioSystem**: Concrete implementation using the `cpal` crate for cross-platform audio
- **AudioStream Trait**: Abstraction for active recording streams
- **CpalAudioStream**: Platform-specific stream implementation with thread-safe sample collection

### Component Interaction

```mermaid
graph TB
    subgraph "Public API"
        AR[AudioRecorder]
        RC[RecordingConfig]
    end

    subgraph "Trait Layer"
        AS[AudioSystem trait]
        AST[AudioStream trait]
    end

    subgraph "cpal Implementation"
        CAS[CpalAudioSystem]
        CAST[CpalAudioStream]
        CPAL[cpal crate]
    end

    subgraph "System"
        OS[Operating System]
        MIC[Microphone Hardware]
    end

    AR --> AS
    AR --> RC
    AS --> AST
    CAS ..|> AS
    CAST ..|> AST
    CAS --> CAST
    CAST --> CPAL
    CPAL --> OS
    OS --> MIC
```

## Components and Interfaces

### AudioRecorder

**Location**: `speakr-core/src/audio/mod.rs`

**Responsibilities**:

- Provide high-level recording API with lifecycle management
- Handle timeout management using tokio tasks
- Prevent concurrent recordings
- Convert between different sample formats to 16-bit signed integers

**Key Methods**:

```rust
impl AudioRecorder {
    pub async fn new(config: RecordingConfig) -> Result<Self, AudioCaptureError>
    pub fn with_audio_system(audio_system: Box<dyn AudioSystem>) -> Self
    pub async fn start_recording(&self) -> Result<(), AudioCaptureError>
    pub async fn stop_recording(&self) -> Result<RecordingResult, AudioCaptureError>
    pub fn is_recording(&self) -> bool
    pub async fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioCaptureError>
}
```

### AudioSystem Trait

**Location**: `speakr-core/src/audio/mod.rs`

**Purpose**: Abstraction layer enabling dependency injection and testing

**Interface**:

```rust
pub trait AudioSystem: Send + Sync {
    fn start_recording(&self, config: &RecordingConfig) -> Result<Box<dyn AudioStream>, AudioCaptureError>;
    fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioCaptureError>;
}
```

### CpalAudioSystem

**Location**: `speakr-core/src/audio/mod.rs`

**Responsibilities**:

- Interface with the `cpal` crate for cross-platform audio access
- Handle device enumeration and default device selection
- Create and configure audio streams with proper sample rate conversion

### RecordingConfig

**Location**: `speakr-core/src/audio/mod.rs`

**Structure**:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct RecordingConfig {
    max_duration_secs: u32,
}

impl RecordingConfig {
    pub fn new(duration_secs: u32) -> Self
    pub fn max_duration_secs(&self) -> u32
    pub fn max_samples(&self) -> usize
}
```

## Data Models

### AudioCaptureError

```rust
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AudioCaptureError {
    #[error("Microphone not available")]
    MicrophoneNotAvailable,

    #[error("Microphone permission denied")]
    PermissionDenied,

    #[error("Audio device error: {0}")]
    DeviceError(String),

    #[error("Audio stream error: {0}")]
    StreamError(String),

    #[error("Recording is already in progress")]
    RecordingInProgress,

    #[error("No recording is active")]
    NoActiveRecording,

    #[error("Recording initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Invalid recording configuration: {0}")]
    InvalidConfiguration(String),
}
```

### RecordingResult

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct RecordingResult {
    pub samples: Vec<i16>,
    pub duration: Duration,
    pub sample_rate: u32,
    pub channels: u16,
}
```

### AudioDevice

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}
```

## Error Handling

### Device Access Failures

1. **Microphone Not Available**: When no audio input devices are detected

   - Return `AudioCaptureError::MicrophoneNotAvailable`
   - Allow graceful degradation in calling code

2. **Permission Denied**: When system denies microphone access

   - Return `AudioCaptureError::PermissionDenied`
   - Enable UI to guide user through permission granting

3. **Device Errors**: When specific audio devices fail
   - Wrap device-specific errors in `AudioCaptureError::DeviceError`
   - Provide detailed error context for debugging

### Stream Management Failures

1. **Stream Creation**: When audio stream cannot be initialized

   - Return `AudioCaptureError::InitializationFailed`
   - Include underlying cpal error details

2. **Concurrent Recording Prevention**: When recording is already active

   - Return `AudioCaptureError::RecordingInProgress`
   - Maintain single active recording constraint

3. **Sample Format Conversion**: When audio formats need conversion
   - Handle F32, I16, U16 formats automatically
   - Convert all formats to target 16-bit signed integers

## Testing Strategy

### Unit Tests with Mock Objects

The trait-based architecture enables comprehensive testing without hardware:

```rust
struct MockAudioSystem {
    should_fail: bool,
    mock_samples: Vec<i16>,
}

impl AudioSystem for MockAudioSystem {
    fn start_recording(&self, config: &RecordingConfig) -> Result<Box<dyn AudioStream>, AudioCaptureError> {
        // Mock implementation for testing
    }
}
```

### Test Categories

1. **Configuration Tests**: Validate recording duration limits and sample calculations
2. **Lifecycle Tests**: Test start/stop recording sequences and state management
3. **Error Handling Tests**: Mock various failure scenarios and verify error responses
4. **Performance Tests**: Measure initialization time (target: <100ms)
5. **Integration Tests**: Test with real hardware when available

### Test Isolation

- Use `MockAudioSystem` for unit tests to avoid hardware dependencies
- Implement conditional integration tests that check for hardware availability
- Use atomic operations and proper synchronization for thread-safe testing

## Performance Considerations

### Memory Management

- Audio samples stored in `Arc<Mutex<Vec<i16>>>` for thread-safe access
- Automatic buffer cleanup when recording completes
- No temporary file creation - all processing in memory

### Timeout Handling

- Use tokio tasks for non-blocking timeout management
- Atomic boolean flags for clean stream termination
- Proper cleanup of resources on timeout or manual stop

### Sample Rate Conversion

- Target 16 kHz sample rate for Whisper compatibility
- Automatic conversion from various input formats (F32, I16, U16)
- Efficient sample format conversion without quality loss

## Integration Points

### Settings Integration

The audio system needs integration with the settings persistence layer:

- Recording duration should be user-configurable through settings UI
- Duration preference should persist across application restarts
- Default values should be used when settings are unavailable

### Hotkey Integration

Audio capture is triggered by the global hotkey system:

- Hotkey press initiates `start_recording()`
- Second hotkey press during recording calls `stop_recording()`
- Automatic timeout handling when duration limit is reached

### Transcription Pipeline

Audio samples are passed to the transcription system:

- `RecordingResult.samples` provides `Vec<i16>` for Whisper processing
- Sample rate and format guaranteed to match Whisper requirements
- Memory-efficient transfer without additional copying
