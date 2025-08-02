//! Core transcription engine functionality.
//!
//! This module provides the main transcription engine that coordinates
//! the conversion of audio samples to text using Whisper models.
//!
//! At this stage of the project we purposefully **avoid linking** to an
//! actual Whisper inference backend (e.g. `whisper-rs`) because that
//! would dramatically increase compile-times and complicate CI.  Instead
//! we focus on:
//!
//! 1.  Verifying that the *selected* model file is present on disk.
//! 2.  Providing a stable, ergonomic API for higher-level callers.
//! 3.  Returning meaningful error variants from `speakr_types` so that
//!     the UI and Tauri backend can react appropriately.
//!
//! The real inference implementation will be plugged in once the rest of
//! the pipeline and settings integration have stabilised.

use std::path::PathBuf;
use std::time::Duration;

use crate::{model::Model, transcription::models::ModelManager};
use speakr_types::{ModelSize, TranscriptionConfig, TranscriptionError, TranscriptionResult};

/// Map a high-level [`speakr_types::ModelSize`] to an *actual* model file
/// variant defined in [`crate::model::Model`].  For the MVP we pick a
/// single representative for every size bucket.
fn map_size_to_model(size: &ModelSize) -> Model {
    match size {
        ModelSize::Small => Model::Small,
        ModelSize::Medium => Model::Medium,
        ModelSize::Large => Model::LargeV3Turbo,
    }
}

/// The main transcription engine – responsible for loading a Whisper model
/// and converting raw PCM samples (`i16`, 16-kHz mono) into text.
#[derive(Debug)]
pub struct TranscriptionEngine {
    config: TranscriptionConfig,
    model_manager: ModelManager,
    active_model: Model,
}

impl TranscriptionEngine {
    /// Create a new engine with [`TranscriptionConfig::default`].  Uses the
    /// default model cache directory.
    pub fn new() -> Result<Self, TranscriptionError> {
        Self::with_config(TranscriptionConfig::default())
    }

    /// Create a new engine with a *custom* configuration.  Uses the default
    /// model cache directory.
    pub fn with_config(config: TranscriptionConfig) -> Result<Self, TranscriptionError> {
        let manager = ModelManager::new();
        Self::with_config_and_manager(config, manager)
    }

    /// Low-level constructor that allows injection of a custom
    /// [`ModelManager`] – handy for tests.
    pub fn with_config_and_manager(
        config: TranscriptionConfig,
        model_manager: ModelManager,
    ) -> Result<Self, TranscriptionError> {
        let model = map_size_to_model(&config.model_size);
        ensure_model_available(&model_manager, &model, &config.model_size)?;

        Ok(Self {
            config,
            model_manager,
            active_model: model,
        })
    }

    /// Return a reference to the engine's active configuration.
    pub fn config(&self) -> &TranscriptionConfig {
        &self.config
    }

    /// Replace the *active* model **at runtime**.  The engine will **not**
    /// download models automatically – higher layers should do so via the
    /// [`ModelManager`] API exposed in the settings UI.
    pub fn switch_model(&mut self, new_size: ModelSize) -> Result<(), TranscriptionError> {
        let new_model = map_size_to_model(&new_size);
        ensure_model_available(&self.model_manager, &new_model, &new_size)?;

        self.active_model = new_model;
        self.config.model_size = new_size;
        Ok(())
    }

    /// Update the language preference (or clear it to enable auto-detection).
    pub fn set_language(&mut self, language: Option<String>) {
        self.config.language = language;
    }

    /// Perform a *blocking* transcription of raw `i16` samples (16-kHz mono).
    ///
    /// **Note:** This is currently a *stub* implementation that returns a
    /// placeholder result so that downstream components can be integrated and
    /// unit-tested without the heavy Whisper dependency.
    pub fn transcribe(&self, _samples: &[i16]) -> Result<TranscriptionResult, TranscriptionError> {
        Ok(TranscriptionResult {
            text: "<stub – transcription engine not yet wired to whisper-rs>".to_string(),
            language: self.config.language.clone(),
            confidence: 0.0,
            processing_time: Duration::from_millis(0),
            model_used: self.config.model_size.clone(),
            segments: vec![],
        })
    }
}

impl Default for TranscriptionEngine {
    fn default() -> Self {
        Self::new().expect("default engine should initialise if default model is present")
    }
}

// -----------------------------------------------------------------------------
// Helper functions
// -----------------------------------------------------------------------------

/// Ensure that the given `model` is present in `manager.cache_dir()` – if the
/// file is missing we eagerly return a [`TranscriptionError::ModelNotFound`]
/// so that callers can trigger a download or show a helpful message.
fn ensure_model_available(
    manager: &ModelManager,
    model: &Model,
    size: &ModelSize,
) -> Result<(), TranscriptionError> {
    let filename = format!("ggml-{}.bin", model.filename());
    let path: PathBuf = manager.cache_dir().join(filename);

    if !path.exists() {
        return Err(TranscriptionError::ModelNotFound {
            model_size: size.clone(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn dummy_model_file(dir: &TempDir, model: &Model) -> PathBuf {
        let filename = format!("ggml-{}.bin", model.filename());
        let path = dir.path().join(filename);
        // create zero-byte placeholder
        fs::write(&path, []).expect("unable to create dummy model file");
        path
    }

    #[test]
    fn engine_initialises_when_model_exists() {
        let tmp = TempDir::new().unwrap();
        let model = Model::Small;
        dummy_model_file(&tmp, &model);

        let manager = ModelManager::with_cache_dir(tmp.path().to_path_buf());
        let cfg = TranscriptionConfig {
            model_size: ModelSize::Small,
            ..Default::default()
        };

        let engine = TranscriptionEngine::with_config_and_manager(cfg, manager);
        assert!(engine.is_ok());
    }

    #[test]
    fn engine_fails_when_model_missing() {
        let tmp = TempDir::new().unwrap();
        let manager = ModelManager::with_cache_dir(tmp.path().to_path_buf());
        let cfg = TranscriptionConfig {
            model_size: ModelSize::Medium,
            ..Default::default()
        };

        let engine = TranscriptionEngine::with_config_and_manager(cfg, manager);
        assert!(matches!(
            engine,
            Err(TranscriptionError::ModelNotFound { .. })
        ));
    }

    #[test]
    fn can_switch_models_at_runtime() {
        let tmp = TempDir::new().unwrap();
        // create dummy small & medium models
        dummy_model_file(&tmp, &Model::Small);
        dummy_model_file(&tmp, &Model::Medium);

        let manager = ModelManager::with_cache_dir(tmp.path().to_path_buf());
        let cfg = TranscriptionConfig {
            model_size: ModelSize::Small,
            ..Default::default()
        };
        let mut engine =
            TranscriptionEngine::with_config_and_manager(cfg, manager).expect("engine init");

        assert_eq!(engine.config().model_size, ModelSize::Small);
        engine.switch_model(ModelSize::Medium).expect("switch ok");
        assert_eq!(engine.config().model_size, ModelSize::Medium);
    }
}
