# Model Management

Catalogue of supported Whisper GGUF models and a manager for caching, checksum verification, and downloads.

## Model Enum

`speakr_core::model::Model` lists all supported variants (e.g. `Small`, `Medium`, `LargeV3Turbo`, and quantised flavours). Helpers:

- `filename() -> &str` – basename without `ggml-` prefix/suffix
- `filesize() -> size::Size`
- `memory_usage_mb() -> u32` – peak RAM estimate
- `supported_languages() -> Option<&[&str]>` – English-only for `.en`
- `sha() -> &str` – expected checksum
- `url() -> String` – default HuggingFace URL (overridable via `SPEAKR_MODEL_BASE_URL`)
- `iter() -> impl Iterator<Item=Model>`

## ModelManager

- `new()`, `with_cache_dir(PathBuf)`
- `cache_dir() -> &Path`
- `download_model(url, expected_checksum) -> Result<PathBuf, ModelManagerError>`
- `download_model_with_retry(model, retries) -> Result<PathBuf, ModelManagerError>`
- `is_available(model, verify_hash) -> Result<bool, std::io::Error>`
- `available_models() -> Vec<Model>`
- `recommend_for_current_system() -> Vec<Model>`

## Example

```rust
use speakr_core::model::Model;
use speakr_core::transcription::models::ModelManager;

# async fn ensure_small() -> anyhow::Result<()> {
let mgr = ModelManager::new();
let model = Model::Small;
if !mgr.is_available(&model, false).await? {
    let _path = mgr.download_model_with_retry(&model, 2).await?;
}
assert!(mgr.is_available(&model, true).await?);
# Ok(()) }
```
