// ============================================================================
//! Model Manager CLI – list, download, remove, verify Whisper GGML models.
//!
//! This is a **developer-facing** utility – it never ships to end-users.  The
//! binary provides a thin wrapper around [`speakr_core::transcription::models::ModelManager`]
//! so that maintainers can quickly inspect or manipulate the on-disk model
//! cache.
//!
//! Example usage:
//!
//! ```bash
//! # List locally cached models (and their status)
//! cargo run -p speakr-core --bin models-cli -- list
//!
//! # Download the tiny.en model (2 retries on failure)
//! cargo run -p speakr-core --bin models-cli -- download tiny.en --retries 2
//!
//! # Verify SHA-256 checksums for *all* cached models
//! cargo run -p speakr-core --bin models-cli -- verify --all
//! ```
//!
//! The CLI is intentionally **minimal** – fancy progress bars and UI bells are
//! handled by the dedicated *update-models-tui* binary.
// ============================================================================

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use speakr_core::model::Model;
use speakr_core::transcription::models::ModelManager;
use std::io::{self, Write};
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Speakr Model Manager CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List models (locally cached, recommended, or all supported)
    List {
        /// Only print models that are already cached locally
        #[arg(long)]
        cached: bool,

        /// Print models that are recommended for this system (based on memory)
        #[arg(long)]
        recommended: bool,
    },

    /// Download a model into the local cache
    Download {
        /// Model to download (e.g. "tiny", "base.en-q5_1"). Use "all" to download every model.
        model: String,

        /// Number of retries on transient network errors
        #[arg(long, default_value_t = 2)]
        retries: u8,

        /// Force re-download even if file is already cached and valid
        #[arg(long)]
        force: bool,
    },

    /// Remove a model from the cache (or clean the entire cache)
    Remove {
        /// Model to remove. Use "all" to remove the entire cache directory.
        model: String,
    },

    /// Verify SHA-256 checksum of cached model(s)
    Verify {
        /// Model to verify. Use "all" to verify every cached model.
        model: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // Disable backtrace for expected user errors
    std::env::set_var("RUST_BACKTRACE", "0");

    let cli = Cli::parse();
    let manager = ModelManager::new();

    let result: Result<()> = match cli.command {
        Commands::List {
            cached,
            recommended,
        } => handle_list(&manager, cached, recommended).await,
        Commands::Download {
            model,
            retries,
            force,
        } => handle_download(&manager, &model, retries, force).await,
        Commands::Remove { model } => handle_remove(&manager, &model).await,
        Commands::Verify { model } => handle_verify(&manager, &model).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

async fn handle_list(manager: &ModelManager, cached: bool, recommended: bool) -> Result<()> {
    if cached && recommended {
        return Err(anyhow!("--cached and --recommended are mutually exclusive"));
    }

    let models: Vec<Model> = if cached {
        manager.available_models().await
    } else if recommended {
        manager.recommend_for_current_system()
    } else {
        Model::iter().collect()
    };

    if models.is_empty() {
        println!("No models found.");
        return Ok(());
    }

    // Header
    println!("{:1} {:<30} {:>8} SHA", " ", "Model", "Size");
    println!("{:-<1} {:-<30} {:-<8} {:-<64}", "", "", "", "");

    for model in models {
        let filename = model.filename();
        let size = model.filesize();
        let available = manager.is_available(&model, false).await.unwrap_or(false);
        let tick = if available { "✔" } else { "✘" };
        println!(
            "{tick} {:<30} {:>8} {}",
            filename,
            format_size(size.bytes() as u64),
            &model.sha()[..8] // shorten sha for table
        );
    }

    Ok(())
}

async fn handle_download(
    manager: &ModelManager,
    model_arg: &str,
    retries: u8,
    force: bool,
) -> Result<()> {
    let client = reqwest::Client::new();
    let targets = resolve_models(model_arg)?;

    // Ensure cache dir exists
    tokio::fs::create_dir_all(manager.cache_dir()).await.ok();

    for m in targets {
        // Skip download if already cached and valid (unless --force)
        if !force && manager.is_available(&m, true).await.unwrap_or(false) {
            println!("✔ {} already downloaded and valid.", m.filename());
            print!("Download again? [y/N]: ");
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            if !matches!(input.chars().next(), Some('y') | Some('Y')) {
                continue;
            }
        }

        let url = m.url();
        let filename = format!("ggml-{}.bin", m.filename());
        let dest_path = manager.cache_dir().join(&filename);
        let tmp_path = dest_path.with_extension("tmp");
        // Remove existing file if forcing re-download
        if dest_path.exists() {
            let _ = tokio::fs::remove_file(&dest_path).await;
        }

        println!("⬇️  Downloading {}...", m.filename());

        let mut attempt = 0;
        loop {
            let resp = client.get(&url).send().await?;
            let total = resp.content_length();
            let pb = match total {
                Some(t) => ProgressBar::new(t),
                None => ProgressBar::new_spinner(),
            };
            pb.set_style(
                ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
                )
                    .unwrap()
                    .progress_chars("▰▱▱")
            );

            let mut file = tokio::fs::File::create(&tmp_path).await?;
            let mut stream = resp.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;
                pb.inc(chunk.len() as u64);
            }
            file.flush().await?;
            pb.finish_and_clear();

            // Rename atomically
            tokio::fs::rename(&tmp_path, &dest_path).await?;

            // Verify checksum
            if manager.is_available(&m, true).await? {
                println!("   ✅ Saved to {}", dest_path.display());
                break;
            } else {
                println!("   ⚠ Checksum mismatch, retrying...");
                attempt += 1;
                if attempt > retries {
                    return Err(anyhow!(
                        "Failed to download {} after {} retries",
                        m.filename(),
                        retries
                    ));
                }
                // Remove bad file and retry
                let _ = tokio::fs::remove_file(&dest_path).await;
                continue;
            }
        }
    }
    Ok(())
}

