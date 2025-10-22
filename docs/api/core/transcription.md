# Transcription Engine

Configurable engine that loads a Whisper model and converts PCM samples into text. Current inference is a stub until `whisper-rs` integration.

## Types and Methods

- `TranscriptionEngine::new() -> Result<Self, TranscriptionError>`
- `TranscriptionEngine::with_config(cfg: TranscriptionConfig) -> Result<Self, TranscriptionError>`
- `TranscriptionEngine::with_config_and_manager(cfg, manager)` – inject custom `ModelManager`
- `config(&self) -> &TranscriptionConfig`
- `switch_model(&mut self, size: ModelSize)`
- `set_language(&mut self, language: Option<String>)`
- `set_performance_mode(&mut self, mode: PerformanceMode)`
- `transcribe(&self, samples: &[i16]) -> Result<TranscriptionResult, TranscriptionError>`
- `transcribe_async(&self, samples: Vec<i16>) -> Result<TranscriptionResult, TranscriptionError>`

## Behaviour

- Verifies model availability; may downgrade size if memory budget is insufficient.
- Returns timings and memory deltas in `TranscriptionResult`.

## Example

```rust
use speakr_core::transcription::engine::TranscriptionEngine;
use speakr_types::{TranscriptionConfig, ModelSize};

let cfg = TranscriptionConfig { model_size: ModelSize::Small, ..Default::default() };
let engine = TranscriptionEngine::with_config(cfg).expect("engine init");
let samples = vec![0_i16; 16_000]; // 1s silence
let result = tokio_test::block_on(engine.transcribe_async(samples)).unwrap();
println!("{}", result.text);
```
