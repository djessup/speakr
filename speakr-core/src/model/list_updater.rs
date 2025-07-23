/// Model metadata updater for automating whisper.cpp model information extraction
///
/// This module provides tools to automatically extract model metadata from the
/// Hugging Face whisper.cpp repository and update the models.rs file with
/// current SHA checksums, file sizes, and git references.
use std::{fs, path::PathBuf, process::Command};
use thiserror::Error;

use super::metadata::ModelMetadata;

/// Errors that can occur during model metadata extraction
#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum ModelUpdateError {
    #[error("Git operation failed: {0}")]
    GitError(String),
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Model metadata extraction tool
pub struct ModelListUpdater {
    repo_path: PathBuf,
    repo_url: String,
    repo_name: String,
}

impl ModelListUpdater {
    /// Creates a new model updater instance with default repository
    ///
    /// # Arguments
    ///
    /// * `workspace_dir` - Directory where the models-repo will be cloned
    ///
    /// # Examples
    ///
    /// ```rust
    /// use speakr_core::model::ModelListUpdater;
    /// use tempfile::TempDir;
    ///
    /// let temp_dir = TempDir::new().unwrap();
    /// let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());
    /// ```
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self::with_repo(workspace_dir, "ggerganov/whisper.cpp")
    }

    /// Creates a new model updater instance with configurable repository
    ///
    /// # Arguments
    ///
    /// * `workspace_dir` - Directory where the models-repo will be cloned
    /// * `repo_name` - Repository name in format "owner/repo" (e.g., "ggerganov/whisper.cpp")
    ///
    /// # Examples
    ///
    /// ```rust
    /// use speakr_core::model::ModelListUpdater;
    /// use tempfile::TempDir;
    ///
    /// let temp_dir = TempDir::new().unwrap();
    /// let updater = ModelListUpdater::with_repo(temp_dir.path().to_path_buf(), "custom/whisper-fork");
    /// ```
    pub fn with_repo(workspace_dir: PathBuf, repo_name: &str) -> Self {
        let repo_url = format!("https://huggingface.co/{repo_name}");
        Self {
            repo_path: workspace_dir.join("models-repo"),
            repo_url,
            repo_name: repo_name.to_string(),
        }
    }

    /// Gets the repository name being used
    pub fn repo_name(&self) -> &str {
        &self.repo_name
    }

    /// Gets the repository URL being used
    pub fn repo_url(&self) -> &str {
        &self.repo_url
    }

    /// Clones the repository without LFS files
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError::GitError` if the git clone operation fails
    pub async fn clone_repository(&self) -> Result<(), ModelUpdateError> {
        // GREEN: Minimal implementation to pass the test

        println!("🏗️  Preparing workspace...");

        // Remove existing directory if it exists
        if self.repo_path.exists() {
            println!("   🧹 Cleaning up existing repository directory");
            fs::remove_dir_all(&self.repo_path).map_err(|e| {
                ModelUpdateError::FileSystemError(format!("Failed to remove existing repo: {e}"))
            })?;
        }

        // Create parent directory if needed
        if let Some(parent) = self.repo_path.parent() {
            println!("   📁 Creating workspace directory: {}", parent.display());
            fs::create_dir_all(parent).map_err(|e| {
                ModelUpdateError::FileSystemError(format!("Failed to create parent dir: {e}"))
            })?;
        }

        println!("📡 Cloning repository: {}", self.repo_url);
        println!("   📍 Target directory: {}", self.repo_path.display());
        println!("   ⚡ Using GIT_LFS_SKIP_SMUDGE=1 to skip large file downloads");

        // Run git clone with GIT_LFS_SKIP_SMUDGE=1
        let output = Command::new("git")
            .arg("clone")
            .arg(&self.repo_url)
            .arg(&self.repo_path)
            .env("GIT_LFS_SKIP_SMUDGE", "1")
            .output()
            .map_err(|e| {
                ModelUpdateError::GitError(format!("Failed to execute git command: {e}"))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ModelUpdateError::GitError(format!(
                "Git clone failed: {stderr}"
            )));
        }

        println!("✅ Repository cloned successfully");
        Ok(())
    }

    /// Extracts metadata for a specific model file
    ///
    /// # Arguments
    ///
    /// * `filename` - The model filename (e.g., "ggml-base-q5_1.bin")
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError` if metadata extraction fails
    pub async fn extract_model_metadata(
        &self,
        filename: &str,
    ) -> Result<ModelMetadata, ModelUpdateError> {
        // GREEN: Minimal implementation to pass the test

        println!("   🔍 Processing: {filename}");

        if !self.repo_path.exists() {
            return Err(ModelUpdateError::FileSystemError(
                "Repository not cloned yet".to_string(),
            ));
        }

        let file_path = self.repo_path.join(filename);
        if !file_path.exists() {
            return Err(ModelUpdateError::FileSystemError(format!(
                "Model file {filename} not found"
            )));
        }

        println!("      📄 Reading LFS pointer file...");
        // Read the LFS pointer file content
        let content = fs::read_to_string(&file_path).map_err(|e| {
            ModelUpdateError::FileSystemError(format!("Failed to read LFS pointer file: {e}"))
        })?;

        // Parse LFS pointer file format:
        // version https://git-lfs.github.com/spec/v1
        // oid sha256:317eb69c11673c9de1e1f0d459b253999804ec71ac4c23c17ecf5fbe24e259a1
        // size 874188075
        let mut sha256 = String::new();
        let mut size_bytes = 0u64;

        println!("      🔢 Parsing metadata from LFS pointer...");
        for line in content.lines() {
            if line.starts_with("oid sha256:") {
                sha256 = line.trim_start_matches("oid sha256:").to_string();
            } else if line.starts_with("size ") {
                let size_str = line.trim_start_matches("size ");
                size_bytes = size_str.parse().map_err(|e| {
                    ModelUpdateError::ParseError(format!("Failed to parse size: {e}"))
                })?;
            }
        }

        if sha256.is_empty() {
            return Err(ModelUpdateError::ParseError(
                "No SHA256 found in LFS pointer file".to_string(),
            ));
        }

        if size_bytes == 0 {
            return Err(ModelUpdateError::ParseError(
                "No size found in LFS pointer file".to_string(),
            ));
        }

        println!("      🏷️  Getting git reference...");
        // Get git reference for this file
        let git_ref = self.get_git_ref_for_file(filename).await?;

        // Construct download URL
        let download_url = format!(
            "https://huggingface.co/{}/resolve/{git_ref}/{filename}",
            self.repo_name
        );

        println!(
            "      ✅ Extracted: {} bytes, SHA: {}...",
            size_bytes,
            &sha256[..8]
        );

        Ok(ModelMetadata {
            filename: filename.to_string(),
            sha256,
            size_bytes,
            git_ref,
            download_url,
        })
    }

    /// Gets the git reference (commit hash) for a specific file
    ///
    /// # Arguments
    ///
    /// * `filename` - The model filename
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError::GitError` if git log operation fails
    async fn get_git_ref_for_file(&self, filename: &str) -> Result<String, ModelUpdateError> {
        let output = Command::new("git")
            .arg("log")
            .arg("-n")
            .arg("1")
            .arg("--pretty=format:%H")
            .arg("--")
            .arg(filename)
            .current_dir(&self.repo_path)
            .output()
            .map_err(|e| ModelUpdateError::GitError(format!("Failed to execute git log: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(ModelUpdateError::GitError(format!(
                "Git log failed: {stderr}"
            )));
        }

        let git_ref = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if git_ref.is_empty() {
            return Err(ModelUpdateError::GitError(
                "No git reference found for file".to_string(),
            ));
        }

        Ok(git_ref)
    }

    /// Gets all available model files from the repository
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError` if file listing fails
    pub async fn list_model_files(&self) -> Result<Vec<String>, ModelUpdateError> {
        // GREEN: Minimal implementation to pass the test

        if !self.repo_path.exists() {
            return Err(ModelUpdateError::FileSystemError(
                "Repository not cloned yet".to_string(),
            ));
        }

        println!("📋 Scanning repository for model files...");
        let read_dir = fs::read_dir(&self.repo_path).map_err(|e| {
            ModelUpdateError::FileSystemError(format!("Failed to read repo directory: {e}"))
        })?;

        let mut model_files = Vec::new();

        for entry in read_dir {
            let entry = entry.map_err(|e| {
                ModelUpdateError::FileSystemError(format!("Failed to read dir entry: {e}"))
            })?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Only include .bin files that start with "ggml-"
            if file_name_str.starts_with("ggml-") && file_name_str.ends_with(".bin") {
                model_files.push(file_name_str.to_string());
            }
        }

        model_files.sort();
        println!("   📊 Found {} model files", model_files.len());
        Ok(model_files)
    }

    /// Extracts metadata for all models in the repository
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError` if any metadata extraction fails
    pub async fn extract_all_metadata(&self) -> Result<Vec<ModelMetadata>, ModelUpdateError> {
        // GREEN: Minimal implementation to pass the test

        let model_files = self.list_model_files().await?;
        let mut all_metadata = Vec::new();

        println!("🔄 Extracting metadata for {} models...", model_files.len());

        for (i, filename) in model_files.iter().enumerate() {
            println!("   📦 [{}/{}]", i + 1, model_files.len());
            let metadata = self.extract_model_metadata(filename).await?;
            all_metadata.push(metadata);
        }

        println!("✅ Metadata extraction complete");
        Ok(all_metadata)
    }

    /// Cleans up the workspace directory
    ///
    /// # Errors
    ///
    /// Returns `ModelUpdateError::FileSystemError` if cleanup fails
    pub fn cleanup(&self) -> Result<(), ModelUpdateError> {
        if self.repo_path.exists() {
            println!("🧹 Cleaning up workspace: {}", self.repo_path.display());
            fs::remove_dir_all(&self.repo_path).map_err(|e| {
                ModelUpdateError::FileSystemError(format!("Failed to cleanup workspace: {e}"))
            })?;
            println!("   ✅ Workspace cleaned up");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio;

    #[tokio::test]
    async fn test_model_updater_new_creates_correct_paths() {
        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");

        // Act
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // Assert
        assert_eq!(
            updater.repo_url,
            "https://huggingface.co/ggerganov/whisper.cpp"
        );
        assert_eq!(updater.repo_path, temp_dir.path().join("models-repo"));
    }

    #[tokio::test]
    async fn test_clone_repository_creates_models_repo_directory() {
        // RED: This test will fail initially because clone_repository is not implemented

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // Act
        let result = updater.clone_repository().await;

        // Assert
        assert!(result.is_ok(), "Repository clone should succeed");
        assert!(
            updater.repo_path.exists(),
            "models-repo directory should exist"
        );
    }

    #[tokio::test]
    async fn test_extract_model_metadata_returns_correct_structure() {
        // RED: This test will fail initially because extract_model_metadata is not implemented

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // First need to clone the repository
        updater
            .clone_repository()
            .await
            .expect("Should clone repository");

        // Act
        let result = updater.extract_model_metadata("ggml-base-q5_1.bin").await;

        // Assert
        assert!(result.is_ok(), "Metadata extraction should succeed");
        let metadata = result.unwrap();
        assert_eq!(metadata.filename, "ggml-base-q5_1.bin");
        assert!(!metadata.sha256.is_empty(), "SHA256 should not be empty");
        assert!(metadata.size_bytes > 0, "Size should be greater than 0");
        assert!(!metadata.git_ref.is_empty(), "Git ref should not be empty");
        assert!(
            metadata.download_url.contains("huggingface.co"),
            "URL should contain huggingface.co"
        );
    }

    #[tokio::test]
    async fn test_list_model_files_returns_expected_files() {
        // RED: This test will fail initially because list_model_files is not implemented

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // First need to clone the repository
        updater
            .clone_repository()
            .await
            .expect("Should clone repository");

        // Act
        let result = updater.list_model_files().await;

        // Assert
        assert!(result.is_ok(), "Model file listing should succeed");
        let files = result.unwrap();
        assert!(!files.is_empty(), "Should find model files");
        assert!(
            files.iter().any(|f| f == "ggml-base-q5_1.bin"),
            "Should contain base model"
        );
        assert!(
            files.iter().any(|f| f == "ggml-tiny.bin"),
            "Should contain tiny model"
        );
    }

    #[tokio::test]
    async fn test_extract_all_metadata_processes_multiple_models() {
        // RED: This test will fail initially because extract_all_metadata is not implemented

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // First need to clone the repository
        updater
            .clone_repository()
            .await
            .expect("Should clone repository");

        // Act
        let result = updater.extract_all_metadata().await;

        // Assert
        assert!(result.is_ok(), "Bulk metadata extraction should succeed");
        let all_metadata = result.unwrap();
        assert!(
            !all_metadata.is_empty(),
            "Should extract metadata for multiple models"
        );

        // Check that we have common models
        assert!(all_metadata
            .iter()
            .any(|m| m.filename == "ggml-base-q5_1.bin"));
        assert!(all_metadata.iter().any(|m| m.filename == "ggml-tiny.bin"));

        // Verify all metadata has required fields
        for metadata in &all_metadata {
            assert!(!metadata.filename.is_empty());
            assert!(!metadata.sha256.is_empty());
            assert!(metadata.size_bytes > 0);
            assert!(!metadata.git_ref.is_empty());
            assert!(metadata.download_url.contains("huggingface.co"));
        }
    }

    #[tokio::test]
    async fn test_extract_model_metadata_handles_nonexistent_file() {
        // Test error handling for files that don't exist

        // Arrange
        let temp_dir = TempDir::new().expect("Should create temp dir");
        let updater = ModelListUpdater::new(temp_dir.path().to_path_buf());

        // First need to clone the repository
        updater
            .clone_repository()
            .await
            .expect("Should clone repository");

        // Act
        let result = updater
            .extract_model_metadata("nonexistent-model.bin")
            .await;

        // Assert
        assert!(result.is_err(), "Should return error for nonexistent file");
        match result.unwrap_err() {
            ModelUpdateError::FileSystemError(_) => {} // Expected error type
            other => panic!("Expected FileSystemError, got {other:?}"),
        }
    }
}
