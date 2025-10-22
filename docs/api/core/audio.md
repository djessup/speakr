# Audio Module

Low-level microphone capture and recording lifecycle utilities. All samples are 16 kHz mono `i16` and remain in-memory.

## Constants

- `SAMPLE_RATE_HZ: 16_000`
- `CHANNELS: 1`
- `DEFAULT_MAX_DURATION_SECS: 10`
- `MAX_ALLOWED_DURATION_SECS: 30`

## Public Types

- `AudioDevice { id: String, name: String, is_default: bool }`
- `AudioCaptureError` – rich error enum for microphone, device, stream, permission and config issues
- `RecordingConfig::new(max_secs)`, `max_duration_secs()`, `max_samples()`
- `RecordingResult::{Success, StoppedAtLimit, ManuallyStoppedEarly}.samples()`
- `AudioSettings::new()`, `default_config()`
- Traits: `AudioSystem`, `AudioStream`
- Implementations: `CpalAudioSystem`, `CpalAudioStream`
- High-level: `AudioRecorder::new(...)`, `start_recording()`, `stop_recording()`, `is_recording()`, `list_input_devices()`

## Usage

```rust
use speakr_core::audio::{AudioRecorder, RecordingConfig};
# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let recorder = AudioRecorder::new(RecordingConfig::new(3)).await?;
recorder.start_recording().await?;
// ... wait for user to finish speaking ...
let samples = recorder.stop_recording().await?.samples();
assert!(!samples.is_empty());
# Ok(()) }
```

List devices:

```rust
use speakr_core::audio::AudioRecorder;
# async fn devices() -> Result<(), Box<dyn std::error::Error>> {
let recorder = AudioRecorder::with_audio_system(Box::new(speakr_core::audio::CpalAudioSystem::new()?));
let inputs = recorder.list_input_devices().await?;
for d in inputs { println!("{} {}{}", d.id, d.name, if d.is_default { " (default)" } else { "" }); }
# Ok(()) }
```
