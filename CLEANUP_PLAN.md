# Initium Cleanup & Refactor Plan

**Generated:** 2025-03-06 | **Scope:** Codebase audit, bug fixes, roadmap sanity, doc cleanup

---

## Executive Summary

Initium is a solid Rust CLI (v1.1.3) that bootstraps project configs. The core is sound: tests pass, clippy clean, generators work. However, there are **bugs**, **dangerous commands**, **redundant/ridiculous roadmap items**, and **dead or misleading docs**. This plan prioritizes fixes by impact.

---

## 1. Critical Bugs

### 1.1 Error type mismatch (`src/error.rs`)

- **`CurrentDirError(#[from] std::env::VarError)`** — Never used. `VarError` comes from `env::var()`, not `env::current_dir()` (which returns `io::Error`). Main swallows `current_dir()` errors with `unwrap_or_else(|_| PathBuf::from("."))` anyway.
- **Action:** Remove `CurrentDirError` entirely, or fix to use `io::Error` if we want to propagate `current_dir()` failures.

### 1.2 Redundant error variants (`src/error.rs`)

- **`FileExists`** vs **`FileAlreadyExists`** — Same semantic meaning, different messages. Used in:
  - `FileExists`: `common.rs` (write_file_if_not_exists), fail_on_exists tests
  - `FileAlreadyExists`: `hooks.rs` (git hooks)
- **Action:** Unify to one variant. Prefer `FileExists` (clearer) and update hooks to use it.

### 1.3 Justfile typo (`justfile`)

- **`release-major`** uses `cargo get version` (no `--`) while `release-patch` and `release-minor` use `cargo get --version`. Inconsistent; `cargo get version` may not work.
- **Action:** Use `cargo get --version` everywhere.

### 1.4 CI: Invalid Rust version (`.github/workflows/ci.yml`)

- Matrix: `rust: [stable, 1.89]` — Rust 1.89 may not exist yet. CI could fail on the second matrix entry.
- **Action:** Use `[stable]` only, or verify 1.89 exists / use a known version (e.g. `1.75` for MSRV).

---

## 2. Dangerous / Nonsensical Commands

### 2.1 `just clear-cache` — **DANGEROUS**

Current behavior:

- `clear-cache`: Deletes `~/.cargo/registry`, `~/.cargo/git`, `target`, **`~/.cargo/bin`**, **`~/.local/bin`**
- `clear-cache-tools`: Deletes **`~/.cargo/bin`** — uninstalls **all** cargo-installed binaries (cargo, rustfmt, clippy, etc.)
- `clear-cache-just`: Deletes **`~/.local/bin`** — uninstalls just and anything else there

**Impact:** A user running `just clear-cache` would nuke their toolchain. This is not "cache clearing" — it's destructive.

**Action:**

- Rename/refine: `clear-cache` should only clear **project-local** or **Cargo** caches: `target`, optionally `~/.cargo/registry` and `~/.cargo/git` for this project's cache key. **Never** touch `~/.cargo/bin` or `~/.local/bin`.
- Remove or heavily warn: `clear-cache-tools` and `clear-cache-just` are footguns. Either remove them or require an explicit `--yes-i-know` flag.

---

## 3. Roadmap — Remove Ridiculous Items

**ROADMAP.md** contains fantasy features that don't fit a small CLI bootstrap tool:

| Item | Why Remove |
|------|------------|
| **v2.0: Machine learning-based recommendations** | Overkill for a config generator. "Smart" detection via heuristics is fine; ML is not. |
| **v2.0: Plugin marketplace** | Massive scope. Plugin system alone is a big project. |
| **v2.0: Web interface / serve** | Different product. Initium is a CLI. |
| **v2.0: Analytics & insights** | Not relevant. No telemetry needed for local config generation. |
| **v3.0: Team collaboration, RBAC** | Enterprise bloat. |
| **v3.0: Security & compliance, audit trails** | Wrong tool. |
| **v3.0: ROI analysis, metrics** | Absurd for a config bootstrapper. |

