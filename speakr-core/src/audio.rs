//! Audio capture implementation for Speakr.
//!
//! This module provides audio recording functionality that meets Whisper's requirements:
//! - 16 kHz sample rate
//! - Mono audio (1 channel)
//! - 16-bit samples
//! - In-memory buffering only (no disk writes)
//! - Configurable duration limits (1-30 seconds)

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use thiserror::Error;
use tokio::sync::oneshot;
use tracing::{debug, error, info, instrument, warn};

/// Target sample rate (Hz) required by Whisper.
pub const SAMPLE_RATE_HZ: u32 = 16_000;

/// Number of audio channels (mono).
pub const CHANNELS: u16 = 1;

/// Default maximum recording duration in seconds.
pub const DEFAULT_MAX_DURATION_SECS: u32 = 10;

/// Maximum allowed recording duration in seconds.
pub const MAX_ALLOWED_DURATION_SECS: u32 = 30;

/// Bit depth for audio samples (16-bit).
pub const SAMPLE_BIT_DEPTH: u16 = 16;

/// Information about an audio input device.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioDevice {
    /// Unique identifier for the device
    pub id: String,
    /// Human-readable name of the device
    pub name: String,
    /// Whether this is the system default input device
    pub is_default: bool,
}

/// Errors that can occur during audio capture.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum AudioCaptureError {
    #[error("Microphone not available")]
    MicrophoneNotAvailable,

    #[error("Microphone permission denied")]
    PermissionDenied,

    #[error("Audio device error: {0}")]
    DeviceError(String),

    #[error("Audio stream error: {0}")]
    StreamError(String),

    #[error("Recording is already in progress")]
    RecordingInProgress,

    #[error("No recording is active")]
    NoActiveRecording,

    #[error("Recording initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Invalid recording configuration: {0}")]
    InvalidConfiguration(String),
}

/// Configuration for audio recording sessions.
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    max_duration_secs: u32,
}

impl RecordingConfig {
    /// Creates a new recording configuration with the specified duration.
    ///
    /// # Arguments
    ///
    /// * `max_duration_secs` - Maximum recording duration in seconds (1-30)
    ///
    /// # Returns
    ///
    /// A recording configuration, with duration clamped to valid range
    pub fn new(max_duration_secs: u32) -> Self {
        let clamped_duration = max_duration_secs.clamp(1, MAX_ALLOWED_DURATION_SECS);

        if clamped_duration != max_duration_secs {
            warn!(
                requested = max_duration_secs,
                clamped = clamped_duration,
                "Recording duration clamped to valid range (1-30 seconds)"
            );
        }

        Self {
            max_duration_secs: clamped_duration,
        }
    }

    /// Returns the maximum recording duration in seconds.
    pub fn max_duration_secs(&self) -> u32 {
        self.max_duration_secs
    }

    /// Returns the maximum number of samples for this configuration.
    pub fn max_samples(&self) -> usize {
        (self.max_duration_secs as usize) * (SAMPLE_RATE_HZ as usize)
    }
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            max_duration_secs: DEFAULT_MAX_DURATION_SECS,
        }
    }
}

/// Result of a recording operation.
#[derive(Debug, Clone)]
pub enum RecordingResult {
    /// Recording completed successfully before reaching time limit.
    Success(Vec<i16>),

    /// Recording was stopped automatically when reaching the duration limit.
    StoppedAtLimit(Vec<i16>),

    /// Recording was manually stopped before reaching the duration limit.
    ManuallyStoppedEarly(Vec<i16>),
}

impl RecordingResult {
    /// Extracts the audio samples regardless of how recording ended.
    pub fn samples(self) -> Vec<i16> {
        match self {
            RecordingResult::Success(samples)
            | RecordingResult::StoppedAtLimit(samples)
            | RecordingResult::ManuallyStoppedEarly(samples) => samples,
        }
    }
}

/// Audio settings management (placeholder for future settings persistence).
#[derive(Debug, Clone)]
pub struct AudioSettings {
    default_config: RecordingConfig,
}

impl AudioSettings {
    /// Creates new audio settings with default configuration.
    pub fn new() -> Self {
        Self {
            default_config: RecordingConfig::default(),
        }
    }

