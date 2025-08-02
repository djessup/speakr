// =========================================================================
//! Whisper model management – downloading, caching and integrity checks.
//!
//! This module fulfils **task&nbsp;2.1 ― _Create ModelManager for model
//! downloading and caching_** (see `docs/specs/FR-3-transcription.md`).
//!
//! The public `ModelManager` API is intentionally small for now – we only need
//! the following capabilities:
//!
//! 1. Determine (and create) the local *model cache directory*.
//! 2. **Download** a Whisper GGUF model from a HuggingFace URL (or any HTTPS
//!    URL).
//! 3. **Validate integrity** of the downloaded file using a SHA-256 checksum.
//!
//! All potentially expensive work is asynchronous and non-blocking.  Future
//! iterations will extend this manager with on-disk metadata, concurrent
//! downloads, progress reporting, etc.
// =========================================================================

use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use sha2::{Digest, Sha256};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Errors returned by [`ModelManager`].
#[derive(Debug, Error)]
pub enum ModelManagerError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("checksum mismatch – expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
}

/// Manages local Whisper GGUF models.
///
/// The manager keeps track of a *cache directory* under the user's
///   `~/Library/Application Support/Speakr/models` (macOS) or the platform
/// equivalent.  A different directory can be supplied via
/// [`ModelManager::with_cache_dir`], which is mainly useful for testing.
#[derive(Debug, Clone)]
pub struct ModelManager {
    cache_dir: PathBuf,
}

impl ModelManager {
    /// Create a new [`ModelManager`] using the default cache directory.
    pub fn new() -> Self {
        let cache_dir = Self::default_cache_dir();
        Self { cache_dir }
    }

    /// Create a new [`ModelManager`] with a custom cache directory – *tests only*.
    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    /// Return the directory where models are cached locally.
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Ensure that [`ModelManager::cache_dir`] exists on disk.
    async fn ensure_cache_dir(&self) -> Result<(), std::io::Error> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir).await?
        }
        Ok(())
    }

    /// Download a model from `url` if it is missing or the checksum does not
    /// match `expected_sha256`.
    ///
    /// On success returns the absolute path of the cached model file.
    pub async fn download_model(
        &self,
        url: &str,
        expected_sha256: Option<&str>,
    ) -> Result<PathBuf, ModelManagerError> {
        // 1. Prepare cache directory ----------------------------------------------------------
        self.ensure_cache_dir().await?;

        // 2. Derive filename from URL ---------------------------------------------------------
        let filename = url
            .rsplit('/')
            .next()
            .ok_or_else(|| ModelManagerError::InvalidUrl(url.to_string()))?;
        let dest_path = self.cache_dir.join(filename);

        // 3. If the file already exists and (optionally) matches the checksum, short-circuit.
        if dest_path.exists() {
            if let Some(expected) = expected_sha256 {
                if Self::verify_checksum(&dest_path, expected).await? {
                    return Ok(dest_path);
                }
            } else {
                return Ok(dest_path);
            }
        }

        // 4. Download the file ---------------------------------------------------------------
        let bytes = reqwest::get(url).await?.error_for_status()?.bytes().await?;

        // 5. Checksum validation -------------------------------------------------------------
        if let Some(expected) = expected_sha256 {
            let actual = hex::encode(Sha256::digest(&bytes));
            if !actual.eq_ignore_ascii_case(expected) {
                return Err(ModelManagerError::ChecksumMismatch {
                    expected: expected.to_string(),
                    actual,
                });
            }
        }

        // 6. Persist to disk (atomically via a tmp file, then rename) ------------------------
        let tmp_path = dest_path.with_extension("tmp");
        let mut tmp_file = fs::File::create(&tmp_path).await?;
        tmp_file.write_all(&bytes).await?;
        tmp_file.flush().await?;
        drop(tmp_file); // close handle before rename
        fs::rename(&tmp_path, &dest_path).await?;

        Ok(dest_path)
    }

    // -------------------------------------------------------------------------
    // Helper functions
    // -------------------------------------------------------------------------

    /// Default cache directory (platform specific).
    fn default_cache_dir() -> PathBuf {
        // Use $SPEAKR_MODELS_DIR if explicitly set – handy for tests.
        if let Ok(dir) = std::env::var("SPEAKR_MODELS_DIR") {
            return PathBuf::from(dir);
        }

        if let Some(dirs) = ProjectDirs::from("com", "Speakr", "Speakr") {
            return dirs.data_local_dir().join("models");
        }

        // Fallback to `./models` inside the current working directory.
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("models")
    }

    /// Verify that the SHA-256 checksum of `path` matches `expected`.
    async fn verify_checksum(path: &Path, expected: &str) -> Result<bool, std::io::Error> {
        let bytes = fs::read(path).await?;
        let actual = hex::encode(Sha256::digest(&bytes));
        Ok(actual.eq_ignore_ascii_case(expected))
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}
