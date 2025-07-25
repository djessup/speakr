# Technology Stack & Build System

## Core Technologies

- **Language**: Rust (stable channel, ≥1.88)
- **Frontend**: Leptos (WebAssembly/WASM)
- **Backend**: Tauri 2.x
- **Audio**: cpal (cross-platform audio library)
- **Speech Recognition**: whisper-rs with GGUF models
- **Text Injection**: enigo (synthetic keystrokes)

## Development Environment

- **Package Manager**: Nix with devenv for reproducible environments
- **Build Tool**: Cargo workspace with 4 crates
- **Frontend Bundler**: Trunk for WASM compilation
- **Documentation**: mdBook with plugins (mermaid, admonish, pagetoc)

## Key Dependencies

- `tauri` - Desktop app framework
- `leptos` - Reactive web framework (compiled to WASM)
- `tokio` - Async runtime
- `serde` - Serialization
- `tracing` - Structured logging
- `tempfile` - Test isolation

## Common Commands

### Development

```bash
# Start full development server (frontend + backend)
cargo tauri dev

# Frontend only (Leptos on port 1420)
trunk serve --port 1420

# Backend only
cargo tauri dev
```

### Code Quality (Required before commits)

```bash
# Format code
cargo fmt --all

# Lint with strict settings
cargo clippy --all-targets --all-features --workspace -- -D warnings

# Run all tests
cargo test --workspace --all-features
```

### Build & Release

```bash
# Production build
cargo tauri build

# Frontend build only
trunk build --release

# Documentation
mdbook serve --open
```

### Development Scripts (via devenv)

```bash
dev          # Start full app
dev-ui       # Frontend only
build        # Production build
format       # Format code
lint         # Run clippy
test         # Run tests
docs-serve   # Start docs server
```

## Build Configuration

- **Rust toolchain**: Stable with `wasm32-unknown-unknown` target
- **Code formatting**: 100 char max width, trailing commas, organized imports
- **Pre-commit hooks**: Format, clippy, and tests run automatically
- **Target platforms**: macOS (Intel + Apple Silicon), Windows/Linux planned