    /// Returns the default recording configuration.
    pub fn default_config(&self) -> &RecordingConfig {
        &self.default_config
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait abstraction for audio system to enable testing and mocking.
pub trait AudioSystem: Send + Sync {
    /// Start recording audio with the given configuration.
    fn start_recording(
        &self,
        config: &RecordingConfig,
    ) -> Result<Box<dyn AudioStream>, AudioCaptureError>;

    /// List all available audio input devices.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `AudioDevice` or an error
    ///
    /// # Errors
    ///
    /// Returns `AudioCaptureError::MicrophoneNotAvailable` if no input devices are found,
    /// or `AudioCaptureError::DeviceError` if device enumeration fails.
    fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioCaptureError>;
}

/// Trait for audio stream management.
pub trait AudioStream: Send + Sync {
    /// Get samples that have been recorded so far.
    fn get_samples(&self) -> Vec<i16>;

    /// Stop the audio stream.
    fn stop(&self);

    /// Check if the stream is still active.
    fn is_active(&self) -> bool;
}

/// Real audio stream implementation.
pub struct CpalAudioStream {
    samples: Arc<Mutex<Vec<i16>>>,
    is_recording: Arc<AtomicBool>,
}

// SAFETY: CpalAudioStream only contains thread-safe types (Arc<Mutex<_>> and Arc<AtomicBool>)
unsafe impl Send for CpalAudioStream {}
unsafe impl Sync for CpalAudioStream {}

impl AudioStream for CpalAudioStream {
    fn get_samples(&self) -> Vec<i16> {
        let samples_guard = self.samples.lock().unwrap();
        samples_guard.clone()
    }

    fn stop(&self) {
        self.is_recording.store(false, Ordering::Release);
    }

    fn is_active(&self) -> bool {
        self.is_recording.load(Ordering::Acquire)
    }
}

/// Real audio system implementation using cpal.
pub struct CpalAudioSystem {
    host: cpal::Host,
}

impl CpalAudioSystem {
    /// Create a new cpal audio system.
    pub fn new() -> Result<Self, AudioCaptureError> {
        let host = cpal::default_host();

        // Verify we can access an input device during initialization
        let _device = host.default_input_device().ok_or_else(|| {
            error!("No default input device available");
            AudioCaptureError::MicrophoneNotAvailable
        })?;

        Ok(Self { host })
    }
}

impl AudioSystem for CpalAudioSystem {
    fn start_recording(
        &self,
        _config: &RecordingConfig,
    ) -> Result<Box<dyn AudioStream>, AudioCaptureError> {
        // Get the default input device
        let device = self
            .host
            .default_input_device()
            .ok_or(AudioCaptureError::MicrophoneNotAvailable)?;

        // Get supported input configs
        let supported_config = device
            .default_input_config()
            .map_err(|e| AudioCaptureError::DeviceError(e.to_string()))?;

        // Create stream config with our requirements
        let stream_config = StreamConfig {
            channels: CHANNELS,
            sample_rate: cpal::SampleRate(SAMPLE_RATE_HZ),
            buffer_size: cpal::BufferSize::Default,
        };

        // Create shared state for the recording
        let samples = Arc::new(Mutex::new(Vec::new()));
        let is_recording = Arc::new(AtomicBool::new(true));

        // Clone for the stream callback
        let samples_clone = Arc::clone(&samples);
        let is_recording_clone = Arc::clone(&is_recording);

        // Create the input stream based on sample format
        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => {
                device
                    .build_input_stream(
                        &stream_config,
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            if is_recording_clone.load(Ordering::Acquire) {
                                let mut samples_guard = samples_clone.lock().unwrap();
                                for &sample in data {
                                    // Convert f32 to i16 and store
                                    let sample_i16 = (sample * (i16::MAX as f32)) as i16;
                                    samples_guard.push(sample_i16);
                                }
                            }
                        },
                        |err| error!("Audio stream error: {}", err),
                        None,
                    )
                    .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?
            }
            SampleFormat::I16 => device
                .build_input_stream(
                    &stream_config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        if is_recording_clone.load(Ordering::Acquire) {
                            let mut samples_guard = samples_clone.lock().unwrap();
                            samples_guard.extend_from_slice(data);
                        }
                    },
                    |err| error!("Audio stream error: {}", err),
                    None,
                )
                .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?,
            SampleFormat::U16 => {
                device
                    .build_input_stream(
                        &stream_config,
                        move |data: &[u16], _: &cpal::InputCallbackInfo| {
                            if is_recording_clone.load(Ordering::Acquire) {
                                let mut samples_guard = samples_clone.lock().unwrap();
                                for &sample in data {
                                    // Convert u16 to i16
                                    let sample_i16 = ((sample as i32) - 32768) as i16;
                                    samples_guard.push(sample_i16);
                                }
                            }
                        },
                        |err| error!("Audio stream error: {}", err),
                        None,
                    )
                    .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?
            }
            format => {
                return Err(AudioCaptureError::DeviceError(format!(
                    "Unsupported sample format: {format:?}"
                )));
            }
        };

