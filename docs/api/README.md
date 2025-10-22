# API Reference

This section documents all public APIs, functions, and UI components across the Speakr workspace.

- speakr-types: Shared types and errors used across backend and UI
- speakr-core: Audio capture, transcription engine, and pipeline helpers
- speakr-tauri: Backend commands exposed via Tauri IPC
- speakr-ui: Leptos components and WASM bootstrap

Conventions used in this reference:
- Code examples prefer concise, copy-pastable snippets.
- All processing is local-only; no network requests are made by production paths.
- Error types are serialisable and safe to pass over IPC.