async fn handle_remove(manager: &ModelManager, model_arg: &str) -> Result<()> {
    let targets = resolve_models(model_arg)?;

    if targets.len() == Model::iter().count() {
        // all
        let dir = manager.cache_dir();
        if dir.exists() {
            fs::remove_dir_all(dir).await?;
            println!("🗑️  Removed entire cache at {}", dir.display());
        } else {
            println!("Cache directory {} does not exist", dir.display());
        }
        return Ok(());
    }

    for m in targets {
        let path = manager
            .cache_dir()
            .join(format!("ggml-{}.bin", m.filename()));
        if path.exists() {
            fs::remove_file(&path).await?;
            println!("🗑️  Removed {}", path.display());
        } else {
            println!("Model {} not found in cache", m.filename());
        }
    }
    Ok(())
}

async fn handle_verify(manager: &ModelManager, model_arg: &str) -> Result<()> {
    let targets: Vec<Model> = if model_arg.eq_ignore_ascii_case("all") {
        manager.available_models().await
    } else {
        resolve_models(model_arg)?
    };

    if targets.is_empty() {
        println!("No cached models found.");
        return Ok(());
    }

    for m in targets {
        let filename = m.filename();
        let path = manager.cache_dir().join(format!("ggml-{filename}.bin"));

        if !path.exists() {
            println!("✘ {filename} missing");
            continue;
        }

        let expected = m.sha();
        let actual = compute_checksum(&path, expected.len()).await?;
        if actual.eq_ignore_ascii_case(expected) {
            println!("✔ {} OK ({})", filename, &actual[..8]);
        } else {
            println!(
                "⚠ {} checksum mismatch (expected {}, got {})",
                filename,
                &expected[..8],
                &actual[..8]
            );
        }
    }
    Ok(())
}

/// Compute SHA-1 (40 hex) or SHA-256 (64 hex) checksum depending on `expected_len`.
async fn compute_checksum(path: &std::path::Path, expected_len: usize) -> Result<String> {
    use sha1::Sha1;
    use sha2::{Digest, Sha256};
    use tokio::io::{AsyncReadExt, BufReader};

    let file = tokio::fs::File::open(path).await?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1 MiB buffer

    match expected_len {
        40 => {
            let mut hasher = Sha1::new();
            let mut buf = vec![0u8; 1024 * 1024];
            loop {
                let n = reader.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        64 => {
            let mut hasher = Sha256::new();
            let mut buf = vec![0u8; 1024 * 1024];
            loop {
                let n = reader.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
            }
            Ok(hex::encode(hasher.finalize()))
        }
        _ => Ok(String::new()),
    }
}

/// Convert a user-provided argument (model name or "all") into a list of Models.
fn resolve_models(input: &str) -> Result<Vec<Model>> {
    if input.eq_ignore_ascii_case("all") {
        return Ok(Model::iter().collect());
    }

    // Accept both filename style ("tiny.en-q5_1") and variant name ("SmallQuantizedQ8_0")
    if let Some(model) = Model::iter()
        .find(|m| (m.filename().eq_ignore_ascii_case(input) || format!("{m:?}") == input))
    {
        return Ok(vec![model]);
    }

    Err(anyhow!("Unknown model: {input}"))
}

fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.1} GiB", (bytes as f64) / (GIB as f64))
    } else if bytes >= MIB {
        format!("{} MiB", bytes / MIB)
    } else if bytes >= KIB {
        format!("{} KiB", bytes / KIB)
    } else {
        format!("{bytes} B")
    }
}

// -----------------------------------------------------------------------------
// Tests – follow TDD 🔴🟢🔵
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_models_by_filename() {
        let result = resolve_models("tiny").expect("should resolve");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Model::Tiny);
    }

    #[tokio::test]
    async fn test_resolve_models_all() {
        let result = resolve_models("all").expect("should resolve all");
        assert_eq!(result.len(), Model::iter().count());
    }

    #[tokio::test]
    async fn test_handle_list_cached_only() {
        let temp = tempfile::TempDir::new().unwrap();
        let manager = ModelManager::with_cache_dir(temp.path().to_path_buf());
        // Should not error even if cache is empty
        handle_list(&manager, true, false).await.unwrap();
    }
}
