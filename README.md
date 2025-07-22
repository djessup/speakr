# Speakr

> Privacy-first macOS dictation utility built with Rust, Tauri 2, and Leptos

Speakr is a local-only voice dictation app for macOS that transcribes your speech and injects the
text directly into any application. No data leaves your device - all processing happens locally
using Whisper models.

## Key Features

- **Privacy-first**: All transcription happens locally - no cloud, no data collection
- **Fast & responsive**: Real-time audio capture and transcription
- **Universal text injection**: Works with any macOS application
- **Local AI**: Uses OpenAI Whisper models (GGUF format) stored on your device
- **Global hotkeys**: Start/stop dictation from anywhere
- **Modern UI**: Clean, accessible interface built with Leptos

## Project Structure

This workspace contains four main crates:

- **[`speakr-core/`](speakr-core/)** - Core functionality (Whisper transcription, audio capture,
  text injection)
- **[`speakr-tauri/`](speakr-tauri/)** - Tauri backend (global hotkeys, settings management,
  event handling)
- **[`speakr-ui/`](speakr-ui/)** - Frontend UI (Leptos components and styling)
- **[`speakr-types/`](speakr-types/)** - Shared types and data structures

## Quick Start

### Prerequisites

- **Rust** (≥1.88 stable) - Install via [`rustup`](https://rustup.rs/)
- **Tauri** - Install with `cargo install tauri-cli --version '^2.0.0' --locked`
- **Trunk** - Install with `cargo install trunk --locked`

### Development Setup

1. **Clone and setup**:

   ```bash
   git clone <repository-url>
   cd speakr
   ```

2. **Install dependencies**:

   ```bash
   cargo build --workspace
   ```

3. **Run development server**:

   ```bash
   # Full app (backend + UI)
   cargo tauri dev

   # UI only (for frontend development)
   trunk serve  # Opens http://localhost:1420
   ```

### Essential Commands

| Command                                                    | Description                   |
| ---------------------------------------------------------- | ----------------------------- |
| `cargo fmt --all -- --check`                               | Check code formatting         |
| `cargo clippy --all-targets --all-features -- -D warnings` | Run linter checks             |
| `cargo test --workspace --all-features`                    | Run all tests                 |
| `cargo tauri dev`                                          | Development server (full app) |
| `trunk serve`                                              | Development server (UI only)  |

🚨 **Always run `fmt → clippy → test` before committing!** 🚨

## 🛠️ Development Workflow

Speakr follows **Test-Driven Development (TDD)**:

1. 🔴 **RED**: Write a failing test first
2. 🟢 **GREEN**: Write minimal code to make the test pass
3. 🔵 **REFACTOR**: Improve code quality while keeping tests green

### Code Quality Standards

- **No `unwrap()` in production** - Use proper error handling with `Result<T, E>`
- **Document public APIs** - Every public function needs rustdoc with examples
- **Test isolation** - Use `tempfile::TempDir` for filesystem tests
- **Tauri commands** - Never mark `#[tauri::command]` functions as `pub`

## 🔐 Privacy & Security

- **Local-only processing**: No network requests, all data stays on your device
- **Minimal permissions**: Only requests microphone and accessibility access
- **Secure settings**: Stored in `~/Library/Application Support/Speakr/settings.json`
- **Model validation**: Whisper models are checksummed on load

## 📖 Documentation

Comprehensive documentation is available via [mdBook](https://github.com/rust-lang/mdBook):

```bash
cargo install mdbook  # Install once
cd docs
mdbook serve -o       # Opens http://localhost:3000
```

Key documentation:

- [Architecture Overview](docs/ARCHITECTURE.md)
- [Implementation Plan](docs/IMPLEMENTATION_PLAN.md)
- [Product Requirements](docs/PRD.md)
- [Development Guide](DEVELOPMENT.md)

## 🏛️ Architecture

Speakr uses a multi-process architecture where heavy transcription work happens in `speakr-core`,
allowing the UI to be closed without disabling dictation functionality.

```text
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   speakr-ui     │    │  speakr-tauri   │    │  speakr-core    │
│   (Leptos)      │◄──►│   (Backend)     │◄──►│ (Transcription) │
│                 │    │                 │    │                 │
│ • Settings UI   │    │ • Global hotkey │    │ • Audio capture │
│ • Status display│    │ • IPC bridge    │    │ • Whisper AI    │
│ • Controls      │    │ • Event system  │    │ • Text injection│
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🤝 Contributing

1. **Fork** the repository
2. **Create** a feature branch following [Conventional Commits](https://conventionalcommits.org/)
3. **Write tests first** (TDD approach)
4. **Implement** your changes
5. **Ensure** all quality checks pass: `cargo fmt && cargo clippy && cargo test`
6. **Submit** a pull request

## 📄 License

[Add your license information here]

---

_Made with cold heartless LLMs for_
_~~privacy-conscious users who want fast, local dictation on macOS~~ myself, it's just for me._
