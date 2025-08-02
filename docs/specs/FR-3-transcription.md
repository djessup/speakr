# FR-3: Transcription

Offline transcription of recorded audio to text using Whisper.

## Requirement

1. Use `whisper-rs` to run Whisper (GGUF) models entirely on-device.
2. Default language: English (en). Allow user language selection in Settings.
3. Transcription must complete within **≤ 3 s** (95th percentile) for 5-second recordings on
   Apple Silicon with the small model.
4. Support user-selectable model sizes for latency/accuracy trade-off.
5. No external network calls during transcription.

## Technical Implementation

### Type System (speakr-types)

**Core transcription types implemented:**

```rust
// Performance optimisation modes
pub enum PerformanceMode {
    Speed,      // Prioritises fastest processing
    Balanced,   // Default - balanced speed/accuracy
    Accuracy,   // Prioritises highest accuracy
}

// Complete transcription configuration
pub struct TranscriptionConfig {
    pub model_size: ModelSize,
    pub language: Option<String>,         // ISO 639-1 format
    pub auto_detect_language: bool,
    pub performance_mode: PerformanceMode,
}

// Comprehensive error handling
pub enum TranscriptionError {
    ModelNotFound { model_size: ModelSize },
    ModelLoadingFailed(String),
    ProcessingFailed(String),
    InsufficientMemory { model_size: ModelSize },
    InvalidAudioFormat(String),
    UnsupportedLanguage { language: String },
    DownloadFailed(String),
}

// Rich result with timing and confidence data
pub struct TranscriptionResult {
    pub text: String,
    pub language: Option<String>,
    pub confidence: f32,
    pub processing_time: Duration,
    pub model_used: ModelSize,
    pub segments: Vec<TranscriptionSegment>,
}
```

### Module Structure (speakr-core)

The transcription functionality is organised into focused submodules:

- `speakr-core/src/transcription/engine.rs` - Core transcription engine
- `speakr-core/src/transcription/models.rs` - Model management and loading
- `speakr-core/src/transcription/language.rs` - Language detection and handling
- `speakr-core/src/transcription/performance.rs` - Performance monitoring and metrics

## Rationale

On-device inference preserves privacy and removes network latency, achieving the product’s
privacy-first promise.

## Acceptance Criteria

- [ ] Transcription completes within latency budget on M1 and Intel reference machines.
- [ ] Selecting a different model in Settings updates the engine without restart.
- [ ] No outbound network traffic observed via packet capture.
- [ ] Errors (e.g. model missing) surface in UI overlay/log with actionable message.

## Test-Driven Design

Begin with failing automated tests for latency, language selection, and network isolation.
Implement transcription until all tests pass, following TDD.

## References

PRD §6 Functional Requirements – FR-3
