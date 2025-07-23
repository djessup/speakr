/// CLI tool to update models/list.rs with latest metadata from Hugging Face
///
/// This tool automates the process of:
/// 1. Clone the whisper.cpp repository without LFS files
/// 2. Extract metadata (SHA256, size, git ref) from LFS pointer files
/// 3. Generate updated Rust code for models/list.rs
///
/// Usage: cargo run --bin update-models [--repo <owner/repo>] [--workspace-dir <dir>] [--output <file>]
use std::fs;
use std::path::PathBuf;

use size::Size;
use tempfile::TempDir;

use speakr_core::model::{ModelListUpdater, ModelMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Parse command line arguments (simple parsing for now)
    let repo_name =
        get_arg_value(&args, "--repo").unwrap_or_else(|| "ggerganov/whisper.cpp".to_string());

    let workspace_dir = if let Some(custom_dir) = get_arg_value(&args, "--workspace-dir") {
        println!("🗂️  Using custom workspace directory: {custom_dir}");
        Some(PathBuf::from(custom_dir))
    } else {
        println!("🗂️  Using temporary workspace directory (will be cleaned up automatically)");
        None
    };

    let output_file = get_arg_value(&args, "--output")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("updated_list.rs"));

    println!("🚀 Starting model metadata extraction...");
    println!("   📦 Repository: {repo_name}");
    println!("   📤 Output:     {}", output_file.display());
    println!();

    // Use temp directory if no custom workspace specified, otherwise use user-provided directory
    let result = if let Some(workspace_path) = workspace_dir {
        // Use persistent directory
        let updater = ModelListUpdater::with_repo(workspace_path.clone(), &repo_name);
        println!(
            "   📁 Workspace:  {} (user-provided)",
            workspace_path.display()
        );
        process_models(&updater, &output_file).await
    } else {
        // Use temporary directory and clean up automatically
        let temp_dir = TempDir::new()?;
        let updater = ModelListUpdater::with_repo(temp_dir.path().to_path_buf(), &repo_name);
        println!(
            "   📁 Workspace:  {} (temporary)",
            temp_dir.path().display()
        );

        let result = process_models(&updater, &output_file).await;

        // Temp directory will be automatically cleaned up when dropped
        println!("🧹 Temporary workspace will be cleaned up automatically");
        result
    };

    match result {
        Ok(metadata_count) => {
            println!();
            println!("🎉 Model update complete!");
            println!("   ✅ Processed {metadata_count} models successfully");
            println!("   📄 Generated code written to: {}", output_file.display());
            println!();
            println!("📋 Next steps:");
            println!(
                "   1. Review the generated code in {}",
                output_file.display()
            );
            println!("   2. Replace the content in src/model/list.rs with the updated code");
            println!("   3. Run `cargo test` to verify everything works");
            println!("   4. Commit the changes if everything looks good");
        }
        Err(e) => {
            eprintln!("❌ Model update failed: {e}");
            return Err(e);
        }
    }

    Ok(())
}

/// Processes models and returns the count of successfully processed models
async fn process_models(
    updater: &ModelListUpdater,
    output_file: &PathBuf,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("📥 Cloning repository...");
    println!("   🌐 URL: {}", updater.repo_url());
    updater.clone_repository().await?;

    println!();
    println!("📋 Extracting metadata for all models...");
    let metadata = updater.extract_all_metadata().await?;

    println!();
    println!("📊 Processing results:");
    println!(
        "   ✅ Successfully extracted metadata for {} models",
        metadata.len()
    );

    // Show summary of models found
    if !metadata.is_empty() {
        println!("   📦 Models found:");
        for (i, meta) in metadata.iter().enumerate() {
            let model_name = meta
                .filename
                .trim_start_matches("ggml-")
                .trim_end_matches(".bin");
            let size = Size::from_bytes(meta.size_bytes);
            let formatted_size = if size.bytes() >= 1_000_000_000 {
                format!(
                    "{:.1} GiB",
                    (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
                )
            } else {
                format!("{} MiB", size.bytes() / (1024 * 1024))
            };
            println!("      {}: {} ({})", i + 1, model_name, formatted_size);
        }
    }

    println!();
    println!("🔧 Generating updated models.rs content...");
    let updated_code = generate_models_code(&metadata);

    println!("💾 Writing generated code to output file...");
    fs::write(output_file, updated_code)?;
    println!("   ✅ Code generation complete");

    Ok(metadata.len())
}

/// Simple command line argument parser
fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|pos| args.get(pos + 1))
        .cloned()
}

/// Generates the complete models.rs file content with updated metadata
fn generate_models_code(metadata: &[ModelMetadata]) -> String {
    let mut code = String::new();

    // Add the header documentation with model table
    code.push_str(&generate_header_docs(metadata));
    code.push_str("\n\n");

    // Add imports
    code.push_str("use size::Size;\n\n");

    // Add the enum definition
    code.push_str(&generate_model_enum(metadata));
    code.push_str("\n\n");

    // Add implementation block
    code.push_str(&generate_impl_block(metadata));
    code.push_str("\n\n");

    code
}

