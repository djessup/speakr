//! Core transcription engine functionality.
//!
//! This module provides the main transcription engine that coordinates the
//! conversion of audio samples to text using Whisper models.  The *actual*
//! inference step is intentionally stubbed (until `whisper-rs` is integrated)
//! so we can focus on ergonomics, performance instrumentation, and settings
//! integration.

use std::path::PathBuf;
use std::time::Instant;

use crate::{model::Model, transcription::models::ModelManager};
use speakr_types::{
    ModelSize, PerformanceMode, TranscriptionConfig, TranscriptionError, TranscriptionResult,
};
use sysinfo::System;
use tokio::task;

/// Map a high-level [`ModelSize`] to a concrete [`Model`] file.
fn map_size_to_model(size: &ModelSize) -> Model {
    match size {
        ModelSize::Small => Model::Small,
        ModelSize::Medium => Model::Medium,
        ModelSize::Large => Model::LargeV3Turbo,
    }
}

/// The main transcription engine – responsible for loading a Whisper model and
/// converting raw PCM samples (`i16`, 16-kHz mono) into text.
#[derive(Debug, Clone)]
pub struct TranscriptionEngine {
    config: TranscriptionConfig,
    model_manager: ModelManager,
    active_model: Model,
}

impl TranscriptionEngine {
    /// Create a new engine using [`TranscriptionConfig::default`].
    pub fn new() -> Result<Self, TranscriptionError> {
        Self::with_config(TranscriptionConfig::default())
    }

    /// Create a new engine with a *custom* configuration.
    pub fn with_config(config: TranscriptionConfig) -> Result<Self, TranscriptionError> {
        let manager = ModelManager::new();
        Self::with_config_and_manager(config, manager)
    }

    /// Low-level constructor that allows injection of a custom [`ModelManager`].
    pub fn with_config_and_manager(
        config: TranscriptionConfig,
        model_manager: ModelManager,
    ) -> Result<Self, TranscriptionError> {
        use tracing::{error, warn};

        let mut cfg = config;
        let mut model = map_size_to_model(&cfg.model_size);

        // 1. Ensure the model file is present – log but continue, we may fall back.
        if let Err(e) = ensure_model_available(&model_manager, &model, &cfg.model_size) {
            error!(?e, "Primary model not available – attempting fallback");
        }

        // 2. Check memory budget and downgrade model size if required.
        let sys = System::new_all();
        let total_mb = ((sys.total_memory() + sys.total_swap()) / 1024) as u32;
        let budget_mb = ((total_mb as f32) * 0.75) as u32; // leave 25% headroom

        if model.memory_usage_mb() > budget_mb {
            warn!(
                model_size = ?cfg.model_size,
                required_mb = model.memory_usage_mb(),
                budget_mb,
                "Model exceeds memory budget – falling back to smaller size"
            );

            cfg.model_size = match cfg.model_size {
                ModelSize::Large => ModelSize::Medium,
                ModelSize::Medium => ModelSize::Small,
                ModelSize::Small => {
                    return Err(TranscriptionError::InsufficientMemory {
                        model_size: cfg.model_size.clone(),
                    });
                }
            };
            model = map_size_to_model(&cfg.model_size);
        }

        // 3. Final availability check for the selected model.
        ensure_model_available(&model_manager, &model, &cfg.model_size)?;

        Ok(Self {
            config: cfg,
            model_manager,
            active_model: model,
        })
    }

    /// Access the active configuration.
    pub fn config(&self) -> &TranscriptionConfig {
        &self.config
    }

    /// Switch the model at runtime (no automatic downloads).
    pub fn switch_model(&mut self, new_size: ModelSize) -> Result<(), TranscriptionError> {
        let new_model = map_size_to_model(&new_size);
        ensure_model_available(&self.model_manager, &new_model, &new_size)?;

        self.active_model = new_model;
        self.config.model_size = new_size;
        Ok(())
    }

    /// Update the preferred language (or `None` for auto-detection).
    pub fn set_language(&mut self, language: Option<String>) {
        self.config.language = language;
    }