        // Start the stream
        stream
            .play()
            .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?;

        // Keep the stream alive by leaking it - this is necessary because cpal streams
        // are not Send/Sync and we can't store them in our thread-safe wrapper.
        // TODO: allow stream to be stopped
        // In a production system, you'd want a more sophisticated approach to stream lifecycle.
        std::mem::forget(stream);

        Ok(Box::new(CpalAudioStream {
            samples,
            is_recording,
        }))
    }

    fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioCaptureError> {
        let devices = self
            .host
            .input_devices()
            .map_err(|e| AudioCaptureError::DeviceError(e.to_string()))?;

        let default_device = self.host.default_input_device();
        let default_device_name = default_device
            .as_ref()
            .and_then(|d| d.name().ok())
            .unwrap_or_default();

        let mut audio_devices = Vec::new();
        for device in devices {
            let name = device.name().map_err(|e| {
                AudioCaptureError::DeviceError(format!("Could not get device name: {e}"))
            })?;

            let is_default = name == default_device_name;

            // Use device name as ID since cpal doesn't provide device IDs
            let id = name.clone();

            audio_devices.push(AudioDevice {
                id,
                name,
                is_default,
            });
        }

        if audio_devices.is_empty() {
            return Err(AudioCaptureError::MicrophoneNotAvailable);
        }

        Ok(audio_devices)
    }
}

/// Internal recording state.
struct RecordingState {
    stream: Box<dyn AudioStream>,
    start_time: Instant,
    config: RecordingConfig,
    stop_sender: Option<oneshot::Sender<()>>,
}

impl std::fmt::Debug for RecordingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecordingState")
            .field("stream", &"<AudioStream>")
            .field("start_time", &self.start_time)
            .field("config", &self.config)
            .field("stop_sender", &"<Option<oneshot::Sender<()>>>")
            .finish()
    }
}

/// Main audio recorder that handles microphone capture.
pub struct AudioRecorder {
    state: Arc<Mutex<Option<RecordingState>>>,
    audio_system: Box<dyn AudioSystem>,
}

impl std::fmt::Debug for AudioRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioRecorder")
            .field("state", &"<RecordingState>")
            .field("audio_system", &"<AudioSystem>")
            .finish()
    }
}

impl AudioRecorder {
    /// Creates a new audio recorder with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Recording configuration to use
    ///
    /// # Returns
    ///
    /// A `Result` containing the new recorder or an error
    ///
    /// # Errors
    ///
    /// Returns `AudioCaptureError::InitializationFailed` if the audio system
    /// cannot be initialized or no suitable input device is found.
    #[instrument(level = "debug")]
    pub async fn new(config: RecordingConfig) -> Result<Self, AudioCaptureError> {
        debug!(
            max_duration = config.max_duration_secs,
            "Creating new AudioRecorder"
        );

        let audio_system = Box::new(CpalAudioSystem::new()?);

        info!("AudioRecorder initialized successfully");

        Ok(Self {
            state: Arc::new(Mutex::new(None)),
            audio_system,
        })
    }

    /// Creates a new audio recorder with a custom audio system (for testing).
    /// This constructor allows dependency injection for testing purposes.
    pub fn with_audio_system(audio_system: Box<dyn AudioSystem>) -> Self {
        Self {
            state: Arc::new(Mutex::new(None)),
            audio_system,
        }
    }

    /// Starts recording audio from the default microphone.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns `AudioCaptureError::RecordingInProgress` if a recording is already active,
    /// or other errors if the recording cannot be started.
    #[instrument(level = "info", skip(self))]
    pub async fn start_recording(&self) -> Result<(), AudioCaptureError> {
        let start_time = Instant::now();

        // Check if recording is already in progress
        {
            let state_guard = self.state.lock().unwrap();
            if state_guard.is_some() {
                return Err(AudioCaptureError::RecordingInProgress);
            }
        }

        debug!("Starting audio recording");

        let config = RecordingConfig::default();

        // Start the audio stream
        let stream = self.audio_system.start_recording(&config)?;

        let (stop_sender, _stop_receiver) = oneshot::channel::<()>();

        // Store the recording state
        {
            let mut state_guard = self.state.lock().unwrap();
            *state_guard = Some(RecordingState {
                stream,
                start_time,
                config: config.clone(),
                stop_sender: Some(stop_sender),
            });
        }

        // Spawn timeout task
        let state_for_timeout = Arc::clone(&self.state);
        let timeout_duration = Duration::from_secs(config.max_duration_secs as u64);

        tokio::spawn(async move {
            tokio::time::sleep(timeout_duration).await;

            // Stop the stream when timeout is reached
            if let Some(state) = state_for_timeout.lock().unwrap().as_ref() {
                state.stream.stop();
                debug!("Recording stopped due to timeout");
            }
        });

        let initialization_time = start_time.elapsed();
        info!(
            duration_ms = initialization_time.as_millis(),
            "Recording started successfully"
        );

        Ok(())
    }

