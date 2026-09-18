# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Initium is a Rust CLI tool (published on crates.io) that bootstraps project
configuration files (.editorconfig, .prettierrc, .rubocop.yml, etc.) for
multiple language ecosystems: Basic, Ruby, Python, Node.js, Go, Rust, and
Bash. Each language supports templates (e.g., `ruby --template rails`,
`bash --template devops`) and optional git hook generation.

## Build & Development Commands

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo test                     # Run all tests
cargo test -p initium --test e2e_tests  # E2E tests only
cargo test <test_name>         # Run a single test by name
cargo fmt --all                # Format code
cargo fmt --all -- --check     # Check formatting
cargo clippy --all-targets --all-features -- -D warnings  # Lint
just ci-local                  # Full local CI (lint + test)
just pre-commit                # Quick pre-commit checks (fmt + clippy + check)
```

## Architecture

**CLI layer** (`src/main.rs`): Uses `clap` derive macros. The `Cli` struct holds global flags (`--force`, `--fail-on-exists`, `--dry-run`, `--hooks`, `--target`). Each language is a `Commands` enum variant with an optional `--template` arg.

**Command dispatch** (`src/commands.rs`): `CommandHandler` holds the resolved flags and delegates to `ConfigGenerator` for file generation and `GitHooksGenerator` for hook generation. All handler methods are async.

**Generators** (`src/generators/`): One file per language (`basic.rs`,
`ruby.rs`, `python.rs`, `node.rs`, `go.rs`, `rust.rs`, `bash.rs`), plus
`hooks.rs` for git hooks and `common.rs` for shared utilities. `mod.rs`
contains `ConfigGenerator` and the `ProjectType` enum used for auto-detection.
Auto-detection checks for marker files (e.g., `Gemfile` -> Ruby, `go.mod` ->
Go, `.shellcheckrc` / `main.sh` / `.bats` -> Bash).

**Config structs** (`src/config.rs`): `EditorConfig`, `PrettierConfig`, `PackageJson` with `Display` implementations that produce the actual file content.

**Errors** (`src/error.rs`): `InitiumError` via `thiserror` with variants for directory issues, file conflicts, serialization, and git state.

## Code Style

- Rust 2021 edition, `rustfmt.toml`: max_width=100, tab_spaces=4, Unix newlines
- Clippy configured with relaxed thresholds in `.clippy.toml` (e.g., cognitive-complexity=35, too-many-arguments=8)
- All clippy warnings treated as errors (`-D warnings`)

## Testing

Tests live in `tests/` as separate integration test files:
- `unit_tests.rs` - Config struct creation and formatting
- `generators_tests.rs` - File generation logic
- `integration_tests.rs` - Full CLI flow with `assert_cmd`
- `cli_tests.rs` - CLI argument parsing
- `e2e_tests.rs` - End-to-end scenarios
- `fail_on_exists_tests.rs` - `--fail-on-exists` flag behavior

Tests use `tempfile`/`assert_fs` for temporary directories and `assert_cmd` for running the binary.

## CI

GitHub Actions in `.github/workflows/`:
- `ci.yml`: Lint (rustfmt + clippy) then test across Ubuntu/macOS/Windows on stable + 1.89, with tarpaulin coverage on Ubuntu
- `release.yml`: Tag-triggered release with cross-platform binary builds

## Release

Version bumps via `just release-patch|minor|major` which uses `cargo-set-version`, commits, tags, and pushes.

<!-- rtk-instructions v2 -->
# Command output

Command output here is condensed to save tokens, keeping every signal and
dropping costly noise. Treat it as the complete result: run commands
normally, and batch related commands into one call to avoid extra turns.
Truncated results state their recovery path in their own output. Re-run a
command as `rtk proxy <cmd>` only when its result is unusable: empty when
output was clearly expected, contradicting its exit code, or garbled.
<!-- /rtk-instructions -->

<!-- icm:start -->
## Persistent memory (ICM) — MANDATORY

This project uses [ICM](https://github.com/rtk-ai/icm) for persistent memory across sessions.
You MUST use it actively. Not optional.

### Recall (before starting work)
```bash
icm recall "query"                        # search memories
icm recall "query" -t "topic-name"        # filter by topic
icm recall-context "query" --limit 5      # formatted for prompt injection
```

### Store — MANDATORY triggers
You MUST call `icm store` when ANY of the following happens:
1. **Error resolved** → `icm store -t errors-resolved -c "description" -i high -k "keyword1,keyword2"`
2. **Architecture/design decision** → `icm store -t decisions-{project} -c "description" -i high`
3. **User preference discovered** → `icm store -t preferences -c "description" -i critical`
4. **Significant task completed** → `icm store -t context-{project} -c "summary of work done" -i high`
5. **Conversation exceeds ~20 tool calls without a store** → store a progress summary

Do this BEFORE responding to the user. Not after. Not later. Immediately.

Do NOT store: trivial details, info already in CLAUDE.md, ephemeral state (build logs, git status).

### Other commands
```bash
icm update <id> -c "updated content"     # edit memory in-place
icm health                                # topic hygiene audit
icm topics                                # list all topics
```
<!-- icm:end -->
