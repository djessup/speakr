#!/usr/bin/env bash
# Advanced selective testing script for Speakr
# This demonstrates more granular testing based on changed files and dependencies

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get changed files since last commit (or specific commit)
get_changed_files() {
    local base_commit="${1:-HEAD~1}"
    git diff --name-only "$base_commit" HEAD 2>/dev/null || git diff --name-only --cached
}

# Determine which packages are affected by changed files
get_affected_packages() {
    local changed_files
    changed_files=$(get_changed_files "$@")

    local affected_packages=()

    while IFS= read -r file; do
        case "$file" in
        speakr-core/*)
            if [[ ! " ${affected_packages[*]} " =~ " speakr-core " ]]; then
                affected_packages+=("speakr-core")
            fi
            # Core changes affect tauri (dependency)
            if [[ ! " ${affected_packages[*]} " =~ " speakr-tauri " ]]; then
                affected_packages+=("speakr-tauri")
            fi
            ;;
        speakr-tauri/*)
            if [[ ! " ${affected_packages[*]} " =~ " speakr-tauri " ]]; then
                affected_packages+=("speakr-tauri")
            fi
            ;;
        speakr-ui/*)
            if [[ ! " ${affected_packages[*]} " =~ " speakr-ui " ]]; then
                affected_packages+=("speakr-ui")
            fi
            ;;
        Cargo.toml | Cargo.lock | .cargo/*)
            # Workspace changes affect everything
            affected_packages=("speakr-core" "speakr-tauri" "speakr-ui")
            break
            ;;
        esac
    done <<<"$changed_files"

    printf '%s\n' "${affected_packages[@]}"
}

# Run tests for specific package
run_package_tests() {
    local package="$1"
    print_status "Running tests for package: $package"

    # Format check
    if ! cargo fmt --package "$package" --check; then
        print_error "Format check failed for $package"
        return 1
    fi

    # Clippy check
    if ! cargo clippy --package "$package" --all-targets --all-features -- -D warnings; then
        print_error "Clippy check failed for $package"
        return 1
    fi

    # Unit tests
    if ! cargo test --package "$package"; then
        print_error "Tests failed for $package"
        return 1
    fi

    print_status "✅ All checks passed for $package"
}

# Main execution
main() {
    local base_commit="${1:-}"

    print_status "Starting selective testing..."

    if [[ -n "$base_commit" ]]; then
        print_status "Comparing against commit: $base_commit"
    else
        print_status "Comparing against last commit"
    fi

    local affected_packages
    affected_packages=$(get_affected_packages "$base_commit")

    if [[ -z "$affected_packages" ]]; then
        print_status "No Rust packages affected by changes"
        exit 0
    fi

    print_status "Affected packages: $(echo "$affected_packages" | tr '\n' ' ')"

    local failed_packages=()

    while IFS= read -r package; do
        if [[ -n "$package" ]]; then
            if ! run_package_tests "$package"; then
                failed_packages+=("$package")
            fi
        fi
    done <<<"$affected_packages"

    if [[ ${#failed_packages[@]} -gt 0 ]]; then
        print_error "Failed packages: ${failed_packages[*]}"
        exit 1
    fi

    print_status "🎉 All affected packages passed!"
}

# Show usage if --help is passed
if [[ "${1:-}" == "--help" ]]; then
    cat <<EOF
Usage: $0 [base-commit]

Runs selective tests only for packages affected by changes since base-commit.
If no base-commit is provided, compares against HEAD~1.

Examples:
  $0                    # Compare against last commit
  $0 main              # Compare against main branch
  $0 abc123def         # Compare against specific commit

This script is more granular than pre-commit hooks and can be used for:
- CI optimization
- Local development workflow
- Manual testing of specific changes
EOF
    exit 0
fi

main "$@"
