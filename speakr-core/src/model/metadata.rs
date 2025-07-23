/// Metadata for a single model file
#[derive(Debug, Clone, PartialEq)]
pub struct ModelMetadata {
    pub filename: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub git_ref: String,
    pub download_url: String,
}
