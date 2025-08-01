//! Transcription module for speech-to-text functionality.
//!
//! This module provides the core transcription capabilities for converting
//! audio samples into text using Whisper models. It handles model loading,
//! transcription processing, language detection, and performance optimisation.
//!
//! # Architecture
//!
//! The transcription module is organised into several submodules:
//! - [`engine`] - Core transcription engine and processing
//! - [`models`] - Whisper model management and loading
//! - [`language`] - Language detection and handling
//! - [`performance`] - Performance monitoring and optimisation
//!
//! # Usage
//!
//! ```no_run
//! use speakr_core::transcription::engine::TranscriptionEngine;
//!
//! // Basic transcription workflow will be implemented here
//! ```
//!
//! # Privacy and Security
//!
//! All transcription processing happens locally on-device:
//! - No network requests or external API calls
//! - Audio data never leaves the device
//! - Models are validated before use

/// Core transcription engine and processing functionality.
///
/// Handles the main transcription workflow, including audio preprocessing,
/// model inference, and result post-processing.
pub mod engine;

/// Whisper model management and loading utilities.
///
/// Provides functionality for loading, validating, and managing
/// Whisper GGUF models used for transcription.
pub mod models;

/// Language detection and language-specific handling.
///
/// Handles automatic language detection and language-specific
/// processing optimisations for transcription.
pub mod language;

/// Performance monitoring and optimisation utilities.
///
/// Provides tools for monitoring transcription performance,
/// benchmarking, and applying optimisations.
pub mod performance;
