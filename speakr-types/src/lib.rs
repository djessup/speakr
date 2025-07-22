//! Shared data types for the Speakr application.
//!
//! This crate contains all the common data structures used across
//! the Speakr frontend (speakr-ui) and backend (speakr-tauri) to
//! ensure consistency and avoid duplication.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default schema version for settings
pub const DEFAULT_SCHEMA_VERSION: u32 = 1;

/// Default global hotkey combination
/// Using F1 to avoid conflicts with macOS system shortcuts
/// Backtick conflicts with system shortcuts on macOS
pub const DEFAULT_HOTKEY: &str = "CmdOrCtrl+Alt+F1";

/// Default Whisper model size
pub const DEFAULT_MODEL_SIZE: &str = "medium";

/// Default auto-launch setting
pub const DEFAULT_AUTO_LAUNCH: bool = false;

/// Unified error type for all Tauri backend operations.
/// This matches the error variants expected by the frontend.
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppError {
    /// Settings-related errors
    #[error("Settings error: {0}")]
    Settings(String),

    /// File system operation errors
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Hot-key registration or validation failed
    #[error("Hot-key error: {0}")]
    HotKey(String),

    /// Hot-key conflict detected
    #[error("Hot-key conflict: {0}")]
    HotKeyConflict(String),

    /// Hot-key not found
    #[error("Hot-key not found: {0}")]
    HotKeyNotFound(String),

    /// General command error
    #[error("Command error: {0}")]
    Command(String),
}

/// Global hotkey registration errors
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HotkeyError {
    #[error("Failed to register global hot-key: {0}")]
    RegistrationFailed(String),
    #[error("Hot-key conflict detected: {0}")]
    ConflictDetected(String),
    #[error("Hot-key not found: {0}")]
    NotFound(String),
}

/// Configuration for global hot-key settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HotkeyConfig {
    pub shortcut: String,
    pub enabled: bool,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            shortcut: DEFAULT_HOTKEY.to_string(),
            enabled: true,
        }
    }
}

/// Unified application settings - the single source of truth.
/// This ensures consistency between frontend and backend representations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    /// Schema version for migration support
    #[serde(default = "default_schema_version")]
    pub version: u32,
    /// Global hot-key combination in Tauri format (e.g., "CmdOrCtrl+Alt+`")
    pub hot_key: String,
    /// Selected model size ("small", "medium", "large")
    pub model_size: String,
    /// Whether to auto-launch the app on system startup
    pub auto_launch: bool,
}

fn default_schema_version() -> u32 {
    DEFAULT_SCHEMA_VERSION
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            version: DEFAULT_SCHEMA_VERSION,
            hot_key: DEFAULT_HOTKEY.to_string(),
            model_size: DEFAULT_MODEL_SIZE.to_string(),
            auto_launch: DEFAULT_AUTO_LAUNCH,
        }
    }
}

/// Available Whisper model sizes for transcription
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ModelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ModelSize {
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelSize::Small => "Small (39MB, fast)",
            ModelSize::Medium => "Medium (769MB, balanced)",
            ModelSize::Large => "Large (1550MB, accurate)",
        }
    }

    pub fn to_string_value(&self) -> &'static str {
        match self {
            ModelSize::Small => "small",
            ModelSize::Medium => "medium",
            ModelSize::Large => "large",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "small" => ModelSize::Small,
            "medium" => ModelSize::Medium,
            "large" => ModelSize::Large,
            _ => ModelSize::Medium, // Default fallback
        }
    }

    /// Returns all available model sizes
    pub fn all() -> Vec<ModelSize> {
        vec![ModelSize::Small, ModelSize::Medium, ModelSize::Large]
    }
}

/// Model file information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub size: ModelSize,
    pub filename: String,
    pub display_name: String,
    pub file_size_mb: u32,
    pub description: &'static str,
}

impl ModelInfo {
    /// Get model information for a given size
    pub fn for_size(size: ModelSize) -> Self {
        match size {
            ModelSize::Small => ModelInfo {
                size: ModelSize::Small,
                filename: "ggml-small.bin".to_string(),
                display_name: "Small (39MB, fast)".to_string(),
                file_size_mb: 39,
                description: "Fast processing, good for quick notes",
            },
            ModelSize::Medium => ModelInfo {
                size: ModelSize::Medium,
                filename: "ggml-medium.bin".to_string(),
                display_name: "Medium (769MB, balanced)".to_string(),
                file_size_mb: 769,
                description: "Balanced accuracy and speed",
            },
            ModelSize::Large => ModelInfo {
                size: ModelSize::Large,
                filename: "ggml-large.bin".to_string(),
                display_name: "Large (1550MB, accurate)".to_string(),
                file_size_mb: 1550,
                description: "Highest accuracy, best for professional use",
            },
        }
    }
}

/// Status of an individual service component in the backend
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Ready,
    Starting,
    Error(String),
    Unavailable,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Starting
    }
}

