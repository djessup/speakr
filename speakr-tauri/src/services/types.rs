// ============================================================================
//! Service Types & Enums
// ============================================================================

/// Enum to identify different service components
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceComponent {
    AudioCapture,
    Transcription,
    TextInjection,
}
