// ============================================================================
//! Shared types and data structures for the Speakr application.
//!
//! This crate provides the unified type system used across all Speakr components:
//! - Frontend (speakr-ui) and backend (speakr-tauri) consistency
//! - Settings management and persistence structures
//! - Error handling and status reporting types
//! - Model configuration and metadata definitions
//!
//! # Usage
//!
//! All shared types implement `Serialize` and `Deserialize` for seamless
//! communication between frontend and backend components.
//!
//! ```no_run
//! use speakr_types::{AppSettings, AppError, BackendStatus};
//!
//! let settings = AppSettings::default();
//! let status = BackendStatus::new_ready();
//! ```
// ============================================================================

// =========================
// External Imports
// =========================
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ============================================================================
// Constants and Configuration Defaults
// ============================================================================

/// Default schema version for settings migration support.
///
/// Used to handle backwards compatibility when settings format changes.
/// Version 2: Added audio_duration_secs field with validation
pub const DEFAULT_SCHEMA_VERSION: u32 = 2;

/// Default global hotkey combination for dictation activation.
///
/// Uses F1 key to avoid conflicts with macOS system shortcuts.
/// Backtick (`) conflicts with system shortcuts on macOS.
pub const DEFAULT_HOTKEY: &str = "CmdOrCtrl+Alt+F1";

/// Default Whisper model size for transcription.
///
/// Medium provides balanced accuracy and performance for most use cases.
pub const DEFAULT_MODEL_SIZE: &str = "medium";

/// Default auto-launch setting for system startup.
///
/// Disabled by default to respect user privacy preferences.
pub const DEFAULT_AUTO_LAUNCH: bool = false;

/// Minimum allowed audio recording duration in seconds.
///
/// Set to 1 second to ensure meaningful audio capture while preventing
/// accidental zero-duration recordings.
pub const MIN_AUDIO_DURATION_SECS: u32 = 1;

/// Maximum allowed audio recording duration in seconds.
///
/// Set to 30 seconds to balance memory usage and practical dictation needs.
/// Longer recordings may consume excessive memory and processing time.
pub const MAX_AUDIO_DURATION_SECS: u32 = 30;

/// Default audio recording duration in seconds.
///
/// Set to 10 seconds to match current behaviour while providing
/// reasonable balance between capturing complete thoughts and memory usage.
pub const DEFAULT_AUDIO_DURATION_SECS: u32 = 10;

// ============================================================================
// Error Types and Error Handling
// ============================================================================

// --------------------------------------------------------------------------
/// Unified error type for all Tauri backend operations.
///
/// This provides consistent error handling across the entire application,
/// ensuring the frontend can handle all error scenarios uniformly.
///
/// # Error Categories
///
/// - `Settings`: Configuration and persistence errors
/// - `FileSystem`: File I/O and permission errors
/// - `HotKey`: Global hotkey registration and validation errors
/// - `Command`: General Tauri command execution errors
///
/// # Examples
///
/// ```no_run
/// use speakr_types::AppError;
///
/// let error = AppError::Settings("Invalid hotkey format".to_string());
/// assert_eq!(error.to_string(), "Settings error: Invalid hotkey format");
/// ```
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppError {
    /// Settings-related errors including validation and persistence failures.
    #[error("Settings error: {0}")]
    Settings(String),

    /// File system operation errors including I/O and permissions.
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Hot-key registration or validation failures.
    #[error("Hot-key error: {0}")]
    HotKey(String),

    /// Hot-key conflict with existing system or application shortcuts.
    #[error("Hot-key conflict: {0}")]
    HotKeyConflict(String),

    /// Hot-key not found during unregistration attempts.
    #[error("Hot-key not found: {0}")]
    HotKeyNotFound(String),

    /// General command execution errors from Tauri commands.
    #[error("Command error: {0}")]
    Command(String),

    /// Audio capture errors including device access and recording failures.
    #[error("Audio capture error: {0}")]
    AudioCapture(String),

    /// Transcription errors including model loading and processing failures.
    #[error("Transcription error: {0}")]
    Transcription(String),

    /// Text injection errors including permission and injection failures.
    #[error("Text injection error: {0}")]
    TextInjection(String),
}

