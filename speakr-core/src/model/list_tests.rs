// ============================================================================
// Model List – Unit-Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::model::*;
    use size::Size;

    #[test]
    fn test_model_filesize_tiny() {
        let size = Model::Tiny.filesize();
        assert_eq!(size.bytes(), Size::from_mib(75).bytes());
        // Test that it formats as MiB (binary unit) and contains "75"
        let formatted = format!("{size}");
        assert!(formatted.contains("75") && formatted.contains("MiB"));
    }

    #[test]
    fn test_model_filesize_medium() {
        let size = Model::Medium.filesize();
        assert_eq!(size.bytes(), Size::from_gib(1.5).bytes());
        // Test that it formats as GiB (binary unit) and contains the appropriate value
        let formatted = format!("{size}");
        assert!(formatted.contains("GiB"));
    }

    #[test]
    fn test_model_filesize_large_v3() {
        let size = Model::LargeV3.filesize();
        assert_eq!(size.bytes(), Size::from_gib(2.9).bytes());
        // Test that it formats as GiB (binary unit)
        let formatted = format!("{size}");
        assert!(formatted.contains("GiB"));
    }

    #[test]
    fn test_model_filename() {
        assert_eq!(Model::Tiny.filename(), "tiny");
        assert_eq!(Model::Medium.filename(), "medium");
        assert_eq!(Model::LargeV3Turbo.filename(), "large-v3-turbo");
    }

    #[test]
    fn test_model_sha() {
        assert_eq!(
            Model::Tiny.sha(),
            "bd577a113a864445d4c299885e0cb97d4ba92b5f"
        );
        assert_eq!(
            Model::Medium.sha(),
            "fd9727b6e1217c2f614f9b698455c4ffd82463b4"
        );
    }

    #[test]
    fn test_model_url() {
        let tiny_url = Model::Tiny.url();
        let medium_url = Model::Medium.url();

        // URL should contain the expected base URL and filename
        assert!(tiny_url.contains("huggingface.co/ggerganov/whisper.cpp/resolve/"));
        assert!(tiny_url.ends_with("/ggml-tiny.bin"));

        assert!(medium_url.contains("huggingface.co/ggerganov/whisper.cpp/resolve/"));
        assert!(medium_url.ends_with("/ggml-medium.bin"));
    }
}
