# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Documentation

This repository uses [mdBook](https://github.com/rust-lang/mdBook) to generate the project documentation.

### Preview locally

```bash
cargo install mdbook  # once
cd docs
mdbook serve -o  # opens http://localhost:3000 and rebuilds on change
```

GitHub Pages is automatically updated on pushes to `main` via the workflow in `.github/workflows/docs.yml`.
