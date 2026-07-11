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
# RTK (Rust Token Killer) - Token-Optimized Commands

## Golden Rule

**Always prefix commands with `rtk`**. If RTK has a dedicated filter, it uses it. If not, it passes through unchanged. This means RTK is always safe to use.

**Important**: Even in command chains with `&&`, use `rtk`:
```bash
# ❌ Wrong
git add . && git commit -m "msg" && git push

# ✅ Correct
rtk git add . && rtk git commit -m "msg" && rtk git push
```

## RTK Commands by Workflow

### Build & Compile (80-90% savings)
```bash
rtk cargo build         # Cargo build output
rtk cargo check         # Cargo check output
rtk cargo clippy        # Clippy warnings grouped by file (80%)
rtk tsc                 # TypeScript errors grouped by file/code (83%)
rtk lint                # ESLint/Biome violations grouped (84%)
rtk prettier --check    # Files needing format only (70%)
rtk next build          # Next.js build with route metrics (87%)
```

### Test (60-99% savings)
```bash
rtk cargo test          # Cargo test failures only (90%)
rtk go test             # Go test failures only (90%)
rtk jest                # Jest failures only (99.5%)
rtk vitest              # Vitest failures only (99.5%)
rtk playwright test     # Playwright failures only (94%)
rtk pytest              # Python test failures only (90%)
rtk rake test           # Ruby test failures only (90%)
rtk rspec               # RSpec test failures only (60%)
rtk test <cmd>          # Generic test wrapper - failures only
```

### Git (59-80% savings)
```bash
rtk git status          # Compact status
rtk git log             # Compact log (works with all git flags)
rtk git diff            # Compact diff (80%)
rtk git show            # Compact show (80%)
rtk git add             # Ultra-compact confirmations (59%)
rtk git commit          # Ultra-compact confirmations (59%)
rtk git push            # Ultra-compact confirmations
rtk git pull            # Ultra-compact confirmations
rtk git branch          # Compact branch list
rtk git fetch           # Compact fetch
rtk git stash           # Compact stash
rtk git worktree        # Compact worktree
```

Note: Git passthrough works for ALL subcommands, even those not explicitly listed.

### GitHub (26-87% savings)
```bash
rtk gh pr view <num>    # Compact PR view (87%)
rtk gh pr checks        # Compact PR checks (79%)
rtk gh run list         # Compact workflow runs (82%)
rtk gh issue list       # Compact issue list (80%)
rtk gh api              # Compact API responses (26%)
```

### JavaScript/TypeScript Tooling (70-90% savings)
```bash
rtk pnpm list           # Compact dependency tree (70%)
rtk pnpm outdated       # Compact outdated packages (80%)
rtk pnpm install        # Compact install output (90%)
rtk npm run <script>    # Compact npm script output
rtk npx <cmd>           # Compact npx command output
rtk prisma              # Prisma without ASCII art (88%)
```

### Files & Search (60-75% savings)
```bash
rtk ls <path>           # Tree format, compact (65%)
rtk read <file>         # Code reading with filtering (60%)
rtk grep <pattern>      # Search grouped by file (75%). Format flags (-c, -l, -L, -o, -Z) run raw.
rtk find <pattern>      # Find grouped by directory (70%)
```

### Analysis & Debug (70-90% savings)
```bash
rtk err <cmd>           # Filter errors only from any command
rtk log <file>          # Deduplicated logs with counts
rtk json <file>         # JSON structure without values
rtk deps                # Dependency overview
rtk env                 # Environment variables compact
rtk summary <cmd>       # Smart summary of command output
rtk diff                # Ultra-compact diffs
```

### Infrastructure (85% savings)
```bash
rtk docker ps           # Compact container list
rtk docker images       # Compact image list
rtk docker logs <c>     # Deduplicated logs
rtk kubectl get         # Compact resource list
rtk kubectl logs        # Deduplicated pod logs
```

### Network (65-70% savings)
```bash
rtk curl <url>          # Compact HTTP responses (70%)
rtk wget <url>          # Compact download output (65%)
```

### Meta Commands
```bash
rtk gain                # View token savings statistics
rtk gain --history      # View command history with savings
rtk discover            # Analyze Claude Code sessions for missed RTK usage
rtk proxy <cmd>         # Run command without filtering (for debugging)
rtk init                # Add RTK instructions to CLAUDE.md
rtk init --global       # Add RTK to ~/.claude/CLAUDE.md
```

## Token Savings Overview

| Category | Commands | Typical Savings |
|----------|----------|-----------------|
| Tests | vitest, playwright, cargo test | 90-99% |
| Build | next, tsc, lint, prettier | 70-87% |
| Git | status, log, diff, add, commit | 59-80% |
| GitHub | gh pr, gh run, gh issue | 26-87% |
| Package Managers | pnpm, npm, npx | 70-90% |
| Files | ls, read, grep, find | 60-75% |
| Infrastructure | docker, kubectl | 85% |
| Network | curl, wget | 65-70% |

Overall average: **60-90% token reduction** on common development operations.
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
