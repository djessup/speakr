//! Audio processing module.
//!
//! This module provides audio file operations and recording functionality
//! for the Speakr application, including WAV file creation, timestamp
//! generation, and audio recording utilities.

pub mod files;
pub mod recording;

// Re-export public interfaces
pub use files::{generate_audio_filename_with_timestamp, save_audio_samples_to_wav_file};
pub use recording::{debug_record_audio_to_file, debug_record_real_audio_to_file};
