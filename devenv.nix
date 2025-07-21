{
  pkgs,
  lib,
  config,
  ...
}:
{
  # Enable debug mode for troubleshooting
  devenv.debug = false;
  devenv.warnOnNewVersion = false;

  # Load environment variables from .env
  dotenv.enable = true;

  # Define environment variables
  env = {
    PROJECT_NAME = "speakr";
    RUST_BACKTRACE = "1";
    RUST_LOG = "info";
    # Tauri development
    TAURI_SKIP_DEVSERVER_CHECK = "true";
    # Frontend development
    TRUNK_SERVE_PORT = "1420";
  };

  # Language configurations
  languages = {
    rust = {
      enable = true;
      channel = "stable";
      targets = [
        "wasm32-unknown-unknown" # Required for Leptos WebAssembly
      ];
    };

    javascript = {
      enable = true;
      package = pkgs.nodejs_20;
    };
  };

  # Development packages
  packages = with pkgs; [
    # Core development tools
    git
    curl
    wget
    jq

    # Rust ecosystem tools
    cargo-watch # Auto-rebuild on file changes
    cargo-expand # Expand macros for debugging
    cargo-audit # Security auditing
    cargo-outdated # Check for outdated dependencies
    # cargo-tauri # Tauri framework

    # Tauri and frontend tools
    trunk # WebAssembly web application bundler
    wasm-pack # WebAssembly build tool

    # Documentation tools
    mdbook # Documentation generator
    mdbook-linkcheck # Link checker for mdbook (install separately)
    mdbook-admonish # Admonition blocks for mdbook (install separately)
    mdbook-mermaid # Mermaid diagrams for mdbook (install separately)

    # Shell and productivity tools
    ripgrep # Fast text search
    fd # Fast file finder
    bat # Better cat with syntax highlighting
    eza # Better ls with colors
    fzf # Fuzzy finder
    pre-commit # Pre-commit hooks
    # direnv # Directory-based environments
  ];

  # Development scripts
  scripts = {
    # Frontend development
    dev-ui.exec = "cd $DEVENV_ROOT/speakr-ui && trunk serve --port 1420 --open";
    build-ui.exec = "cd $DEVENV_ROOT/speakr-ui && trunk build --release";

    # Tauri development
    dev-tauri.exec = "cd $DEVENV_ROOT/speakr-tauri && cargo tauri dev";
    build-tauri.exec = "cd $DEVENV_ROOT/speakr-tauri && cargo tauri build";

    # Full application
    dev.exec = "cd $DEVENV_ROOT && cargo tauri dev";
    build.exec = "cd $DEVENV_ROOT && cargo tauri build";

    # Documentation
    docs-build.exec = "cd $DEVENV_ROOT && mdbook build";
    docs-watch.exec = "cd $DEVENV_ROOT && mdbook watch";
    docs-serve.exec = "cd $DEVENV_ROOT && mdbook serve --open";
    docs-install-plugins.exec = ''
      mdbook-admonish install $DEVENV_ROOT --css-dir $DEVENV_ROOT/docs/theme
      mdbook-mermaid install $DEVENV_ROOT
      mv $DEVENV_ROOT/docs/theme/mermaid-init.js $DEVENV_ROOT/docs/theme/mermaid.min.js $DEVENV_ROOT/docs/theme/
    '';

    # Development utilities
    format.exec = "cargo fmt --all";
    lint.exec = "cargo clippy --all-targets --all-features --workspace";
    test.exec = "cargo test --all --workspace";
    audit.exec = "cargo audit";
    update-deps.exec = "cargo update";

    # Pre-commit setup
    pre-commit-install.exec = "pre-commit install";
    pre-commit-run.exec = "pre-commit run --all-files";
  };

  # Pre-commit hooks configuration (disabled temporarily)
  # pre-commit = {
  #   enable = true;
  #   hooks = {
  #     rustfmt = {
  #       enable = true;
  #     };
  #     clippy = {
  #       enable = true;
  #     };
  #   };
  # };

  # Development processes (optional - for running services in background)
  processes = {
    # Uncomment if you want to auto-start frontend dev server
    # frontend.exec = "trunk serve --port 1420";
    # frontend.ready = "curl -f http://localhost:1420";
  };

  # Shell initialization
  enterShell = ''
    echo "🦀 Welcome to ${config.env.PROJECT_NAME} development environment!"
    echo ""
    echo "==== 📦 Toolchain ================================================="
    echo "     Rust $(rustc --version)"
    echo "     Trunk $(trunk --version)"
    echo "     Tauri CLI $(cargo tauri --version 2>/dev/null || echo 'not installed')"
    echo "     mdbook $(mdbook --version)"
    echo ""
    echo ""
    echo "==== 🚀 Run ======================================================="
    echo "     dev                   - Start full Tauri development server"
    echo "     dev-ui                - Start frontend only (Trunk serve)"
    echo "     dev-tauri             - Start Tauri development server"
    echo ""
    echo "==== 🔨 Build ====================================================="
    echo "     build                 - Build production application"
    echo "     build-ui              - Build frontend (Trunk build --release)"
    echo "     build-tauri           - Build Tauri application"
    echo ""
    echo "==== 📖 Docs ======================================================"
    echo "     docs-serve            - Start documentation server"
    echo "     docs-build            - Build documentation"
    echo "     docs-watch            - Watch documentation and rebuild on changes"
    echo "     docs-install-plugins  - Install mdbook plugins"
    echo ""
    echo "==== 🧰 Utils ====================================================="
    echo "     format                - Format all code"
    echo "     lint                  - Run clippy linter"
    echo "     test                  - Run all tests"
    echo "     audit                 - Run cargo audit"
    echo "     update-deps           - Update Rust dependencies"
    echo "     pre-commit-run        - Run all pre-commit hooks"
    echo ""
    echo "🔧 Setup steps (run once):"
    echo "  1. pre-commit-install  - Install git hooks"
    echo "  2. docs-install-plugins - Install mdbook plugins"
    echo ""

    # Check if this is the first run
    if [ ! -f .devenv-initialized ]; then
      echo "🔄 First time setup..."
      echo "Installing pre-commit hooks..."
      pre-commit install 2>/dev/null || true
      echo "Installing documentation plugins..."
      docs-install-plugins 2>/dev/null || true
      touch .devenv-initialized
      echo "✅ Setup complete!"
    fi

    echo "Ready to develop! Run 'dev' to start the application."
  '';

}
