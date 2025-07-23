/// Metadata for a single model file
#[derive(Debug, Clone, PartialEq)]
pub struct ModelMetadata {
    pub filename: String,
    pub sha256: String,
    pub size_bytes: u64,
    pub git_ref: String,
    pub download_url: String,
}

/// Converts a model filename to a valid Rust enum variant name
///
/// This function transforms model filenames (like "ggml-base.en.bin") into
/// valid Rust identifiers using camelCase convention. The conversion process:
/// 1. Removes "ggml-" prefix and ".bin" suffix
/// 2. Converts separators (-, _, .) to camelCase boundaries
/// 3. Capitalizes the first letter of each word
/// 4. Handles special quantized model patterns (Q51 → QuantizedQ5_1)
/// 5. Preserves numbers in their original positions
///
/// # Arguments
///
/// * `filename` - The original model filename (e.g., "ggml-base.bin")
///
/// # Returns
///
/// A valid Rust identifier suitable for use as an enum variant (e.g., "BaseEn")
///
/// # Examples
///
/// ```rust
/// use speakr_core::model::filename_to_variant_name;
///     assert_eq!(filename_to_variant_name("ggml-base.bin"), "Base");
///     assert_eq!(filename_to_variant_name("ggml-small.en.bin"), "SmallEn");
///     assert_eq!(filename_to_variant_name("ggml-medium.q5_1.bin"), "MediumQuantizedQ5_1");
/// ```
pub fn filename_to_variant_name(filename: &str) -> String {
    let base = filename
        .trim_start_matches("ggml-")
        .trim_end_matches(".bin");

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

    // Handle quantized model patterns after camel case conversion
    // FIXME: Use a regex to replace generic Q_AB with QuantizedQ_A_B
    result = result.replace("Q51", "QuantizedQ5_1");
    result = result.replace("Q80", "QuantizedQ8_0");
    result = result.replace("Q50", "QuantizedQ5_0");

    result
}