impl ServiceStatus {
    /// Returns the display name for the status
    pub fn display_name(&self) -> &str {
        match self {
            ServiceStatus::Ready => "Ready",
            ServiceStatus::Starting => "Starting",
            ServiceStatus::Error(_) => "Error",
            ServiceStatus::Unavailable => "Unavailable",
        }
    }

    /// Returns true if the service is ready
    pub fn is_ready(&self) -> bool {
        matches!(self, ServiceStatus::Ready)
    }
}

/// Overall backend system status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendStatus {
    pub audio_capture: ServiceStatus,
    pub transcription: ServiceStatus,
    pub text_injection: ServiceStatus,
    pub timestamp: u64,
}

impl BackendStatus {
    /// Returns true if all services are ready
    pub fn is_ready(&self) -> bool {
        self.audio_capture.is_ready()
            && self.transcription.is_ready()
            && self.text_injection.is_ready()
    }

    /// Creates a new status with all services starting
    pub fn new_starting() -> Self {
        Self {
            audio_capture: ServiceStatus::Starting,
            transcription: ServiceStatus::Starting,
            text_injection: ServiceStatus::Starting,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Creates a new status with all services ready
    pub fn new_ready() -> Self {
        Self {
            audio_capture: ServiceStatus::Ready,
            transcription: ServiceStatus::Ready,
            text_injection: ServiceStatus::Ready,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

/// Type alias for status updates sent to frontend
pub type StatusUpdate = BackendStatus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.version, DEFAULT_SCHEMA_VERSION);
        assert_eq!(settings.hot_key, DEFAULT_HOTKEY);
        assert_eq!(settings.model_size, DEFAULT_MODEL_SIZE);
        assert_eq!(settings.auto_launch, DEFAULT_AUTO_LAUNCH);
    }

    #[test]
    fn test_hotkey_config_default() {
        let config = HotkeyConfig::default();
        assert_eq!(config.shortcut, DEFAULT_HOTKEY);
        assert!(config.enabled);
    }

    #[test]
    fn test_model_size_conversions() {
        let medium = ModelSize::Medium;
        assert_eq!(medium.to_string_value(), "medium");
        assert_eq!(medium.display_name(), "Medium (769MB, balanced)");

        let from_str = ModelSize::from_string("small");
        assert_eq!(from_str, ModelSize::Small);
    }

    #[test]
    fn test_model_info() {
        let info = ModelInfo::for_size(ModelSize::Small);
        assert_eq!(info.filename, "ggml-small.bin");
        assert_eq!(info.file_size_mb, 39);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    // 🔴 RED: Tests for status indicator functionality
    #[test]
    fn test_service_status_default() {
        let status = ServiceStatus::default();
        assert!(matches!(status, ServiceStatus::Starting));
    }

    #[test]
    fn test_service_status_display() {
        assert_eq!(ServiceStatus::Ready.display_name(), "Ready");
        assert_eq!(ServiceStatus::Starting.display_name(), "Starting");
        assert_eq!(
            ServiceStatus::Error("test error".to_string()).display_name(),
            "Error"
        );
        assert_eq!(ServiceStatus::Unavailable.display_name(), "Unavailable");
    }

    #[test]
    fn test_backend_status_ready_when_all_services_ready() {
        let status = BackendStatus {
            audio_capture: ServiceStatus::Ready,
            transcription: ServiceStatus::Ready,
            text_injection: ServiceStatus::Ready,
            timestamp: 12345,
        };
        assert!(status.is_ready());
    }

    #[test]
    fn test_backend_status_not_ready_when_services_starting() {
        let status = BackendStatus {
            audio_capture: ServiceStatus::Starting,
            transcription: ServiceStatus::Ready,
            text_injection: ServiceStatus::Ready,
            timestamp: 12345,
        };
        assert!(!status.is_ready());
    }

    #[test]
    fn test_backend_status_not_ready_when_service_error() {
        let status = BackendStatus {
            audio_capture: ServiceStatus::Ready,
            transcription: ServiceStatus::Error("Failed to load model".to_string()),
            text_injection: ServiceStatus::Ready,
            timestamp: 12345,
        };
        assert!(!status.is_ready());
    }

    #[test]
    fn test_backend_status_serialization() {
        let status = BackendStatus {
            audio_capture: ServiceStatus::Ready,
            transcription: ServiceStatus::Starting,
            text_injection: ServiceStatus::Error("Permission denied".to_string()),
            timestamp: 67890,
        };

        let json = serde_json::to_string(&status).unwrap();
        let deserialized: BackendStatus = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.timestamp, status.timestamp);
        assert_eq!(deserialized.audio_capture, status.audio_capture);
        assert_eq!(deserialized.transcription, status.transcription);
        assert_eq!(deserialized.text_injection, status.text_injection);
    }

    #[test]
    fn test_status_update_creation() {
        let update = StatusUpdate::new_ready();
        assert!(update.is_ready());

        let update_starting = StatusUpdate::new_starting();
        assert!(!update_starting.is_ready());
    }
}
