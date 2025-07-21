Speakr Project Rules

Speakr is a privacy-first macOS (desktop) dictation utility built with **Rust**, **Tauri 2**,
and **Leptos**.

## Modules

- `speakr-core` – core functionality (whisper, audio capture, text injection)
- `speakr-tauri` – Tauri backend (hot-key, settings, events)
- `speakr-ui` – frontend (Leptos)

## Build & Commands

- **Check formatting & lints**      `cargo fmt --all -- --check`  +
  `cargo clippy --all-targets --all-features -- -D warnings`
- **Run tests**                     `cargo test --workspace --all-features`
- **Dev run (backend + UI)**        `cargo tauri dev` (triggers Trunk + Tauri)
- **Dev run (UI only)**            `trunk serve`
- **Release build**                 `trunk build --release`  →  `cargo tauri build`

> Always run *fmt → clippy → test* before committing.

---

## Code Style (Rust 2021)

1. Use `rustfmt` defaults (100 char max, trailing commas, imports grouped by std / external /
2. crate).
3. Follow idiomatic naming: `snake_case` for items, `PascalCase` for types, `SCREAMING_SNAKE_CASE`
   for constants.
4. Treat **all Clippy warnings as errors**; allow specific lints only with an inline justification
   comment.
5. Document every public item with rustdoc – include parameters, errors, examples.
6. Prefer borrowing over cloning; use `?` for error propagation; avoid `unwrap/expect` outside
   tests.
7. Use `tracing` for structured logs (no `println!`).
8. Hide unsafe code behind safe wrappers and add `// SAFETY:` explanations.

---

## Testing

- **Test-driven development** – write failing tests first, then implement production code to make
  them pass.
- Unit tests live next to source (`mod tests { … }`) and cover success, error, and edge cases.
- Integration tests go in `tests/` folders per crate (e.g. `speakr-core/tests`).
- Use `tokio::test(flavor = "multi_thread")` for async cases.
- Mock microphone or Shortcut handlers with test doubles; do **not** access real hardware in CI.

---

## Architecture Summary

– Heavy work happens in `speakr-core`; the UI can be closed without disabling dictation.
– Whisper models (GGUF) are downloaded by the user into `models/` and loaded at startup.
– Status events are emitted via `tauri::AppHandle::emit_all` and consumed by the Leptos front-end.
– No network requests – everything is on-device.

---

## Security

1. **No secrets in repo** – credentials, if ever needed, go in user Keychain / env vars.
2. Request only necessary OS permissions:
   - Microphone (audio capture)
   - Accessibility (synthetic keystrokes)
3. Validate model file checksum on load; fail fast if invalid.
4. Keep dependencies up-to-date; run `cargo audit` in CI.

---

## Git Workflow

- Default branch: `main`.  Feature work happens on topic branches.
- Use **Conventional Commits** (feat:, fix:, refactor:, docs:, chore:).
- Never force-push `main`; use `--force-with-lease` only on personal branches.
- CI runs `cargo fmt`, `clippy`, `test`, and `trunk build`.

---

## Configuration

Configuration lives in `src-tauri/tauri.conf.json` and (future) `settings.json` stored via
`tauri-plugin-store`. Add new keys in all places **and** document them in
`docs/IMPLEMENTATION_PLAN.md`.

- Hot-key string (default: `CMD+OPTION+SPACE`)
- Whisper model path
- Max recording seconds

Follow `kebab-case` for JSON keys and prefix booleans with `enable_` / `disable_` for clarity.