// --------------------------------------------------------------------------
/// Specific error type for global hotkey operations.
///
/// Provides detailed error information for hotkey-related failures,
/// enabling appropriate user feedback and recovery strategies.
///
/// # Error Scenarios
///
/// - Registration failures due to system limitations
/// - Conflicts with existing shortcuts
/// - Hotkey not found during operations
///
/// # Examples
///
/// ```no_run
/// use speakr_types::HotkeyError;
///
/// let error = HotkeyError::ConflictDetected("Cmd+Space already in use".to_string());
/// ```
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HotkeyError {
    /// Failed to register the hotkey with the system.
    #[error("Failed to register global hot-key: {0}")]
    RegistrationFailed(String),

    /// Hotkey conflicts with existing system or application shortcuts.
    #[error("Hot-key conflict detected: {0}")]
    ConflictDetected(String),

    /// Hotkey not found in the registry during operations.
    #[error("Hot-key not found: {0}")]
    NotFound(String),
}

// ============================================================================
// Configuration and Settings Types
// ============================================================================

// --------------------------------------------------------------------------
/// Configuration for global hot-key settings and behaviour.
///
/// Encapsulates both the hotkey combination and its enabled state,
/// allowing fine-grained control over hotkey functionality.
///
/// # Fields
///
/// - `shortcut`: Tauri-format hotkey string (e.g., "CmdOrCtrl+Alt+F1")
/// - `enabled`: Whether the hotkey is active
///
/// # Examples
///
/// ```no_run
/// use speakr_types::HotkeyConfig;
///
/// let config = HotkeyConfig {
///     shortcut: "CmdOrCtrl+Shift+Space".to_string(),
///     enabled: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HotkeyConfig {
    /// The hotkey combination string in Tauri format.
    pub shortcut: String,
    /// Whether the hotkey is currently enabled.
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

// --------------------------------------------------------------------------
/// Unified application settings - the single source of truth.
///
/// This structure ensures consistency between frontend and backend
/// representations of user preferences and configuration.
///
/// # Schema Evolution
///
/// The `version` field supports settings migration when the format changes.
/// Increment `DEFAULT_SCHEMA_VERSION` when making breaking changes.
///
/// # Fields
///
/// - `version`: Schema version for migration support
/// - `hot_key`: Global hotkey combination string
/// - `model_size`: Selected Whisper model size identifier
/// - `auto_launch`: Whether to start with system
/// - `audio_duration_secs`: Recording duration limit in seconds (1-30)
///
/// # Examples
///
/// ```no_run
/// use speakr_types::AppSettings;
///
/// let settings = AppSettings {
///     version: 1,
///     hot_key: "CmdOrCtrl+Alt+F1".to_string(),
///     model_size: "medium".to_string(),
///     auto_launch: false,
///     audio_duration_secs: 10,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    /// Schema version for migration support.
    #[serde(default = "default_schema_version")]
    pub version: u32,

    /// Global hot-key combination in Tauri format (e.g., "CmdOrCtrl+Alt+F1").
    pub hot_key: String,

    /// Selected model size identifier ("small", "medium", "large").
    pub model_size: String,

    /// Whether to auto-launch the app on system startup.
    pub auto_launch: bool,

    /// Audio recording duration limit in seconds (1-30 seconds).
    #[serde(default = "default_audio_duration_secs")]
    pub audio_duration_secs: u32,
}

/// Provides the default schema version for serde deserialization.
fn default_schema_version() -> u32 {
    DEFAULT_SCHEMA_VERSION
}

/// Provides the default audio duration for serde deserialization.
fn default_audio_duration_secs() -> u32 {
    DEFAULT_AUDIO_DURATION_SECS
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            version: DEFAULT_SCHEMA_VERSION,
            hot_key: DEFAULT_HOTKEY.to_string(),
            model_size: DEFAULT_MODEL_SIZE.to_string(),
            auto_launch: DEFAULT_AUTO_LAUNCH,
            audio_duration_secs: DEFAULT_AUDIO_DURATION_SECS,
        }
    }
}

