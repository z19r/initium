# Initium — project command runner
# https://github.com/casey/just

set dotenv-load := false

version := `cargo get package.version 2>/dev/null || echo 'unknown'`

# ─── Default ──────────────────────────────────────────────────────────

default:
    @just --list --unsorted

# ─── Development ──────────────────────────────────────────────────────

# Build debug binary
build:
    cargo build

# Build optimized release binary
build-release:
    cargo build --release

# Install initium to ~/.cargo/bin
install:
    cargo install --path .

# Run the CLI with arguments (e.g. `just run -- ruby --template rails`)
run *ARGS:
    cargo run -- {{ARGS}}

# Watch for changes and rebuild on save
watch:
    cargo watch -x check -x 'test -- --nocapture'

# Check compilation without producing binaries
check:
    cargo check --all-targets --all-features

# ─── Testing ──────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test

# Run a single test by name
test-one NAME:
    cargo test {{NAME}} -- --nocapture

# Run only unit tests
test-unit:
    cargo test -p initium --test unit_tests

# Run only integration tests
test-integration:
    cargo test -p initium --test integration_tests

# Run only CLI argument parsing tests
test-cli:
    cargo test -p initium --test cli_tests

# Run only end-to-end tests
test-e2e:
    cargo test -p initium --test e2e_tests

# Run only fail-on-exists tests
test-fail-on-exists:
    cargo test -p initium --test fail_on_exists_tests

# Run only generator tests
test-generators:
    cargo test -p initium --test generators_tests

# Run tests with stdout visible
test-verbose:
    cargo test -- --nocapture

# Generate HTML coverage report via tarpaulin
test-coverage:
    cargo tarpaulin --out Html --output-directory coverage
    @echo "Report: coverage/tarpaulin-report.html"

# Generate coverage and fail if below threshold
test-coverage-check THRESHOLD="80":
    cargo tarpaulin --fail-under {{THRESHOLD}}

# ─── Linting & Formatting ────────────────────────────────────────────

# Format all source files
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run clippy with warnings-as-errors
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy and auto-fix what it can
clippy-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# Lint everything (format check + clippy)
lint: fmt-check clippy

# Fix everything (format + clippy auto-fix)
fix: fmt clippy-fix

# ─── Security & Dependencies ─────────────────────────────────────────

# Audit dependencies for known vulnerabilities
audit:
    cargo audit

# Show outdated dependencies
outdated:
    cargo outdated

# Show the full dependency tree
deps:
    cargo tree

# Show duplicate dependencies
deps-dupes:
    cargo tree -d

# Update all dependencies to latest compatible versions
deps-update:
    cargo update

# ─── Pre-commit & CI ─────────────────────────────────────────────────

# Quick pre-commit checks (format + clippy + check)
pre-commit: fmt clippy check

# Full local CI pipeline (lint + all tests)
ci-local: lint test

# CI lint stage (matches GitHub Actions)
ci-lint:
    @echo "CI: lint + format check"
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

# CI test stage (matches GitHub Actions)
ci-test:
    @echo "CI: tests + coverage"
    cargo test --all-features
    cargo tarpaulin --out Xml --output-dir coverage

# ─── Release ──────────────────────────────────────────────────────────

# Release quality gate (fmt + clippy + test)
release-check:
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# Preview what a release would do without changing anything
release-dry-run LEVEL:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! "{{LEVEL}}" =~ ^(patch|minor|major)$ ]]; then
        echo "Usage: just release-dry-run patch|minor|major"; exit 1
    fi
    CURRENT=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    echo "Current version: $CURRENT"
    echo "Bump level: {{LEVEL}}"
    just release-check
    echo ""
    echo "All checks passed. Run: just release {{LEVEL}}"

# Bump version, create release branch + PR (requires: cargo-set-version, gh)
release LEVEL: release-check
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ ! "{{LEVEL}}" =~ ^(patch|minor|major)$ ]]; then
        echo "Usage: just release patch|minor|major"; exit 1
    fi
    if [[ -n "$(git status --porcelain)" ]]; then
        echo "Error: dirty working tree"; exit 1
    fi
    BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [[ "$BRANCH" != "main" ]]; then
        echo "Error: must be on main (currently on $BRANCH)"; exit 1
    fi
    git pull --ff-only origin main
    cargo set-version --bump {{LEVEL}}
    cargo check --quiet
    VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
    git checkout -b "release/v${VERSION}"
    git add Cargo.toml Cargo.lock
    git commit -m "release: v${VERSION}"
    git push -u origin "release/v${VERSION}"
    gh pr create \
        --title "release: v${VERSION}" \
        --body "Bump to v${VERSION} ({{LEVEL}} release)" \
        --base main
    echo ""
    echo "PR created. Next steps:"
    echo "  gh pr checks           # watch CI"
    echo "  gh pr merge --squash --delete-branch"
    echo "  gh run watch           # watch release workflow"

# ─── Cleanup ──────────────────────────────────────────────────────────

# Remove build artifacts
clean:
    cargo clean

# Remove build artifacts and coverage reports
clean-all: clean
    rm -rf coverage dist

# ─── Project Info ─────────────────────────────────────────────────────

# Show project and toolchain versions
info:
    @echo "Initium v{{version}}"
    @echo ""
    @echo "Toolchain"
    @echo "  rustc:  $(rustc --version)"
    @echo "  cargo:  $(cargo --version)"
    @echo "  just:   $(just --version)"
    @echo ""
    @echo "Dev Tools"
    @echo "  cargo-get:         $(cargo get --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-set-version: $(cargo set-version --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-audit:       $(cargo audit --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-outdated:    $(cargo outdated --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-watch:       $(cargo watch --version 2>/dev/null || echo 'not installed')"
    @echo "  cargo-tarpaulin:   $(cargo tarpaulin --version 2>/dev/null || echo 'not installed')"

# Show lines of code
loc:
    @echo "Source:"
    @find src -name '*.rs' | xargs wc -l | tail -1
    @echo "Tests:"
    @find tests -name '*.rs' | xargs wc -l | tail -1

# Install all development tools
install-tools:
    cargo install cargo-get cargo-set-version cargo-audit cargo-outdated cargo-watch
    cargo install cargo-tarpaulin --version 0.32.8

# Show disk usage of build artifacts and caches
cache-status:
    @echo "Disk Usage"
    @echo "  target/:         $(du -sh target 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  coverage/:       $(du -sh coverage 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  dist/:           $(du -sh dist 2>/dev/null | cut -f1 || echo 'n/a')"
    @echo "  ~/.cargo/registry: $(du -sh ~/.cargo/registry 2>/dev/null | cut -f1 || echo 'n/a')"

# ─── Dogfooding ───────────────────────────────────────────────────────

# Generate configs into a temp directory (test the CLI output)
try PROFILE="ruby" *ARGS="":
    #!/usr/bin/env bash
    set -euo pipefail
    DIR=$(mktemp -d)
    echo "Target: $DIR"
    cargo run -- --target "$DIR" {{PROFILE}} {{ARGS}}
    echo ""
    echo "Generated files:"
    ls -la "$DIR"
    echo ""
    echo "Cleaning up..."
    rm -rf "$DIR"

# Dry-run a profile to preview output
preview PROFILE="ruby" *ARGS="":
    cargo run -- --dry-run {{PROFILE}} {{ARGS}}
