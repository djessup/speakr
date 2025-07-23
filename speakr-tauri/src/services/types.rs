//! Service types and enums.

/// Enum to identify different service components
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceComponent {
    AudioCapture,
    Transcription,
    TextInjection,
}