impl AppSettings {
    /// Validates that the audio duration is within acceptable range.
    ///
    /// # Arguments
    ///
    /// * `duration_secs` - The duration in seconds to validate
    ///
    /// # Returns
    ///
    /// `true` if the duration is between 1 and 30 seconds (inclusive), `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::AppSettings;
    ///
    /// assert!(AppSettings::validate_audio_duration(10));
    /// assert!(!AppSettings::validate_audio_duration(0));
    /// assert!(!AppSettings::validate_audio_duration(31));
    /// ```
    pub fn validate_audio_duration(duration_secs: u32) -> bool {
        (MIN_AUDIO_DURATION_SECS..=MAX_AUDIO_DURATION_SECS).contains(&duration_secs)
    }

    /// Validates all fields in the AppSettings structure.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all settings are valid, `Err(String)` with error message if invalid.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::AppSettings;
    ///
    /// let mut settings = AppSettings::default();
    /// assert!(settings.validate().is_ok());
    ///
    /// settings.audio_duration_secs = 0;
    /// assert!(settings.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if !Self::validate_audio_duration(self.audio_duration_secs) {
            return Err(format!(
                "Invalid audio duration: {} seconds. Must be between {} and {} seconds.",
                self.audio_duration_secs, MIN_AUDIO_DURATION_SECS, MAX_AUDIO_DURATION_SECS
            ));
        }

        // Add other validation checks here as needed
        Ok(())
    }
}

// ============================================================================
// Model Configuration and Metadata
// ============================================================================

// --------------------------------------------------------------------------
/// Available Whisper model sizes for transcription quality and performance.
///
/// Each model size represents a different balance between accuracy, speed,
/// and resource consumption. Larger models provide better accuracy but
/// require more memory and processing time.
///
/// # Model Characteristics
///
/// - `Small`: Fast processing, good for quick notes (39MB)
/// - `Medium`: Balanced accuracy and speed (769MB)
/// - `Large`: Highest accuracy, best for professional use (1550MB)
///
/// # Examples
///
/// ```no_run
/// use speakr_types::ModelSize;
///
/// let size = ModelSize::Medium;
/// assert_eq!(size.display_name(), "Medium (769MB, balanced)");
/// assert_eq!(size.to_string_value(), "medium");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ModelSize {
    /// Small model: 39MB, optimised for speed.
    Small,

    /// Medium model: 769MB, balanced performance (default).
    #[default]
    Medium,

    /// Large model: 1550MB, maximum accuracy.
    Large,
}

impl ModelSize {
    /// Returns the user-friendly display name with size and characteristics.
    ///
    /// # Returns
    ///
    /// A static string describing the model's characteristics.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ModelSize;
    ///
    /// assert_eq!(ModelSize::Small.display_name(), "Small (39MB, fast)");
    /// ```
    pub fn display_name(&self) -> &'static str {
        match self {
            ModelSize::Small => "Small (39MB, fast)",
            ModelSize::Medium => "Medium (769MB, balanced)",
            ModelSize::Large => "Large (1550MB, accurate)",
        }
    }

    /// Returns the string identifier used for settings storage.
    ///
    /// # Returns
    ///
    /// A static string suitable for serialisation and storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ModelSize;
    ///
    /// assert_eq!(ModelSize::Medium.to_string_value(), "medium");
    /// ```
    pub fn to_string_value(&self) -> &'static str {
        match self {
            ModelSize::Small => "small",
            ModelSize::Medium => "medium",
            ModelSize::Large => "large",
        }
    }

    /// Creates a ModelSize from a string identifier.
    ///
    /// # Arguments
    ///
    /// * `s` - The string identifier ("small", "medium", "large")
    ///
    /// # Returns
    ///
    /// The corresponding ModelSize variant, defaulting to Medium for unknown values.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ModelSize;
    ///
    /// assert_eq!(ModelSize::from_string("small"), ModelSize::Small);
    /// assert_eq!(ModelSize::from_string("unknown"), ModelSize::Medium);
    /// ```
    pub fn from_string(s: &str) -> Self {
        match s {
            "small" => ModelSize::Small,
            "medium" => ModelSize::Medium,
            "large" => ModelSize::Large,
            _ => ModelSize::Medium, // Default fallback for unknown values
        }
    }

    /// Returns all available model sizes for UI enumeration.
    ///
    /// # Returns
    ///
    /// A vector containing all ModelSize variants in order.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ModelSize;
    ///
    /// let sizes = ModelSize::all();
    /// assert_eq!(sizes.len(), 3);
    /// ```
    pub fn all() -> Vec<ModelSize> {
        vec![ModelSize::Small, ModelSize::Medium, ModelSize::Large]
    }
}