/// Generates the header documentation with model table
fn generate_header_docs(metadata: &[ModelMetadata]) -> String {
    let mut docs = String::new();
    docs.push_str(
        "/// # OpenAI's Whisper models converted to ggml format for use with [whisper.cpp](https://github.com/ggerganov/whisper.cpp)\n"
    );
    docs.push_str("///\n");

    // Use the first metadata entry to determine the repo for the link
    let repo_link = if let Some(first_meta) = metadata.first() {
        // Extract repo name from download URL
        if let Some(repo_start) = first_meta.download_url.find("huggingface.co/") {
            let after_hf = &first_meta.download_url[repo_start + 15..];
            if let Some(resolve_pos) = after_hf.find("/resolve/") {
                let repo_name = &after_hf[..resolve_pos];
                format!("https://huggingface.co/{repo_name}/tree/main")
            } else {
                "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
            }
        } else {
            "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
        }
    } else {
        "https://huggingface.co/ggerganov/whisper.cpp/tree/main".to_string()
    };

    docs.push_str(&format!("/// [Available models]({repo_link})\n"));
    docs.push_str("///\n");
    docs.push_str(
        "/// | Model               | Disk    | SHA                                        |\n",
    );
    docs.push_str(
        "/// | ------------------- | ------- | ------------------------------------------ |\n",
    );

    for meta in metadata {
        let model_name = meta
            .filename
            .trim_start_matches("ggml-")
            .trim_end_matches(".bin");
        let size = Size::from_bytes(meta.size_bytes);
        let formatted_size = if size.bytes() >= 1_000_000_000 {
            format!(
                "{:.1} GiB",
                (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
            )
        } else {
            format!("{} MiB", size.bytes() / (1024 * 1024))
        };

        docs.push_str(&format!(
            "/// | {:<19} | {:<7} | `{}` |\n",
            model_name, formatted_size, meta.sha256
        ));
    }

    docs.push_str("///\n");
    docs.push_str(&format!(
        "/// Example: {}\n",
        metadata
            .first()
            .map(|m| m.download_url.as_str())
            .unwrap_or("")
    ));

    docs
}

/// Generates the Model enum based on available models
fn generate_model_enum(metadata: &[ModelMetadata]) -> String {
    let mut enum_code = String::new();
    enum_code.push_str("#[allow(dead_code)]\n");
    enum_code.push_str("#[derive(Debug)]\n");
    enum_code.push_str("pub enum Model {\n");

    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        enum_code.push_str(&format!("    {variant_name},\n"));
    }

    enum_code.push_str("}\n");
    enum_code
}

/// Generates the impl block with all methods
fn generate_impl_block(metadata: &[ModelMetadata]) -> String {
    let mut impl_code = String::new();
    impl_code.push_str("impl Model {\n");

    // Generate filename method
    impl_code.push_str("    pub fn filename(&self) -> &'static str {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        let base_name = meta
            .filename
            .trim_start_matches("ggml-")
            .trim_end_matches(".bin");
        impl_code.push_str(&format!(
            "            Model::{variant_name} => \"{base_name}\",\n"
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate filesize method
    impl_code.push_str("    pub fn filesize(&self) -> Size {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        let size = Size::from_bytes(meta.size_bytes);
        let size_expr = if size.bytes() >= 1_000_000_000 {
            format!(
                "Size::from_gib({:.1})",
                (size.bytes() as f64) / (1024.0 * 1024.0 * 1024.0)
            )
        } else {
            format!("Size::from_mib({})", size.bytes() / (1024 * 1024))
        };
        impl_code.push_str(&format!(
            "            Model::{variant_name} => {size_expr},\n"
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate sha method
    impl_code.push_str("    pub fn sha(&self) -> &'static str {\n");
    impl_code.push_str("        match self {\n");
    for meta in metadata {
        let variant_name = filename_to_variant_name(&meta.filename);
        impl_code.push_str(&format!(
            "            Model::{} => \"{}\",\n",
            variant_name, meta.sha256
        ));
    }
    impl_code.push_str("        }\n");
    impl_code.push_str("    }\n\n");

    // Generate url method
    impl_code.push_str("    pub fn url(&self) -> String {\n");
    impl_code.push_str("        format!(\n");

    // Use the first metadata entry to get the repo and git ref
    let (repo_name, git_ref) = if let Some(first_meta) = metadata.first() {
        // Extract repo name from download URL
        let repo = if let Some(repo_start) = first_meta.download_url.find("huggingface.co/") {
            let after_hf = &first_meta.download_url[repo_start + 15..];
            if let Some(resolve_pos) = after_hf.find("/resolve/") {
                after_hf[..resolve_pos].to_string()
            } else {
                "ggerganov/whisper.cpp".to_string()
            }
        } else {
            "ggerganov/whisper.cpp".to_string()
        };
        (repo, first_meta.git_ref.clone())
    } else {
        ("ggerganov/whisper.cpp".to_string(), "main".to_string())
    };

    impl_code.push_str(&format!(
        "            \"https://huggingface.co/{repo_name}/resolve/{git_ref}/ggml-{{}}.bin\",\n"
    ));
    impl_code.push_str("            self.filename()\n");
    impl_code.push_str("        )\n");
    impl_code.push_str("    }\n");

    impl_code.push_str("}\n");
    impl_code
}

/// Converts a filename to a Rust enum variant name
fn filename_to_variant_name(filename: &str) -> String {
    let base = filename
        .trim_start_matches("ggml-")
        .trim_end_matches(".bin");

    // Convert to PascalCase
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in base.chars() {
        match c {
            '-' | '_' | '.' => {
                capitalize_next = true;
            }
            c if c.is_numeric() => {
                result.push(c);
                capitalize_next = false;
            }
            c if capitalize_next => {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            }
            c => {
                result.push(c);
                capitalize_next = false;
            }
        }
    }

    // Handle special cases
    result = result.replace("Q5_1", "QuantizedQ5_1");
    result = result.replace("Q8_0", "QuantizedQ8_0");
    result = result.replace("Q5_0", "QuantizedQ5_0");
    // V1, V2, V3 are already in the correct format, no replacement needed

    result
}
