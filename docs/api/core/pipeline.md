# Pipeline

High-level helper that validates raw audio samples and runs the transcription engine asynchronously with zero-copy forwarding.

## Function

```rust
pub async fn transcription_pipeline(samples: Vec<i16>, config: TranscriptionConfig)
  -> Result<TranscriptionResult, TranscriptionError>
```

- Validates non-empty buffer and channel count.
- Instantiates `TranscriptionEngine` using `config`.
- Moves ownership of `samples` into background work.

## Example

```rust
use speakr_types::{TranscriptionConfig, ModelSize};
# async fn transcribe(samples: Vec<i16>) -> Result<String, Box<dyn std::error::Error>> {
let cfg = TranscriptionConfig { model_size: ModelSize::Small, ..Default::default() };
let result = speakr_core::pipeline::transcription_pipeline(samples, cfg).await?;
Ok(result.text)
# }
```
