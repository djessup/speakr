// ============================================================================
//! Service Types & Enums
// ============================================================================

/// Enum to identify different service components
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ServiceComponent {
    AudioCapture,
    Transcription,
    TextInjection,
}
