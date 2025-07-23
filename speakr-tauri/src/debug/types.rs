//! Debug types and data structures.

use serde::{Deserialize, Serialize};
use speakr_core::audio::AudioRecorder;

/// Debug log levels for categorizing log messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebugLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// A debug log message with timestamp and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLogMessage {
    pub timestamp: String,
    pub level: DebugLogLevel,
    pub target: String,
    pub message: String,
}

impl DebugLogMessage {
    /// Creates a new debug log message with current timestamp
    pub fn new(level: DebugLogLevel, target: &str, message: &str) -> Self {
        Self {
            timestamp: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            level,
            target: target.to_string(),
            message: message.to_string(),
        }
    }
}

/// Shared state for debug recording session
#[derive(Debug)]
pub(crate) struct DebugRecordingState {
    pub recorder: Option<AudioRecorder>,
    pub start_time: Option<std::time::Instant>,
}
