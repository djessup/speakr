# Speakr Product Overview

Speakr is a **privacy-first macOS dictation utility** built with Rust, Tauri 2, and Leptos. It
enables users to transcribe speech and inject text directly into any application using global
hotkeys.

## Core Features

- **Privacy-first**: All transcription happens locally using Whisper models - no cloud, no data
  collection
- **Fast & responsive**: Real-time audio capture with <3s end-to-end latency target
- **Universal text injection**: Works with any macOS application via synthetic keystrokes
- **Global hotkeys**: Start/stop dictation from anywhere (default: `CmdOrCtrl+Alt+Space`)
- **Modern UI**: Clean, accessible interface built with Leptos

## Key Requirements

- **Latency**: ≤3s end-to-end for 5-second recordings on Apple Silicon
- **Privacy**: 100% offline operation, no external network calls
- **Footprint**: Binary ≤20MB, RAM ≤400MB including model
- **Reliability**: >99.5% crash-free sessions
- **Compatibility**: macOS 13+ (Intel and Apple Silicon)

## Architecture

Multi-process architecture where heavy transcription work happens in `speakr-core`, allowing the UI
to be closed without disabling dictation functionality.

## Target Users

- **Developers**: Insert comments/code quickly without losing keyboard context
- **Writers**: Draft snippets into any text editor without toggling apps
- **Privacy-conscious users**: Dictate confidential material offline
- **Accessibility users**: Replace or augment typing due to RSI
