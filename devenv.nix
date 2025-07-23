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
    PROJECT_NAME = "Speakr";
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

    # Suppress direnv export listings
    DIRENV_LOG_FORMAT = "";
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
  packages =
    with pkgs;
    [
      # Core development tools
      git
      git-lfs
      curl
      wget
      jq

      # Rust ecosystem tools
      cargo-watch # Auto-rebuild on file changes
      cargo-expand # Expand macros for debugging
      cargo-audit # Security auditing
      cargo-outdated # Check for outdated dependencies
      cargo-tauri # Tauri framework
      cargo-tarpaulin # Code coverage for Rust

      # Tauri and frontend tools
      trunk # WebAssembly web application bundler
      wasm-pack # WebAssembly build tool
      pkg-config # Package configuration

      # Documentation tools
      mdbook # Documentation generator
      mdbook-linkcheck # Link checker for mdbook
      mdbook-admonish # Admonition blocks for mdbook
      mdbook-mermaid # Mermaid diagrams for mdbook
      mdbook-pagetoc # Page Table of Contents for mdbook

      # Shell and productivity tools
      ripgrep # Fast text search
      fd # Fast file finder
      bat # Better cat with syntax highlighting
      eza # Better ls with colors
      fzf # Fuzzy finder
      pre-commit # Pre-commit hooks
      markdownlint-cli # Markdown linter
      # direnv # Directory-based environments
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
      # macOS-specific libraries
      pkgs.darwin.libiconv
    ];

  # Development scripts
  scripts = {
    # Help and information
    help = {
      description = "Display toolchain info and available commands";
      exec =
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

          # Build script data with inferred groups (excluding help itself)
          scriptLines = lib.mapAttrsToList (
            name: value: if name != "help" then "${getGroup name}|${name}|${value.description}" else ""
          ) config.scripts;

          scriptData = builtins.concatStringsSep "\n" (builtins.filter (x: x != "") scriptLines);

          # Define custom group order and process each group
          processGroups = ''
            # Define group order and icons
            declare -a GROUP_ORDER=("run" "build" "docs" "tools")
            declare -A GROUP_ICONS=(
              ["run"]=" 🚀 Run  "
              ["build"]="🔨 Build "
              ["docs"]=" 📖 Docs "
              ["tools"]="🧰 Utils "
            )

            # Process each group in custom order
            for group in "''${GROUP_ORDER[@]}"; do
              # Get commands for this group, sorted alphabetically by command name
              group_commands=$(echo '${scriptData}' | grep "^$group|" | sort -t'|' -k2,2)

              if [ -n "$group_commands" ]; then
                if [ "$group" != "run" ]; then
                  echo ""
                fi

                header="''${GROUP_ICONS[$group]:-$(echo $group | sed 's/./\U&/')}"
                printf "\033[2m────\033[0m \033[1m%s\033[0m \033[2m──────────────────────────────────────────────────────\033[0m\n" "$header"

                echo "$group_commands" | while IFS='|' read -r grp name desc; do
                  printf "     \033[1;35m%-20s\033[0m - %s\n" "$name" "$desc"
                done
              fi
            done
          '';
        in
        ''
          printf "\033[2m────\033[0m \033[1m📦 Toolchain\033[0m \033[2m───────────────────────────────────────────────────\033[0m\n"
          echo "        🦀  Rust $(rustc --version)"
          echo "        🌐  Trunk $(trunk --version)"
          echo "        🔧  Tauri CLI $(cargo tauri --version 2>/dev/null || echo 'not installed')"
          echo "        📖  mdbook $(mdbook --version)"
          echo ""
          echo ""

          # Process groups in custom order
          ${processGroups}

          echo ""
          printf "🦀 Welcome to the \033[1;36m${config.env.PROJECT_NAME}\033[0m 🗣️  dev-env!\n"
          echo ""
          printf "Run \033[1;36mdev\033[0m to start the application.\n"
          echo ""
        '';
    };

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
      exec = "
        local BOOK_DIR=$DEVENV_ROOT/book/
        pushd $BOOK_DIR && \
        mdbook build && \
        popd
      ";
    };
    docs-serve = {
      description = "Start documentation server";
      exec = "
        local BOOK_DIR=$DEVENV_ROOT/book/
        pushd $BOOK_DIR && \
        mdbook serve --open && \
        popd
      ";
    };
    docs-install-plugins = {
      description = "Install mdbook plugins";
      exec = ''
        local BOOK_DIR=$DEVENV_ROOT/book/
        pushd $THEME_DIR && \
        mdbook-admonish install $BOOK_DIR && \
        mdbook-mermaid install $BOOK_DIR && \
        mdbook-pagetoc install $BOOK_DIR && \
        popd
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
    coverage = {
      description = "Run cargo coverage";
      exec = "cargo tarpaulin --workspace --all-features --out Html --output-dir ./coverage";
    };

    # Pre-commit setup
    pre-commit-install = {
      description = "Install pre-commit hooks";
      exec = "pre-commit install";
    };
    pre-commit-run = {
      description = "Run pre-commit hooks on staged files";
      exec = "pre-commit run";
    };
    pre-commit-run-all = {
      description = "Run pre-commit hooks on all files";
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

      # Define custom group order and process each group
      processGroups = ''
        # Define group order and icons
        declare -a GROUP_ORDER=("run" "build" "docs" "tools")
        declare -A GROUP_ICONS=(
          ["run"]=" 🚀 Run  "
          ["build"]="🔨 Build "
          ["docs"]=" 📖 Docs "
          ["tools"]="🧰 Utils "
        )

        # Process each group in custom order
        for group in "''${GROUP_ORDER[@]}"; do
          # Get commands for this group, sorted alphabetically by command name
          group_commands=$(echo '${scriptData}' | grep "^$group|" | sort -t'|' -k2,2)

          if [ -n "$group_commands" ]; then
            if [ "$group" != "run" ]; then
              echo ""
            fi

            header="''${GROUP_ICONS[$group]:-$(echo $group | sed 's/./\U&/')}"
            printf "\033[2m────\033[0m \033[1m%s\033[0m \033[2m──────────────────────────────────────────────────────\033[0m\n" "$header"

            echo "$group_commands" | while IFS='|' read -r grp name desc; do
              # printf "     %-20s - %s\n" "$name" "$desc"
              printf "     \033[1;35m%-20s\033[0m - %s\n" "$name" "$desc"
            done
          fi
        done
      '';
    in
    ''
      printf "\033[2m────\033[0m \033[1m📦 Toolchain\033[0m \033[2m───────────────────────────────────────────────────\033[0m\n"
      echo "        🦀  Rust $(rustc --version)"
      echo "        🌐  Trunk $(trunk --version)"
      echo "        🔧  Tauri CLI $(cargo tauri --version 2>/dev/null || echo 'not installed')"
      echo "        📖  mdbook $(mdbook --version)"
      echo ""
      echo ""

      # Process groups in custom order
      ${processGroups}

      # First-run setup
      if [ ! -f .devenv-initialized ]; then
        echo "🔄 First time setup..."
        echo "Installing pre-commit hooks..."
        pre-commit install 2>/dev/null || true
        echo "Installing documentation plugins..."
        docs-install-plugins 2>/dev/null || true
        touch .devenv-initialized
        echo "✨ Setup complete!"
      fi

      echo ""
      printf "🦀 Welcome to the \033[1;36m${config.env.PROJECT_NAME}\033[0m 🗣️  dev-env!\n"
      echo ""
      printf "Run \033[1;36mdev\033[0m to start the application.\n"
      echo ""
    '';

}