// --------------------------------------------------------------------------
/// Comprehensive information about a specific Whisper model file.
///
/// Contains all metadata needed for model download, storage, and
/// user interface display including file characteristics and descriptions.
///
/// # Fields
///
/// - `size`: The ModelSize variant
/// - `filename`: Expected filename for the model file
/// - `display_name`: User-friendly name with characteristics
/// - `file_size_mb`: Approximate file size in megabytes
/// - `description`: Detailed usage recommendations
///
/// # Examples
///
/// ```no_run
/// use speakr_types::{ModelInfo, ModelSize};
///
/// let info = ModelInfo::for_size(ModelSize::Medium);
/// assert_eq!(info.filename, "ggml-medium.bin");
/// assert_eq!(info.file_size_mb, 769);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    /// The model size variant.
    pub size: ModelSize,
    /// Expected filename for the model file.
    pub filename: String,
    /// User-friendly display name with characteristics.
    pub display_name: String,
    /// Approximate file size in megabytes.
    pub file_size_mb: u32,
    /// Detailed description of model characteristics and use cases.
    pub description: &'static str,
}

impl ModelInfo {
    /// Creates ModelInfo for a specific model size.
    ///
    /// # Arguments
    ///
    /// * `size` - The ModelSize to create information for
    ///
    /// # Returns
    ///
    /// Complete ModelInfo with all metadata populated.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::{ModelInfo, ModelSize};
    ///
    /// let info = ModelInfo::for_size(ModelSize::Small);
    /// assert_eq!(info.description, "Fast processing, good for quick notes");
    /// ```
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

// ============================================================================
// Status and Service Management
// ============================================================================

// --------------------------------------------------------------------------
/// Status of an individual service component in the backend.
///
/// Provides detailed status information for each service, enabling
/// the frontend to display appropriate UI states and error messages.
///
/// # Status Values
///
/// - `Ready`: Service is operational and available
/// - `Starting`: Service is initialising
/// - `Error(String)`: Service failed with specific error details
/// - `Unavailable`: Service is not available (e.g., permissions)
///
/// # Examples
///
/// ```no_run
/// use speakr_types::ServiceStatus;
///
/// let status = ServiceStatus::Ready;
/// assert!(status.is_ready());
/// assert_eq!(status.display_name(), "Ready");
///
/// let error_status = ServiceStatus::Error("Permission denied".to_string());
/// assert!(!error_status.is_ready());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceStatus {
    /// Service is ready and operational.
    Ready,
    /// Service is currently starting up.
    Starting,
    /// Service encountered an error with details.
    Error(String),
    /// Service is unavailable (e.g., missing permissions).
    Unavailable,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Starting
    }
}

impl ServiceStatus {
    /// Returns the user-friendly display name for the status.
    ///
    /// # Returns
    ///
    /// A static string representing the status for UI display.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ServiceStatus;
    ///
    /// assert_eq!(ServiceStatus::Ready.display_name(), "Ready");
    /// assert_eq!(ServiceStatus::Error("test".to_string()).display_name(), "Error");
    /// ```
    pub fn display_name(&self) -> &str {
        match self {
            ServiceStatus::Ready => "Ready",
            ServiceStatus::Starting => "Starting",
            ServiceStatus::Error(_) => "Error",
            ServiceStatus::Unavailable => "Unavailable",
        }
    }

