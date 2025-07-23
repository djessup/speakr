# Speakr Core Library

## Models

whisper.cpp models are available on Hugging Face.

### Automated Model Metadata Updates

The preferred way to update model metadata is using the automated CLI tool:

```sh
# Update models.rs with latest metadata from Hugging Face
cargo run --bin update-models

# Specify custom workspace and output locations
cargo run --bin update-models -- --workspace-dir ./temp --output ./updated_models.rs
```

**What it does:**

1. Automatically clones the [whisper.cpp repository][] without LFS files
2. Extracts metadata (SHA256, file size, git reference) from all model files
3. Generates updated Rust code for `src/model.rs` with current data
4. Creates properly formatted enum variants and implementation methods

**Output:** The tool generates a complete `models.rs` file with:

- Updated SHA256 checksums for all models
- Current file sizes and download URLs
- Proper Rust enum variants and match arms
- Documentation table with all available models

**Example output:**

```rust
/// | Model               | Disk    | SHA                                        |
/// | ------------------- | ------- | ------------------------------------------ |
/// | tiny                | 75 MiB  | `bd577a113a864445d4c299885e0cb97d4ba92b5f` |
/// | base-q5_1           | 57 MiB  | `a3733eda680ef76256db5fc5dd9de8629e62c5e7` |

#[derive(Debug)]
pub enum Model {
    Tiny,
    BaseQuantizedQ5_1,
    // ... all available models
}

impl Model {
    pub fn sha(&self) -> &'static str {
        match self {
            Model::Tiny => "bd577a113a864445d4c299885e0cb97d4ba92b5f",
            Model::BaseQuantizedQ5_1 => "a3733eda680ef76256db5fc5dd9de8629e62c5e7",
            // ... updated with current values
        }
    }
    // ... other methods
}
```

**Next steps after running:**

1. Review the generated code in `updated_models.rs` (or specified output file)
2. Replace the content in `src/model.rs` with the updated code
3. Run `cargo test` to verify everything works correctly

### Programmatic Usage

You can also use the model updater programmatically in your Rust code:

```rust
use speakr_core::model::{ModelListUpdater, ModelMetadata};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create updater with workspace directory
    let updater = ModelListUpdater::new(PathBuf::from("./temp-models"));

    // Clone repository and extract metadata
    updater.clone_repository().await?;
    let metadata = updater.extract_all_metadata().await?;

    // Process metadata
    for model in metadata {
        println!("Model: {} (SHA: {}, Size: {} bytes)",
                 model.filename, model.sha256, model.size_bytes);
    }

    Ok(())
}
```

**Available methods:**

- `ModelListUpdater::new(workspace_dir)` - Create new updater instance
- `clone_repository()` - Clone whisper.cpp repo without LFS files
- `list_model_files()` - Get list of available model files
- `extract_model_metadata(filename)` - Extract metadata for specific model
- `extract_all_metadata()` - Extract metadata for all models

### Manual Process (Reference)

For understanding how the automation works, here's the manual process:

Determine the file name, checksum, size, and download URL, as follows:

1. Prerequisites: install `git` and `git-lfs` (these are already included in
   [devenv.nix](../devenv.nix))
2. Clone the repository (without downloading the LFS binary files)

   ```sh
   export REPO="ggerganov/whisper.cpp"
   GIT_LFS_SKIP_SMUDGE=1 git clone "https://huggingface.co/${REPO}" ./models-repo
   ```

3. Git will create a directory `models-repo` in the current working directory with the model
   files containing a small text-based metadata payload, instead of the full binary - e.g.:

   ```sh
   # file: ./models-repo/ggml-large-v3-turbo-q8_0.bin
   version https://git-lfs.github.com/spec/v1
   oid sha256:317eb69c11673c9de1e1f0d459b253999804ec71ac4c23c17ecf5fbe24e259a1
   size 874188075
   ```

   This provides the **file name**, **SHA256 checksum**, and **size** of the model file (in bytes).

4. The download URL is constructed as:

   ```sh
   "https://huggingface.co/${REPO}/resolve/${GIT_REF}/${FILE_NAME}"
   ```

   The git reference is obtained via:

    ```sh
    export FILE_NAME="ggml-base-q5_1.bin"
    git log -n 1 --pretty=format:%H -- $FILE_NAME | cat

    # Output: f281eb45af861ab5e5297d23694b7d46e090c02c
    ```

    Putting it all together:

    ```sh
    # Clone the repository
    export REPO="ggerganov/whisper.cpp"
    GIT_LFS_SKIP_SMUDGE=1 git clone "https://huggingface.co/${REPO}" ./models-repo
    cd ./models-repo

    # Get the download URL
    export FILE_NAME="ggml-base-q5_1.bin"
    export GIT_REF=$(git log -n 1 --pretty=format:%H -- $FILE_NAME | cat)
    export DOWNLOAD_URL="https://huggingface.co/${REPO}/resolve/${GIT_REF}/${FILE_NAME}"
    echo "The download URL for ${FILE_NAME} is: ${DOWNLOAD_URL}"

    # Output: The download URL for ggml-base-q5_1.bin is: https://huggingface.co/ggerganov/whisper.cpp/resolve/f281eb45af861ab5e5297d23694b7d46e090c02c/ggml-base-q5_1.bin
    ```

[whisper.cpp repository]: https://huggingface.co/ggerganov/whisper.cpp
