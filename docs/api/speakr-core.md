# Core Library (`speakr-core`)

Audio capture, model management, transcription engine, and pipeline helpers.

- Audio: microphone access, in-memory PCM samples, duration-limited recording
- Model: Whisper model catalogue and cache/download manager
- Transcription: configurable engine and async API
- Pipeline: validate samples and run transcription end-to-end

See detailed subpages:

- Audio – `api/core/audio.md`
- Transcription Engine – `api/core/transcription.md`
- Model Management – `api/core/model.md`
- Pipeline – `api/core/pipeline.md`

Quick start:

```rust
use speakr_core::audio::{AudioRecorder, RecordingConfig};
use speakr_types::{TranscriptionConfig, ModelSize};

# async fn demo() -> Result<(), Box<dyn std::error::Error>> {
let rec = AudioRecorder::new(RecordingConfig::new(5)).await?;
rec.start_recording().await?; // ...wait or listen for hot-key release
let samples = rec.stop_recording().await?.samples();

let cfg = TranscriptionConfig { model_size: ModelSize::Small, ..Default::default() };
let result = speakr_core::pipeline::transcription_pipeline(samples, cfg).await?;
println!("{}", result.text);
# Ok(()) }
```