    /// Returns true if the service is ready for operation.
    ///
    /// # Returns
    ///
    /// Boolean indicating operational readiness.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::ServiceStatus;
    ///
    /// assert!(ServiceStatus::Ready.is_ready());
    /// assert!(!ServiceStatus::Starting.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        matches!(self, ServiceStatus::Ready)
    }
}

// --------------------------------------------------------------------------
/// Overall backend system status combining all service states.
///
/// Provides a comprehensive view of system health, enabling the frontend
/// to determine overall application readiness and display appropriate
/// status indicators to users.
///
/// # Service Components
///
/// - `audio_capture`: Microphone access and recording capability
/// - `transcription`: Whisper model loading and processing
/// - `text_injection`: Keyboard simulation and text insertion
/// - `timestamp`: Unix timestamp in milliseconds for status age
///
/// # Examples
///
/// ```no_run
/// use speakr_types::{BackendStatus, ServiceStatus};
///
/// let status = BackendStatus::new_ready();
/// assert!(status.is_ready());
///
/// let partial_status = BackendStatus {
///     audio_capture: ServiceStatus::Ready,
///     transcription: ServiceStatus::Starting,
///     text_injection: ServiceStatus::Ready,
///     timestamp: 12345,
/// };
/// assert!(!partial_status.is_ready());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendStatus {
    /// Status of audio capture service (microphone access).
    pub audio_capture: ServiceStatus,
    /// Status of transcription service (Whisper model).
    pub transcription: ServiceStatus,
    /// Status of text injection service (keyboard simulation).
    pub text_injection: ServiceStatus,
    /// Unix timestamp in milliseconds when status was created.
    pub timestamp: u64,
}

impl BackendStatus {
    /// Returns true if all services are ready for operation.
    ///
    /// # Returns
    ///
    /// Boolean indicating if the entire system is operational.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::BackendStatus;
    ///
    /// let ready_status = BackendStatus::new_ready();
    /// assert!(ready_status.is_ready());
    /// ```
    pub fn is_ready(&self) -> bool {
        self.audio_capture.is_ready()
            && self.transcription.is_ready()
            && self.text_injection.is_ready()
    }

