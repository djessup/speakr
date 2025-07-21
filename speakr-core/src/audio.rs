//! Audio capture implementation.

/// Target sample rate (Hz) required by Whisper.
pub const SAMPLE_RATE_HZ: u32 = 16_000;

/// Number of audio channels (mono).
pub const CHANNELS: u16 = 1;

/// Record audio from the default microphone into a vector of i16 samples.
///
/// `max_duration_secs` – maximum capture duration in seconds.
///
/// NOTE: This is currently unimplemented; tests verifying behaviour should fail
/// until a real implementation lands.
pub fn record_to_vec(_max_duration_secs: u32) -> Vec<i16> {
    unimplemented!("Audio capture not yet implemented")
}