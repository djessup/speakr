# INIT-01: Project Scaffold & Initial Structure

Define the baseline repository layout, build tooling, and development workflows for Speakr.

## Requirement

1. **Workspace Layout (multi-crate)**
   - `speakr-core/` – pure Rust library (record → transcribe → inject).
   - `speakr-tauri/` – Tauri desktop shell; contains `src-tauri/` and embeds Leptos frontend by default.
   - `speakr-ui/` – optional standalone Leptos UI crate (only if the UI is fully separated).
   - `models/` – user-downloaded GGUF Whisper models (git-ignored).
   - `docs/` – architecture, PRD, and spec docs (this folder).
   - `nix/` – flakes, overlays, `devenv.nix`, CI helpers.
   - `scripts/` – one-off dev scripts (lint, release, etc.).
   - Root-level `Cargo.toml` / `Cargo.lock` defining a `[workspace]` with `members`.
2. **Build Tooling**
   - Use **Cargo workspace** to manage crates and enable incremental rebuilds.
   - Root-level **Nix flake** + `devenv.nix` for reproducible shells.
   - `Trunk.toml` (in `speakr-tauri/`) bundles static assets for the WebView.
3. **CI / CD**
   - GitHub Actions workflow for: lint (`rustfmt`, `clippy`), test, macOS build, docs build.
   - Release workflow signs and notarises macOS DMG.
4. **Linters & Hooks**
   - Pre-commit config: `rustfmt`, `markdownlint`, `shellcheck`, `nixpkgs-fmt`.
5. **Documentation Site**
   - mdBook in `docs/book/` published via GitHub Pages.
6. **Version Control Hygiene**
   - `.gitignore` tracks target, model files, and local config overrides.

## Rationale

A consistent scaffold accelerates onboarding, enforces build reproducibility, and aligns with the
 project’s privacy-first & cross-platform goals.

## Acceptance Criteria

- [ ] Fresh clone followed by `devenv shell` (or `devenv up`) yields a working shell with `cargo`,
- [ ]  `tauri`, and `mdbook` available.
- [ ] `cargo test` passes with placeholder tests.
- [ ] `npm run tauri dev` (via Trunk) launches stub window.
- [ ] GitHub Actions green on lint + test.
- [ ] `mdbook serve` builds documentation without errors.

## Migration Steps (from mono-crate → multi-crate)

1. **Create workspace file**

   ```bash
   # At repo root
   echo "[workspace]\nmembers = [ \"speakr-core\", \"speakr-tauri\", \"speakr-ui\" ]" > Cargo.toml
   ```

2. **Scaffold core crate**

   ```bash
   cargo new --lib speakr-core
   mv src/*.rs speakr-core/src/          # move existing logic
   rm -rf src/
   ```

3. **Scaffold Tauri crate**

   ```bash
   cargo tauri init --template leptos speakr-tauri
   # move existing src-tauri/ into speakr-tauri/
   mv src-tauri speakr-tauri/
   ```

4. **Wire dependency**
   In `speakr-tauri/Cargo.toml` add:

   ```toml
   speakr-core = { path = "../speakr-core" }
   ```

5. **(Optional) Separate UI crate**

   ```bash
   cargo new --lib speakr-ui
   mv speakr-tauri/src-leptos/* speakr-ui/src/
   # then depend on speakr-ui from speakr-tauri via WASM asset pipeline
   ```

6. **Update paths in code & imports**.
7. **Run tests & build**

   ```bash
   cargo test --workspace
   cargo tauri dev -p speakr-tauri
   ```

8. **CI / Nix** – update workflows and `devenv.nix` to use `--workspace`.

Completion of these steps should yield the new structure with all tests & `tauri dev` working.
