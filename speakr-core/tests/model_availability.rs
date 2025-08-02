//! Integration tests for model availability and listing (task 2.2)

use speakr_core::model::Model;
use speakr_core::transcription::models::ModelManager;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test(flavor = "multi_thread")]
async fn lists_available_models_in_cache() {
    // ---------------------------------------------------------------------
    // Arrange – prepare temporary cache with a single dummy model
    // ---------------------------------------------------------------------
    let tmp_dir = TempDir::new().expect("create temp dir");
    let manager = ModelManager::with_cache_dir(tmp_dir.path().to_path_buf());

    // Create a dummy file that represents a downloaded Tiny model
    let tiny_model = Model::Tiny;
    let path = tmp_dir
        .path()
        .join(format!("ggml-{}.bin", tiny_model.filename()));
    fs::write(&path, b"dummy").await.expect("write dummy model");

    // ---------------------------------------------------------------------
    // Act – query available models
    // ---------------------------------------------------------------------
    let list = manager.available_models().await;

    // ---------------------------------------------------------------------
    // Assert – the list contains the Tiny model and nothing else
    // ---------------------------------------------------------------------
    assert!(list.contains(&tiny_model));
}

#[test]
fn model_memory_usage_ordering() {
    // The Tiny model should require less RAM than the Medium model which in turn
    // is lighter than the Large model variants.
    assert!(Model::Tiny.memory_usage_mb() < Model::Medium.memory_usage_mb());
    assert!(Model::Medium.memory_usage_mb() < Model::LargeV1.memory_usage_mb());
}
