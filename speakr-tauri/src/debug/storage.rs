//! Debug message storage and management.

use crate::debug::types::{DebugLogLevel, DebugLogMessage, DebugRecordingState};
use std::collections::VecDeque;
use std::sync::{Arc, LazyLock, Mutex};

/// Global storage for debug log messages with capacity limit
pub(crate) static DEBUG_LOG_MESSAGES: LazyLock<Arc<Mutex<VecDeque<DebugLogMessage>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(VecDeque::with_capacity(1000))));

/// Global state for debug recording sessions
pub(crate) static DEBUG_RECORDING_STATE: LazyLock<Arc<Mutex<DebugRecordingState>>> =
    LazyLock::new(|| {
        Arc::new(Mutex::new(DebugRecordingState {
            recorder: None,
            start_time: None,
        }))
    });

/// Adds a debug log message to the global storage
///
/// # Arguments
///
/// * `level` - The log level
/// * `target` - The source component (e.g., "speakr-debug", "speakr-core")
/// * `message` - The log message content
///
/// # Note
///
/// Messages are stored in a circular buffer with a maximum of 1000 entries.
/// Oldest messages are automatically removed when capacity is exceeded.
pub fn add_debug_log(level: DebugLogLevel, target: &str, message: &str) {
    if let Ok(mut logs) = DEBUG_LOG_MESSAGES.lock() {
        logs.push_back(DebugLogMessage::new(level, target, message));

        // Keep only the last 1000 messages
        while logs.len() > 1000 {
            logs.pop_front();
        }
    }
}
