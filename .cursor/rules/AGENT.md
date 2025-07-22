<project_summary>
Speakr is a privacy-first macOS (desktop) dictation utility built with Rust, Tauri 2, and Leptos.
</project_summary>

<project_structure>

- `speakr-core` – core functionality (whisper, audio capture, text injection)
- `speakr-tauri` – Tauri backend (hot-key, settings, events)
- `speakr-ui` – frontend (Leptos)
- `speakr-types` – shared types
</project_structure>

<build_and_commands>
| Command                                                    | Description                                |
| ---------------------------------------------------------- | ------------------------------------------ |
| `cargo fmt --all -- --check`                               | Check formatting (fmt)                     |
| `cargo clippy --all-targets --all-features -- -D warnings` | Check lints (clippy)                       |
| `cargo test --workspace --all-features`                    | Run tests (test)                           |
| `cargo tauri dev`                                          | Dev run (backend + UI) (dev)               |
| `trunk serve`                                              | Dev run (UI only: <http://localhost:1420>) |

🚨 ALWAYS run *fmt → clippy → test* before committing! 🚨
</build_and_commands>

<code_style>

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
9. **IMPORTANT**: Functions with `#[tauri::command]` must NOT be marked as `pub` - this causes
   duplicate macro generation errors. Use `async fn command_name()` not `pub async fn
   command_name()`.
</code_style>

<test_driven_development>

- **Test-driven development** – write failing tests FIRST, then implement production code to make
  them pass.
- Unit tests live next to source (`mod tests { … }`) and cover success, error, and edge cases.
- Integration tests go in `tests/` folders per crate (e.g. `speakr-core/tests`).
- Use `tokio::test(flavor = "multi_thread")` for async cases.
- Mock microphone or Shortcut handlers with test doubles; do **not** access real hardware in CI.
</test_driven_development>

<architecture_summary>
– Heavy work happens in `speakr-core`; the UI can be closed without disabling dictation.
– Whisper models (GGUF) are downloaded by the user into `models/` and loaded at startup.
– Status events are emitted via `tauri::AppHandle::emit_all` and consumed by the Leptos front-end.
– No network requests – everything is on-device.
</architecture_summary>

<security>
- **Local-only processing**: No data leaves the device
- **Minimal permissions**: Only microphone + accessibility
- **Settings location**: `$HOME/Library/Application Support/Speakr/settings.json`
- **No secrets in repo** – credentials, if ever needed, go in user Keychain / env vars.
- **Request only necessary OS permissions**:
  - Microphone (audio capture)
  - Accessibility (synthetic keystrokes)
- **Validate model file checksum on load**: fail fast if invalid.
- **Keep dependencies up-to-date**: run `cargo audit` in CI.
</security>

<git_workflow>

- Default branch: `main`.  Feature work happens on topic branches.
- Use **Conventional Commits** (feat:, fix:, refactor:, docs:, chore:, etc).
- Never force-push `main`; use `--force-with-lease` only on personal branches.
- CI runs `cargo fmt`, `clippy`, `test`, and `trunk build`.
</git_workflow>

<tauri>
- Use `cargo tauri` to manage plugins, register plugins in code.
- Define backend commands with `#[tauri::command]` (never `pub`), and enforce strict security and
  IPC patterns.
- Use Context7 tool to access accurate Tauri documentation, code examples, and best practices.

> Read [tauri.mdc](/.cursor/rules/tauri.mdc) for details.
</tauri>

<critical_tdd_reminder>

- Use TDD for all code changes: 🔴 RED: Test fails - function doesn't exist, 🟢 GREEN: Minimal
  implementation to pass test, 🔵 REFACTOR: Integrate into production code, all tests still pass
- Write 🔴 RED: Test tests FIRST.
- Implement 🟢 GREEN: production code to make tests pass.
- Refactor 🔵 REFACTOR: to improve code quality.
- Repeat until done.
</critical_tdd_reminder>
