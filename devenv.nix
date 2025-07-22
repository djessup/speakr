{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  pkgs-stable = import inputs.nixpkgs-stable { system = pkgs.stdenv.system; };
in
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

    # macOS linking fix - expose library and include paths
    LIBRARY_PATH = "${pkgs.darwin.libiconv}/lib:${pkgs.lib.makeLibraryPath [ pkgs.darwin.libiconv ]}";
    CPATH = "${pkgs.darwin.libiconv}/include";
    LDFLAGS = "-L${pkgs.darwin.libiconv}/lib";
    CPPFLAGS = "-I${pkgs.darwin.libiconv}/include";
  };

  # Language configurations
  languages = {
    # Rust configuration
    rust = {
      enable = true;
      channel = "stable";
      targets = [
        "wasm32-unknown-unknown" # Required for Leptos WebAssembly
      ];
    };
    # Node.js configuration
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

    # macOS-specific libraries
    darwin.libiconv
    # darwin.apple_sdk.frameworks.Security
    # darwin.apple_sdk.frameworks.CoreFoundation
    # darwin.apple_sdk.frameworks.SystemConfiguration

    # Rust ecosystem tools
    cargo-watch # Auto-rebuild on file changes
    cargo-expand # Expand macros for debugging
    cargo-audit # Security auditing
    cargo-outdated # Check for outdated dependencies
    cargo-tauri # Tauri framework

    # Tauri and frontend tools
    trunk # WebAssembly web application bundler
    wasm-pack # WebAssembly build tool

    # Documentation tools
    mdbook # Documentation generator
    mdbook-linkcheck # Link checker for mdbook
    mdbook-admonish # Admonition blocks for mdbook
    mdbook-mermaid # Mermaid diagrams for mdbook

    # Shell and productivity tools
    ripgrep # Fast text search
    fd # Fast file finder
    bat # Better cat with syntax highlighting
    eza # Better ls with colors
    fzf # Fuzzy finder
    pre-commit # Pre-commit hooks
    markdownlint-cli # Markdown linter
    # direnv # Directory-based environments
  ];

  # Development scripts
  scripts = {
    # Frontend development
    dev-ui = {
      description = "Start frontend only (Trunk serve)";
      exec = ''
        cd $DEVENV_ROOT/speakr-ui && \
        trunk serve --port 1420 --open
      '';
    };
    build-ui = {
      description = "Build frontend (Trunk build --release)";
      exec = ''
        cd $DEVENV_ROOT/speakr-ui && \
        trunk build --release
      '';
    };

    # Tauri development
    dev-tauri = {
      description = "Start Tauri development server";
      exec = ''
        cd $DEVENV_ROOT/speakr-tauri && \
        cargo tauri dev
      '';
    };
    build-tauri = {
      description = "Build Tauri application";
      exec = ''
        cd $DEVENV_ROOT/speakr-tauri && \
        cargo tauri build
      '';
    };

    # Full application
    dev = {
      description = "Start full Tauri development server";
      exec = "cd $DEVENV_ROOT && cargo tauri dev";
    };
    build = {
      description = "Build production application";
      exec = "cd $DEVENV_ROOT && cargo tauri build";
    };

    # Documentation
    docs-build = {
      description = "Build documentation";
      exec = "cd $DEVENV_ROOT && mdbook build";
    };
    docs-watch = {
      description = "Watch documentation and rebuild on changes";
      exec = "cd $DEVENV_ROOT && mdbook watch";
    };
    docs-serve = {
      description = "Start documentation server";
      exec = "cd $DEVENV_ROOT && mdbook serve --open";
    };
    docs-install-plugins = {
      description = "Install mdbook plugins";
      exec = ''
        mdbook-admonish install $DEVENV_ROOT --css-dir $DEVENV_ROOT/docs/theme
        mdbook-mermaid install $DEVENV_ROOT
        cp $DEVENV_ROOT/docs/theme/mermaid-init.js $DEVENV_ROOT/docs/theme/mermaid.min.js $DEVENV_ROOT/docs/theme/
        rm -f $DEVENV_ROOT/docs/theme/mermaid-init.js $DEVENV_ROOT/docs/theme/mermaid.min.js
      '';
    };

    # Development utilities
    format = {
      description = "Format all code";
      exec = "cargo fmt --all";
    };
    lint = {
      description = "Run clippy linter";
      exec = "cargo clippy --all-targets --all-features --workspace";
    };
    test = {
      description = "Run all tests";
      exec = "cargo test --all --workspace";
    };
    audit = {
      description = "Run cargo audit";
      exec = "cargo audit --workspace";
    };
    update-deps = {
      description = "Update Rust dependencies";
      exec = "cargo update --workspace";
    };

    # Pre-commit setup
    pre-commit-install = {
      description = "Install pre-commit hooks";
      exec = "pre-commit install";
    };
    pre-commit-run = {
      description = "Run all pre-commit hooks";
      exec = "pre-commit run --all-files";
    };
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
  enterShell =
    let
      # Function to determine group based on script name
      getGroup =
        name:
        if lib.hasPrefix "dev" name then
          "run"
        else if lib.hasPrefix "build" name then
          "build"
        else if lib.hasPrefix "docs" name then
          "docs"
        else
          "tools";

      # Build script data with inferred groups
      scriptLines = lib.mapAttrsToList (
        name: value: "${getGroup name}|${name}|${value.description}"
      ) config.scripts;

      scriptData = builtins.concatStringsSep "\n" scriptLines;
    in
    ''
      echo "🦀 Welcome to ${config.env.PROJECT_NAME} development environment!"
      echo ""
      echo "==== 📦 Toolchain ================================================="
      echo "     Rust $(rustc --version)"
      echo "     Trunk $(trunk --version)"
      echo "     Tauri CLI $(cargo tauri --version 2>/dev/null || echo 'not installed')"
      echo "     mdbook $(mdbook --version)"
      echo ""
      echo ""

      # Dynamically generate command sections
      echo '${scriptData}' | sort -t'|' -k1,1 -k2,2 | awk -F'|' '
        BEGIN {
          icons["run"]   = "🚀 Run";
          icons["build"] = "🔨 Build";
          icons["docs"]  = "📖 Docs";
          icons["tools"] = "🧰 Utils";
        }
        {
          grp = $1; name = $2; desc = $3;
          if (grp != current) {
            if (NR > 1) print "";
            header = (icons[grp] ? icons[grp] : toupper(substr(grp,1,1)) substr(grp,2));
            printf "==== %s =======================================================\n", header;
            current = grp;
          }
          printf "     %-20s - %s\n", name, desc;
        }'

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