    /// Stops the current recording and returns the captured audio samples.
    ///
    /// # Returns
    ///
    /// A `Result` containing the recording result or an error
    ///
    /// # Errors
    ///
    /// Returns `AudioCaptureError::NoActiveRecording` if no recording is currently active.
    #[instrument(level = "info", skip(self))]
    pub async fn stop_recording(&self) -> Result<RecordingResult, AudioCaptureError> {
        let recording_state = {
            let mut state_guard = self.state.lock().unwrap();
            state_guard.take()
        };

        let mut state = recording_state.ok_or(AudioCaptureError::NoActiveRecording)?;

        // Signal stop if sender is still available
        if let Some(stop_sender) = state.stop_sender.take() {
            let _ = stop_sender.send(()); // Ignore error if receiver is already dropped
        }

        // Stop the stream
        state.stream.stop();

        // Wait a brief moment for any in-flight samples to be processed
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Extract the samples
        let samples = state.stream.get_samples();

        let recording_duration = state.start_time.elapsed();
        let expected_duration = Duration::from_secs(state.config.max_duration_secs as u64);

        info!(
            sample_count = samples.len(),
            duration_ms = recording_duration.as_millis(),
            "Recording stopped"
        );

        // Determine result type based on how recording ended
        let result = if recording_duration >= expected_duration - Duration::from_millis(50) {
            // Close to time limit, assume it was stopped by timeout
            RecordingResult::StoppedAtLimit(samples)
        } else if recording_duration < expected_duration - Duration::from_millis(100) {
            // Significantly before time limit, assume manual stop
            RecordingResult::ManuallyStoppedEarly(samples)
        } else {
            // Default case
            RecordingResult::Success(samples)
        };

        Ok(result)
    }

    /// Returns whether a recording is currently in progress.
    pub fn is_recording(&self) -> bool {
        let state_guard = self.state.lock().unwrap();
        match state_guard.as_ref() {
            Some(state) => state.stream.is_active(),
            None => false,
        }
    }

    /// Lists all available audio input devices.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `AudioDevice` or an error
    ///
    /// # Errors
    ///
    /// Returns `AudioCaptureError::MicrophoneNotAvailable` if no input devices are found,
    /// or `AudioCaptureError::DeviceError` if device enumeration fails.
    #[instrument(level = "debug", skip(self))]
    pub async fn list_input_devices(&self) -> Result<Vec<AudioDevice>, AudioCaptureError> {
        debug!("Listing available input devices");
        let devices = self.audio_system.list_input_devices()?;
        info!(device_count = devices.len(), "Found input devices");
        Ok(devices)
    }
}

/// Record audio from the default microphone into a vector of i16 samples.
///
/// This is a legacy function maintained for backward compatibility.
/// New code should use `AudioRecorder` for more control and better error handling.
///
/// # Arguments
///
/// * `max_duration_secs` – Maximum capture duration in seconds (1-30)
///
/// # Returns
///
/// A vector of 16-bit audio samples
///
/// # Panics
///
/// Panics if audio capture fails. Use `AudioRecorder` for proper error handling.
///
/// # Examples
///
/// ```no_run
/// use speakr_core::audio::record_to_vec;
///
/// let samples = record_to_vec(5); // Record for 5 seconds
/// println!("Captured {} samples", samples.len());
/// ```
#[deprecated(note = "Use AudioRecorder for better error handling and async support")]
pub fn record_to_vec(max_duration_secs: u32) -> Vec<i16> {
    // For backward compatibility, use the new implementation
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create async runtime");

    runtime.block_on(async {
        let config = RecordingConfig::new(max_duration_secs);
        let recorder = AudioRecorder::new(config)
            .await
            .expect("Failed to create recorder");

        recorder
            .start_recording()
            .await
            .expect("Failed to start recording");

        // Wait for the full duration
        tokio::time::sleep(Duration::from_secs(max_duration_secs as u64)).await;

        let result = recorder
            .stop_recording()
            .await
            .expect("Failed to stop recording");
        result.samples()
    })
}