    /// Update the performance mode (speed ↔ accuracy trade-off).
    pub fn set_performance_mode(&mut self, mode: PerformanceMode) {
        self.config.performance_mode = mode;
    }

    /// *Blocking* transcription API – returns once processing is finished.
    pub fn transcribe(&self, _samples: &[i16]) -> Result<TranscriptionResult, TranscriptionError> {
        // --------------------------- Instrumentation ---------------------------
        let mut sys = System::new();
        sys.refresh_memory();
        let mem_before = sys.used_memory();
        let start = Instant::now();

        // --------------------------- Placeholder inference --------------------
        let text_stub = "<stub – transcription engine not yet wired to whisper-rs>".to_string();

        // --------------------------- Metrics ----------------------------------
        let duration = start.elapsed();
        sys.refresh_memory();
        let mem_after = sys.used_memory();
        let mem_delta_bytes = mem_after.saturating_sub(mem_before) * 1024;

        Ok(TranscriptionResult {
            text: text_stub,
            language: self.config.language.clone(),
            confidence: 0.0,
            processing_time: duration,
            memory_delta_bytes: mem_delta_bytes,
            model_used: self.config.model_size.clone(),
            segments: vec![],
        })
    }

    /// Non-blocking transcription helper – spawns work on a background thread.
    pub async fn transcribe_async(
        &self,
        samples: Vec<i16>,
    ) -> Result<TranscriptionResult, TranscriptionError> {
        let engine_clone = self.clone();
        task::spawn_blocking(move || engine_clone.transcribe(&samples))
            .await
            .map_err(|e| TranscriptionError::ProcessingFailed(e.to_string()))?
    }
}

impl Default for TranscriptionEngine {
    fn default() -> Self {
        Self::new().expect("default engine should initialise if default model is present")
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------
fn ensure_model_available(
    manager: &ModelManager,
    model: &Model,
    size: &ModelSize,
) -> Result<(), TranscriptionError> {
    let filename = format!("ggml-{}.bin", model.filename());
    let path: PathBuf = manager.cache_dir().join(filename);

    #[allow(unused_imports)]
    use tokio::runtime::Runtime;

    // 1. If the file is missing entirely, surface a ModelNotFound error (UI will guide the user).
    if !path.exists() {
        tracing::error!(missing_model=?path, "Model file not found");
        return Err(TranscriptionError::ModelNotFound {
            model_size: size.clone(),
        });
    }

    // 2. Verify checksum – treat mismatch as corruption and trigger re-download.
    #[cfg(test)]
    {
        // Skip expensive network access during unit tests – assume model is valid if present.
        Ok(())
    }
    #[cfg(not(test))]
    {
        let rt = Runtime::new().expect("tokio runtime");
        let is_valid = rt
            .block_on(async { manager.is_available(model, true).await })
            .unwrap_or(false);

        if is_valid {
            return Ok(());
        }

        // 3. Attempt automatic re-download for corrupted files (with retry).
        tracing::warn!(
            ?path,
            "Model checksum mismatch – attempting automatic re-download"
        );
        match rt.block_on(async { manager.download_model_with_retry(model, 2).await }) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!(?e, "Automatic model download failed");
                Err(TranscriptionError::DownloadFailed(e.to_string()))
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn dummy_model_file(dir: &TempDir, model: &Model) -> PathBuf {
        let filename = format!("ggml-{}.bin", model.filename());
        let path = dir.path().join(filename);
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

    #[tokio::test(flavor = "multi_thread")]
    async fn async_transcription_runs_on_background_thread() {
        let tmp = TempDir::new().unwrap();
        dummy_model_file(&tmp, &Model::Small);

        let manager = ModelManager::with_cache_dir(tmp.path().to_path_buf());
        let cfg = TranscriptionConfig {
            model_size: ModelSize::Small,
            ..Default::default()
        };
        let engine =
            TranscriptionEngine::with_config_and_manager(cfg, manager).expect("engine init");

        let samples = vec![0_i16; 16000];
        let result = engine
            .transcribe_async(samples)
            .await
            .expect("transcription");
        assert!(result.text.contains("stub"));
    }
}
