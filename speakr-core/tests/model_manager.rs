//! Integration tests for `ModelManager` (task 2.1)
//!
//! We avoid any real network traffic by spinning up an in-process HTTP mock
//! server (powered by `wiremock`).  The server serves a tiny payload so that
//! tests finish quickly even on CI machines.

use std::path::Path;

use sha2::{Digest, Sha256};
use speakr_core::transcription::models::ModelManager;
use tempfile::TempDir;
use tokio::fs;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test(flavor = "multi_thread")]
async fn downloads_and_validates_model() {
    // ---------------------------------------------------------------------
    // Arrange – spin up mock server with deterministic payload
    // ---------------------------------------------------------------------
    let server = MockServer::start().await;

    let payload = b"dummy model bytes";
    let checksum = hex::encode(Sha256::digest(payload));

    Mock::given(method("GET"))
        .and(path("/model.gguf"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(payload))
        .mount(&server)
        .await;

    let url = format!("{}/model.gguf", server.uri());

    // Use a temporary cache directory to keep the test isolated
    let tmp_dir = TempDir::new().expect("create temp dir");
    let manager = ModelManager::with_cache_dir(tmp_dir.path().to_path_buf());

    // ---------------------------------------------------------------------
    // Act – download model
    // ---------------------------------------------------------------------
    let path = manager
        .download_model(&url, Some(&checksum))
        .await
        .expect("download succeeds");

    // ---------------------------------------------------------------------
    // Assert – file exists & content matches
    // ---------------------------------------------------------------------
    assert!(Path::new(&path).exists(), "cached file exists");

    let downloaded = fs::read(&path).await.expect("read file");
    assert_eq!(
        payload.as_slice(),
        downloaded.as_slice(),
        "file content matches"
    );
}