    /// Creates a new status with all services in starting state.
    ///
    /// # Returns
    ///
    /// BackendStatus with all services marked as Starting.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::BackendStatus;
    ///
    /// let status = BackendStatus::new_starting();
    /// assert!(!status.is_ready());
    /// ```
    pub fn new_starting() -> Self {
        Self {
            audio_capture: ServiceStatus::Starting,
            transcription: ServiceStatus::Starting,
            text_injection: ServiceStatus::Starting,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// Creates a new status with all services ready.
    ///
    /// # Returns
    ///
    /// BackendStatus with all services marked as Ready.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use speakr_types::BackendStatus;
    ///
    /// let status = BackendStatus::new_ready();
    /// assert!(status.is_ready());
    /// ```
    pub fn new_ready() -> Self {
        Self {
            audio_capture: ServiceStatus::Ready,
            transcription: ServiceStatus::Ready,
            text_injection: ServiceStatus::Ready,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

// =========================
// Type Aliases and Exports
// =========================

/// Type alias for status updates sent to the frontend.
///
/// Provides semantic clarity when used specifically for status broadcasting
/// while maintaining the same underlying structure as BackendStatus.
pub type StatusUpdate = BackendStatus;

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================
    // Settings and Configuration Tests
    // =========================

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.version, DEFAULT_SCHEMA_VERSION);
        assert_eq!(settings.hot_key, DEFAULT_HOTKEY);
        assert_eq!(settings.model_size, DEFAULT_MODEL_SIZE);
        assert_eq!(settings.auto_launch, DEFAULT_AUTO_LAUNCH);
        assert_eq!(settings.audio_duration_secs, DEFAULT_AUDIO_DURATION_SECS);
    }

    #[test]
    fn test_audio_duration_validation_valid_range() {
        assert!(AppSettings::validate_audio_duration(
            MIN_AUDIO_DURATION_SECS
        ));
        assert!(AppSettings::validate_audio_duration(
            DEFAULT_AUDIO_DURATION_SECS
        ));
        assert!(AppSettings::validate_audio_duration(
            MAX_AUDIO_DURATION_SECS
        ));
    }

    #[test]
    fn test_audio_duration_validation_invalid_range() {
        assert!(!AppSettings::validate_audio_duration(
            MIN_AUDIO_DURATION_SECS - 1
        ));
        assert!(!AppSettings::validate_audio_duration(
            MAX_AUDIO_DURATION_SECS + 1
        ));
        assert!(!AppSettings::validate_audio_duration(100));
    }

    #[test]
    fn test_audio_duration_constants_are_consistent() {
        assert!(MIN_AUDIO_DURATION_SECS <= DEFAULT_AUDIO_DURATION_SECS);
        assert!(DEFAULT_AUDIO_DURATION_SECS <= MAX_AUDIO_DURATION_SECS);
        assert!(AppSettings::validate_audio_duration(
            DEFAULT_AUDIO_DURATION_SECS
        ));
    }

    #[test]
    fn test_audio_duration_validation_uses_constants() {
        // Test that validation uses the defined constants
        assert!(AppSettings::validate_audio_duration(
            MIN_AUDIO_DURATION_SECS
        ));
        assert!(AppSettings::validate_audio_duration(
            MAX_AUDIO_DURATION_SECS
        ));
        assert!(!AppSettings::validate_audio_duration(
            MIN_AUDIO_DURATION_SECS - 1
        ));
        assert!(!AppSettings::validate_audio_duration(
            MAX_AUDIO_DURATION_SECS + 1
        ));
    }

    #[test]
    fn test_app_settings_validate_method() {
        // Test that AppSettings has a validate method that checks audio duration
        let mut settings = AppSettings::default();
        assert!(settings.validate().is_ok());

        settings.audio_duration_secs = 0;
        assert!(settings.validate().is_err());

        settings.audio_duration_secs = 31;
        assert!(settings.validate().is_err());

        settings.audio_duration_secs = 15;
        assert!(settings.validate().is_ok());
    }

    #[test]
    fn test_hotkey_config_default() {
        let config = HotkeyConfig::default();
        assert_eq!(config.shortcut, DEFAULT_HOTKEY);
        assert!(config.enabled);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = AppSettings::default();
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    // =========================
    // Model Configuration Tests
    // =========================

    #[test]
    fn test_model_size_conversions() {
        let medium = ModelSize::Medium;
        assert_eq!(medium.to_string_value(), "medium");
        assert_eq!(medium.display_name(), "Medium (769MB, balanced)");

        let from_str = ModelSize::from_string("small");
        assert_eq!(from_str, ModelSize::Small);
    }

    #[test]
    fn test_model_size_from_string_fallback() {
        let unknown = ModelSize::from_string("unknown_size");
        assert_eq!(unknown, ModelSize::Medium);
    }

    #[test]
    fn test_model_size_all() {
        let all_sizes = ModelSize::all();
        assert_eq!(all_sizes.len(), 3);
        assert!(all_sizes.contains(&ModelSize::Small));
        assert!(all_sizes.contains(&ModelSize::Medium));
        assert!(all_sizes.contains(&ModelSize::Large));
    }

    #[test]
    fn test_model_info() {
        let info = ModelInfo::for_size(ModelSize::Small);
        assert_eq!(info.filename, "ggml-small.bin");
        assert_eq!(info.file_size_mb, 39);
        assert_eq!(info.size, ModelSize::Small);
        assert_eq!(info.description, "Fast processing, good for quick notes");
    }

    // =========================
    // Service Status Tests
    // =========================

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
    fn test_service_status_is_ready() {
        assert!(ServiceStatus::Ready.is_ready());
        assert!(!ServiceStatus::Starting.is_ready());
        assert!(!ServiceStatus::Error("error".to_string()).is_ready());
        assert!(!ServiceStatus::Unavailable.is_ready());
    }

    // =========================
    // Backend Status Tests
    // =========================

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

// ===========================================================================
