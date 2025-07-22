# Development Guide

This guide will help you set up the development environment for the Speakr Tauri + Leptos
application.

## Prerequisites

### macOS Setup

1. **Install Nix** (if not already installed):

   ```bash
   sh <(curl -L https://nixos.org/nix/install)
   ```

2. **Enable Nix flakes** (add to `~/.config/nix/nix.conf`):

   ```sh
   experimental-features = nix-command flakes
   ```

3. **Install devenv**:

   ```bash
   nix profile install --accept-flake-config github:cachix/devenv/latest
   ```

4. **Install direnv** (recommended):

   ```bash
   nix profile install nixpkgs#direnv
   echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc  # or ~/.bashrc
   ```

## Getting Started

1. **Clone the repository**:

   ```bash
   git clone <your-repo-url>
   cd speakr
   ```

2. **Allow direnv** (if using direnv):

   ```bash
   direnv allow
   ```

   OR **Enter devenv shell** manually:

   ```bash
   devenv shell
   ```

3. **First-time setup** (done automatically):
   - Pre-commit hooks will be installed
   - Documentation plugins will be installed
   - Development environment will be initialized

4. **Copy environment variables**:

   ```bash
   cp .env.example .env
   # Edit .env as needed
   ```

## Available Commands

Once in the development environment, you have access to these commands:

### Development

- `dev` - Start full Tauri development server (frontend + backend)
- `dev-ui` - Start frontend only (Trunk serve on port 1420)
- `dev-tauri` - Start Tauri development server only
- `build` - Build production application
- `build-ui` - Build frontend for production

### Docs (mdbook)

- `docs-serve` - Start documentation server (mdbook)
- `docs-build` - Build documentation
- `docs-install-plugins` - Install mdbook plugins

### Utilities

- `format` - Format all code with rustfmt
- `lint` - Run clippy linter
- `test` - Run all tests
- `audit` - Run cargo audit for security issues
- `update-deps` - Update dependencies

### Pre-commit

- `pre-commit-install` - Install pre-commit hooks
- `pre-commit-run` - Run pre-commit on all files

## Project Structure

```text
speakr/
├── speakr-ui/              # Leptos frontend source
├── speakr-tauri/           # Tauri backend source
├── speakr-core/            # Core functionality (whisper, audio capture, text injection)
├── speakr-types/           # Shared types (events, config, etc.)
├── devenv.nix              # Devenv definition
├── devenv.yaml             # Devenv inputs
├── .envrc                  # Direnv configuration
├── .env.example            # Environment variables template
└── .pre-commit-config.yaml # Pre-commit config
```

## Development Workflow

### Starting Development

1. Enter the development environment (if not using direnv):

   ```bash
   devenv shell
   ```

2. Start the development server:

   ```bash
   dev
   ```

   This will start both the Leptos frontend (on port 1420) and the Tauri backend.

### Code Quality

The environment includes automatic code quality checks:

- **Pre-commit hooks**: Run `cargo fmt`, `cargo clippy`, and tests before each commit
- **Format code**: `format` - formats all Rust code
- **Lint code**: `lint` - runs Clippy with strict settings
- **Run tests**: `test` - runs all tests

### Documentation

- **Serve docs locally**: `docs-serve`
- **Build docs**: `docs-build`

The documentation uses mdbook with additional plugins:

- **mdbook-admonish**: For callout blocks
- **mdbook-mermaid**: For diagrams
- **mdbook-linkcheck**: For link validation

### Building for Production

- **Full build**: `build` - creates optimized Tauri application
- **Frontend only**: `build-ui` - creates optimized web assets

## Troubleshooting

### Rust Version Issues

The environment uses the latest stable Rust channel. If you need a specific version, modify
`devenv.nix`:

```nix
languages.rust.channel = "1.80.0";  # Specific version
```

### Missing Tools

If a tool is missing, add it to the `packages` array in `devenv.nix`:

```nix
packages = with pkgs; [
  # existing packages...
  your-tool
];
```

### Environment Variables

Add custom environment variables to `.env` or to the `env` section in `devenv.nix`.

### macOS-Specific Issues

1. **Xcode Command Line Tools**: Ensure they're installed:

   ```bash
   xcode-select --install
   ```

2. **System Frameworks**: The devenv automatically includes required macOS frameworks for Tauri.

## VS Code Integration

If using VS Code, the environment includes rust-analyzer and other useful extensions. Install the
recommended extensions:

- rust-analyzer
- Tauri
- Better TOML
- Error Lens

## Contributing

1. Make your changes
2. Run tests: `test`
3. Format code: `format`
4. Check linting: `lint`
5. Commit (pre-commit hooks will run automatically)

The pre-commit hooks ensure code quality and consistency across the project.