**Action:** Slim ROADMAP to realistic v1.2/v1.3 items (Git hooks, CI templates, Docker, dry-run improvements, backup, validation). Move v2.0+ to a "maybe someday" section or delete. Keep the spirit: Git hooks, CI/CD, Docker, IDE configs, dry-run/backup/validation are reasonable.

---

## 4. Refactor Opportunities

### 4.1 `commands.rs` — Heavy duplication

- Each `handle_*` (basic, ruby, python, node, go, rust) repeats the same pattern: dry-run branch, generate branch, hooks branch. ~450 lines with lots of copy-paste.
- **Action:** Extract a helper like `run_generator(icon, name, dry_run_fn, generate_fn, hooks_fn)` or a macro. One refactor at a time.

### 4.2 `detect_project_type` — Unnecessary async

- Purely filesystem checks, no I/O that requires async. Marked `async` for no reason.
- **Action:** Make it `fn detect_project_type(&self) -> Result<ProjectType, InitiumError>` (sync). Callers can wrap in `block_in_place` or we keep the API async and just call it — low priority.

### 4.3 ConfigGenerator — Huge `mod.rs`

- `generators/mod.rs` is ~900 lines with many `#[allow(dead_code)]` test helpers returning static strings. These belong in test modules or a `templates` module.
- **Action:** Move `get_*_content` helpers to `tests/` or `src/generators/templates.rs`. Reduces lib surface and clarifies intent.

### 4.4 rustfmt vs user rule

- `rustfmt.toml`: `max_width = 100`
- User rule: "do not write lines longer than 80 characters"
- **Action:** Align: either `max_width = 80` in rustfmt, or update the rule to 100. Recommend 80 for consistency with rule.

---

## 5. Documentation Cleanup

### 5.1 `TODO.md`

- All items marked complete. Says "Total remaining work: ~0 minutes". Obsolete.
- **Action:** Delete or archive. If kept, trim to "Done. See CHANGELOG/ROADMAP."

### 5.2 `SETUP.md`

- References `just cargo-test`, `just cargo-test-coverage`, `just cargo-check`, etc. Actual justfile has `test`, `test-coverage`, `check`.
- **Action:** Update to `just test`, `just test-coverage`, `just check`. Audit all justfile references.

### 5.3 `CACHING.md` / `CI_OPTIMIZATION_SUMMARY.md`

- Reference `cache-setup.yml` and "optimized" workflows that don't exist. Current CI uses `ci.yml` only.
- **Action:** Either implement the referenced workflows or strip docs to match reality. Prefer stripping — CI works; no need for extra complexity.

### 5.4 `handle_list` typo (`commands.rs`)

- `.node-version (Ruby projects)` — `.node-version` is for Node (nvm). In initium it's generated for Ruby projects (frontend tooling). Minor confusion.
- **Action:** Clarify: ".node-version (Ruby projects, for frontend tooling)" or similar.

---

## 6. Nice-to-Have (Lower Priority)

- **PackageJson::from_template**: `rails`, `sinatra`, `gem` return Ruby-focused package.json; `express`, `react`, `default` return Node. They're mixed in one match. Consider splitting or renaming for clarity.
- **EditorConfig::default**: Has `enc/*` and `reg*.[ch]` sections — seem Ruby/stdlib-specific. Maybe move to a Ruby template.
- **PrettierConfig::fmt**: Manual JSON formatting; `serde_json::to_string_pretty` would be more robust.

---

## 7. Execution Order

1. **Immediate (bugs):** Fix error types, justfile `cargo get`, CI Rust version.
2. **Safety:** Fix or remove dangerous `clear-cache*` commands.
3. **Docs:** Update SETUP.md, slim CACHING/CI_OPTIMIZATION, handle TODO.md.
4. **Roadmap:** Remove ridiculous v2/v3 items.
5. **Refactor:** Extract command helpers, move test helpers, align rustfmt.

---

## 8. Verification

After changes:

```bash
just test
just clippy
just fmt-check
```

Manual smoke test:

```bash
just release-build  # ensure cargo get works
initium list
initium basic --dry-run
```
