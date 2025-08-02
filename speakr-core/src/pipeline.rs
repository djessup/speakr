// =============================================================================
//! Pipeline orchestration helpers
//!
//! This module fulfils **task 6.1 – *Create pipeline integration for audio
//! samples*** of the FR-3 transcription feature.  It provides a thin, async
//! wrapper around the [`transcription::engine::TranscriptionEngine`] that
//! validates raw audio samples coming from [`audio::AudioRecorder`] and forwards
//! them to the engine **without copying**.
//!
//! # Guarantees
//!
//! 1. **Audio format validation** – Ensures that the provided samples are
//!    non-empty and compatible with the fixed *16 kHz, mono, i16* format
//!    required by Whisper.
//! 2. **Zero-copy forwarding** – The incoming `Vec<i16>` is *moved* into the
//!    background task that performs transcription, avoiding any extra
//!    allocations or copies.
//!
//! # Usage
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use speakr_core::{audio, pipeline, transcription};
//! use speakr_types::TranscriptionConfig;
//!
//! // 1. Capture audio samples using `AudioRecorder` (omitted for brevity)
//! let samples: Vec<i16> = vec![0; audio::SAMPLE_RATE_HZ as usize * 3]; // 3 s stub
//!
//! // 2. Transcribe
//! let mut cfg = TranscriptionConfig::default();
//! cfg.model_size = speakr_types::ModelSize::Small;
//! let result = pipeline::transcription_pipeline(samples, cfg).await.unwrap();
//! println!("{}", result.text);
//! # });
//! ```
// =============================================================================

use crate::{audio, transcription};
use speakr_types::{TranscriptionConfig, TranscriptionError, TranscriptionResult};
use tracing::instrument;

/// Validate that the provided samples conform to the *16 kHz mono i16* format.
///
/// Currently this merely checks that the buffer is **non-empty** and that the
/// length is a multiple of the channel count (which is `1`).  Higher-level
/// components (`AudioRecorder`) already guarantee sample-rate and bit-depth.
fn validate_audio_format(samples: &[i16]) -> Result<(), TranscriptionError> {
    if samples.is_empty() {
        return Err(TranscriptionError::InvalidAudioFormat(
            "Empty audio sample buffer".into(),
        ));
    }

    // Redundant with CHANNELS = 1 but kept for completeness / future-proofing.
    if samples.len() % (audio::CHANNELS as usize) != 0 {
        return Err(TranscriptionError::InvalidAudioFormat(format!(
            "Sample count ({}) is not divisible by channel count",
            samples.len()
        )));
    }

    Ok(())
}

/// High-level helper that converts raw audio samples to text.
///
/// * **Input** – A `Vec<i16>` containing *16 kHz mono* PCM samples recorded by
///   [`audio::AudioRecorder`].  Ownership of the vector is taken to avoid
///   copying.
/// * **Output** – A [`TranscriptionResult`] returned by the Whisper engine.
///
/// # Errors
///
/// Returns [`TranscriptionError::InvalidAudioFormat`] when sample validation
/// fails or any error propagated by the underlying transcription engine.
#[instrument(level = "debug", skip(samples))]
pub async fn transcription_pipeline(
    samples: Vec<i16>,
    config: TranscriptionConfig,
) -> Result<TranscriptionResult, TranscriptionError> {
    // 1. Audio format validation -------------------------------------------------------------
    validate_audio_format(&samples)?;

    // 2. Initialises a new engine using the provided configuration ---------------------------
    let engine = transcription::engine::TranscriptionEngine::with_config(config)?;

    // 3. Forward samples **by move** into the background task (zero-copy) --------------------
    engine.transcribe_async(samples).await
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    use std::fs;
    use tempfile::TempDir;

    // Helper to create a dummy model file required by the engine initialisation
    fn create_dummy_model(dir: &TempDir, model: &Model) {
        let filename = format!("ggml-{}.bin", model.filename());
        let path = dir.path().join(filename);
        fs::write(path, []).expect("unable to create dummy model file");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn pipeline_accepts_valid_samples() {
        // ---------------------------------------------------------------------
        // Arrange
        let tmp = TempDir::new().unwrap();
        std::env::set_var("SPEAKR_MODELS_DIR", tmp.path());

        let model = Model::Small;
        create_dummy_model(&tmp, &model);

        let samples = vec![0i16; audio::SAMPLE_RATE_HZ as usize]; // 1 second of silence
        let cfg = speakr_types::TranscriptionConfig {
            model_size: speakr_types::ModelSize::Small,
            ..Default::default()
        };

        // ---------------------------------------------------------------------
        // Act
        let result = transcription_pipeline(samples, cfg).await;
        if let Err(ref e) = result {
            println!("Pipeline error: {e:?}");
        }
        // ---------------------------------------------------------------------
        // Assert
        assert!(result.is_ok());
        let text = &result.unwrap().text;
        assert!(text.contains("stub"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn pipeline_rejects_empty_buffer() {
        let cfg = speakr_types::TranscriptionConfig {
            model_size: speakr_types::ModelSize::Small,
            ..Default::default()
        };
        let err = transcription_pipeline(Vec::new(), cfg).await.unwrap_err();
        assert!(matches!(err, TranscriptionError::InvalidAudioFormat(_)));
    }
}
