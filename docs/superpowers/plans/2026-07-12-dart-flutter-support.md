# Dart & Flutter Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `initium dart` and `initium flutter` CLI commands that bootstrap Dart/Flutter project configuration files (`pubspec.yaml`, `analysis_options.yaml`, `.gitignore`, `justfile`), following the exact generator/handler pattern already established for Ruby/Python/Node/Go/Rust/Bash.

**Architecture:** Two new generator modules (`src/generators/dart.rs`, `src/generators/flutter.rs`) implementing `generate_<lang>_with_template`, wired into `ProjectType` auto-detection (plain-text `pubspec.yaml` sniffing, no new YAML dependency), `Commands` enum dispatch in `src/main.rs`, `CommandHandler::handle_<lang>` in `src/commands.rs`, and git hook generators in `src/generators/hooks.rs`.

**Tech Stack:** Rust 2021, `clap` (derive), `tokio` (async), `thiserror`, `colored`, `serde_json` (dev-dep `toml`, `assert_fs`, `assert_cmd`, `tempfile`). No new dependencies are added.

## Global Constraints

- Rust 2021 edition, `rustfmt.toml`: max_width=100, tab_spaces=4, Unix newlines — run `cargo fmt --all` before every commit.
- `cargo clippy --all-targets --all-features -- -D warnings` must pass — all clippy warnings are treated as errors.
- No new dependencies — auto-detection uses plain substring sniffing of `pubspec.yaml`, not a YAML parser (spec: "Non-goals / deferred").
- Two separate commands, `dart` and `flutter` — not one command with a flavor template (spec: "Scope").
- Dart templates: `default`, `cli`, `package`. Flutter templates: `default`, `package`, `plugin` (spec: "initium dart" / "initium flutter").
- Both generators write `pubspec.yaml` themselves, like `go.mod`/`package.json` (spec: "Architecture").
- `.gitignore`: app/cli templates omit `pubspec.lock`; package/plugin templates include it (spec: "initium dart" / "initium flutter" `.gitignore` sections).
- Target 80%+ coverage on new code (spec: "Testing").
- Out of scope: platform folder scaffolding (android/, ios/, web/, example/), anything `flutter create`/`dart create` already generates beyond config files, FFI/native build tooling (spec: "Scope").

---

## Task 1: `ProjectType::Dart` / `ProjectType::Flutter` + auto-detection

**Files:**
- Modify: `src/generators/mod.rs:5-13` (module declarations), `src/generators/mod.rs:15-24` (`ProjectType` enum), `src/generators/mod.rs:87-103` (`detect_project_type`)
- Test: `tests/integration_tests.rs` (new tests near `test_project_type_detection`, currently at line 421)

**Interfaces:**
- Consumes: nothing new — pure additions to existing `ProjectType` enum and `ConfigGenerator::detect_project_type`.
- Produces: `ProjectType::Dart`, `ProjectType::Flutter` variants (used by Task 10's `handle_auto_generate`), `pub mod dart;` / `pub mod flutter;` declarations (required by Task 2/5 to compile).

- [ ] **Step 1: Write the failing tests**

Add to `tests/integration_tests.rs`, immediately after the closing brace of `test_project_type_detection` (after line 468, before `test_dry_run_modes`):

```rust
#[tokio::test]
async fn test_project_type_detection_dart() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    std::fs::write(
        temp_dir.child("pubspec.yaml").path(),
        "name: my_package\ndescription: A Dart project.\nversion: 0.1.0\n\nenvironment:\n  sdk: ^3.0.0\n",
    )
    .unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Dart));
}

#[tokio::test]
async fn test_project_type_detection_flutter_sdk_dependency() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    std::fs::write(
        temp_dir.child("pubspec.yaml").path(),
        "name: my_app\ndescription: A new Flutter project.\nversion: 0.1.0+1\n\ndependencies:\n  flutter:\n    sdk: flutter\n",
    )
    .unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Flutter));
}

#[tokio::test]
async fn test_project_type_detection_flutter_top_level_section() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    std::fs::write(
        temp_dir.child("pubspec.yaml").path(),
        "name: my_package\nenvironment:\n  sdk: ^3.0.0\n\nflutter:\n  uses-material-design: true\n",
    )
    .unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Flutter));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test integration_tests test_project_type_detection_dart test_project_type_detection_flutter -- --nocapture`
Expected: FAIL with a compile error — `no variant named 'Dart' found for enum 'initium::ProjectType'` (and `Flutter`).

- [ ] **Step 3: Add module declarations**

In `src/generators/mod.rs`, replace lines 5-13:

```rust
pub mod bash;
pub mod basic;
pub mod common;
pub mod go;
pub mod hooks;
pub mod node;
pub mod python;
pub mod ruby;
pub mod rust;
```

with:

```rust
pub mod bash;
pub mod basic;
pub mod common;
pub mod dart;
pub mod flutter;
pub mod go;
pub mod hooks;
pub mod node;
pub mod python;
pub mod ruby;
pub mod rust;
```

- [ ] **Step 4: Add `ProjectType` variants**

In `src/generators/mod.rs`, replace lines 15-24:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Basic,
    Ruby,
    Python,
    Node,
    Go,
    Rust,
    Bash,
}
```

with:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Basic,
    Ruby,
    Python,
    Node,
    Go,
    Rust,
    Bash,
    Dart,
    Flutter,
}
```

- [ ] **Step 5: Add detection logic**

In `src/generators/mod.rs`, the `detect_project_type` function currently reads (lines 87-103):

```rust
        // Check for Go project
        if self.target_dir.join("go.mod").exists()
            || self.target_dir.join("go.sum").exists()
            || self.target_dir.join("main.go").exists()
            || self.target_dir.join("cmd").exists()
            || self.target_dir.join("pkg").exists()
        {
            return Ok(ProjectType::Go);
        }

        // Check for Rust project
        if self.target_dir.join("Cargo.toml").exists()
            || self.target_dir.join("Cargo.lock").exists()
            || self.target_dir.join("main.rs").exists()
        {
            return Ok(ProjectType::Rust);
        }
```

Replace it with (inserting the Dart/Flutter check between Go and Rust):

```rust
        // Check for Go project
        if self.target_dir.join("go.mod").exists()
            || self.target_dir.join("go.sum").exists()
            || self.target_dir.join("main.go").exists()
            || self.target_dir.join("cmd").exists()
            || self.target_dir.join("pkg").exists()
        {
            return Ok(ProjectType::Go);
        }

        // Check for Dart/Flutter project
        if self.target_dir.join("pubspec.yaml").exists() {
            let pubspec_content =
                std::fs::read_to_string(self.target_dir.join("pubspec.yaml")).unwrap_or_default();
            if pubspec_content.contains("sdk: flutter") || pubspec_content.contains("\nflutter:")
            {
                return Ok(ProjectType::Flutter);
            }
            return Ok(ProjectType::Dart);
        }

        // Check for Rust project
        if self.target_dir.join("Cargo.toml").exists()
            || self.target_dir.join("Cargo.lock").exists()
            || self.target_dir.join("main.rs").exists()
        {
            return Ok(ProjectType::Rust);
        }
```

- [ ] **Step 6: Create empty placeholder generator files so the crate compiles**

Create `src/generators/dart.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {}
```

Create `src/generators/flutter.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {}
```

(These are filled in by Tasks 2-4 and 5-7 respectively. They exist now only so `pub mod dart;` / `pub mod flutter;` compile. The unused `InitiumError` import will be used starting in Task 2/5 — until then it triggers an unused-import warning, not a compile error, so `cargo test` still runs.)

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test --test integration_tests test_project_type_detection -- --nocapture`
Expected: PASS — all four `test_project_type_detection*` tests (the original plus the three new ones) succeed.

- [ ] **Step 8: Commit**

```bash
git add src/generators/mod.rs src/generators/dart.rs src/generators/flutter.rs tests/integration_tests.rs
git commit -m "feat: add ProjectType::Dart/Flutter and pubspec.yaml auto-detection"
```

---

## Task 2: Dart generator — `pubspec.yaml`

**Files:**
- Modify: `src/generators/dart.rs` (created empty in Task 1)
- Modify: `src/generators/mod.rs` (append test-helper content getter, before the final `}` at line 1311)
- Test: `tests/unit_tests.rs` (new test), `tests/generators_tests.rs` (new test)

**Interfaces:**
- Consumes: `ConfigGenerator::emit_file(&self, filename: &str, content: &str, fail_on_exists: bool, force_override: bool) -> Result<(), InitiumError>` (from `src/generators/common.rs`).
- Produces: `ConfigGenerator::get_dart_pubspec_content(&self, template: &str) -> &'static str` (test helper in `mod.rs`, consumed directly by this task's tests only — Task 4 wires the real `generate_dart_with_template` entry point that later tasks and CLI code call).

- [ ] **Step 1: Write the failing tests**

Add to `tests/unit_tests.rs`, after `test_ruby_package_json_valid_json` (end of file):

```rust
#[test]
fn test_dart_pubspec_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_dart_pubspec_content("default");
    assert!(content.contains("name: my_package"));
    assert!(content.contains("publish_to: 'none'"));
    assert!(content.contains("sdk: ^3.0.0"));
    assert!(!content.contains("executables:"));
    assert!(!content.contains("homepage:"));

    // Test cli template
    let content = generator.get_dart_pubspec_content("cli");
    assert!(content.contains("executables:"));
    assert!(content.contains("publish_to: 'none'"));

    // Test package template
    let content = generator.get_dart_pubspec_content("package");
    assert!(!content.contains("publish_to: 'none'"));
    assert!(content.contains("homepage:"));
    assert!(content.contains("repository:"));
}
```

Add to `tests/generators_tests.rs`, after `test_all_rust_templates` (line 76):

```rust
#[test]
fn test_all_dart_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    let templates = ["default", "cli", "package"];
    for template in templates {
        let content = generator.get_dart_pubspec_content(template);
        assert!(!content.is_empty());
        assert!(content.contains("environment:"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_dart_pubspec_content_generation test_all_dart_templates`
Expected: FAIL with `no method named 'get_dart_pubspec_content' found for struct 'ConfigGenerator'`.

- [ ] **Step 3: Add the test-helper content getter to `mod.rs`**

In `src/generators/mod.rs`, insert immediately before the final closing `}` (currently line 1311):

```rust

    #[allow(dead_code)]
    pub fn get_dart_pubspec_content(&self, template: &str) -> &'static str {
        match template {
            "cli" => {
                r#"name: my_package
description: A Dart project.
version: 0.1.0
publish_to: 'none'

environment:
  sdk: ^3.0.0

executables:
  my_package:

dev_dependencies:
  lints: ^4.0.0
  test: ^1.24.0
"#
            }
            "package" => {
                r#"name: my_package
description: A Dart project.
version: 0.1.0
homepage: https://github.com/your-org/my_package
repository: https://github.com/your-org/my_package

environment:
  sdk: ^3.0.0

dev_dependencies:
  lints: ^4.0.0
  test: ^1.24.0
"#
            }
            _ => {
                r#"name: my_package
description: A Dart project.
version: 0.1.0
publish_to: 'none'

environment:
  sdk: ^3.0.0

dev_dependencies:
  lints: ^4.0.0
  test: ^1.24.0
"#
            }
        }
    }
```

- [ ] **Step 4: Implement `generate_dart_pubspec` in `dart.rs`**

Replace the full contents of `src/generators/dart.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {
    async fn generate_dart_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_dart_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test test_dart_pubspec_content_generation test_all_dart_templates`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/generators/dart.rs src/generators/mod.rs tests/unit_tests.rs tests/generators_tests.rs
git commit -m "feat: add Dart pubspec.yaml generation"
```

---

## Task 3: Dart generator — `analysis_options.yaml` + `.gitignore`

**Files:**
- Modify: `src/generators/dart.rs`
- Test: `tests/integration_tests.rs` (new test)

**Interfaces:**
- Consumes: `ConfigGenerator::emit_file` (as Task 2).
- Produces: `generate_dart_analysis_options(&self) -> Result<(), InitiumError>`, `generate_dart_gitignore(&self, template: &str) -> Result<(), InitiumError>` — both private, called by Task 4's `generate_dart_with_template`.

- [ ] **Step 1: Write the failing test**

Add to `tests/integration_tests.rs`, after `test_project_type_detection_flutter_top_level_section` (added in Task 1):

```rust
#[tokio::test]
async fn test_generate_dart_analysis_options_and_gitignore() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_dart_analysis_options().await.unwrap();
    let analysis_options =
        std::fs::read_to_string(temp_dir.child("analysis_options.yaml").path()).unwrap();
    assert!(analysis_options.contains("package:lints/recommended.yaml"));
    assert!(analysis_options.contains("prefer_single_quotes"));

    // Default template: pubspec.lock is NOT ignored
    generator.generate_dart_gitignore("default").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains(".dart_tool/"));
    assert!(!gitignore.contains("pubspec.lock"));

    // Package template: pubspec.lock IS ignored
    std::fs::remove_file(temp_dir.child(".gitignore").path()).unwrap();
    generator.generate_dart_gitignore("package").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test integration_tests test_generate_dart_analysis_options_and_gitignore`
Expected: FAIL with `no method named 'generate_dart_analysis_options' found for struct 'ConfigGenerator'`.

- [ ] **Step 3: Implement `generate_dart_analysis_options` and `generate_dart_gitignore`**

In `src/generators/dart.rs`, add these two methods inside the existing `impl super::ConfigGenerator` block (after `generate_dart_pubspec`):

```rust

    async fn generate_dart_analysis_options(&self) -> Result<(), InitiumError> {
        let content = r#"include: package:lints/recommended.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
"#;
        self.emit_file("analysis_options.yaml", content, false, false)
            .await
    }

    async fn generate_dart_gitignore(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "package" => {
                r#".dart_tool/
.packages
build/
*.iml
.idea/
.vscode/
pubspec.lock
"#
            }
            _ => {
                r#".dart_tool/
.packages
build/
*.iml
.idea/
.vscode/
"#
            }
        };
        self.emit_file(".gitignore", content, false, false).await
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test integration_tests test_generate_dart_analysis_options_and_gitignore`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/generators/dart.rs tests/integration_tests.rs
git commit -m "feat: add Dart analysis_options.yaml and .gitignore generation"
```

---

## Task 4: Dart generator — `justfile` + `generate_dart_with_template` wiring

**Files:**
- Modify: `src/generators/dart.rs`
- Modify: `src/generators/mod.rs` (append test-helper content getter)
- Test: `tests/unit_tests.rs`, `tests/generators_tests.rs`, `tests/integration_tests.rs`

**Interfaces:**
- Consumes: `ConfigGenerator::generate_basic_with_template(&self, fail_on_exists: bool, template: &str) -> Result<(), InitiumError>` (from `src/generators/basic.rs`), `generate_dart_pubspec`/`generate_dart_analysis_options`/`generate_dart_gitignore` (Tasks 2-3).
- Produces: `pub async fn generate_dart_with_template(&self, template: &str) -> Result<(), InitiumError>` — this is the entry point Task 10's `handle_dart` and `handle_auto_generate` call. `pub async fn generate_dart(&self) -> Result<(), InitiumError>` (default-template convenience wrapper, matching every other language). `ConfigGenerator::get_dart_justfile_content(&self, template: &str) -> &'static str` (test helper).

- [ ] **Step 1: Write the failing tests**

Add to `tests/unit_tests.rs`, after `test_dart_pubspec_content_generation` (added in Task 2):

```rust
#[test]
fn test_dart_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_dart_justfile_content("default");
    assert!(content.contains("Dart Project Justfile"));
    assert!(content.contains("dart run"));
    assert!(content.contains("dart test"));
    assert!(content.contains("dart analyze"));
    assert!(!content.contains("dart compile exe"));
    assert!(!content.contains("publish-check"));

    // Test cli template
    let content = generator.get_dart_justfile_content("cli");
    assert!(content.contains("dart compile exe"));

    // Test package template
    let content = generator.get_dart_justfile_content("package");
    assert!(content.contains("publish-check"));
    assert!(content.contains("test-coverage"));
}
```

Add to `tests/generators_tests.rs`, after `test_all_dart_templates` (added in Task 2):

```rust
#[test]
fn test_dart_justfile_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    let content = generator.get_dart_justfile_content("default");
    assert!(content.contains("dart pub get"));

    let content = generator.get_dart_justfile_content("cli");
    assert!(content.contains("bin/main.dart"));

    let content = generator.get_dart_justfile_content("package");
    assert!(content.contains("dart pub publish --dry-run"));
}
```

Add to `tests/integration_tests.rs`, after `test_generate_dart_analysis_options_and_gitignore` (added in Task 3):

```rust
#[tokio::test]
async fn test_generate_dart_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_dart_with_template("default").await.unwrap();

    temp_dir.child(".editorconfig").assert(predicates::path::exists());
    temp_dir.child(".prettierrc").assert(predicates::path::exists());
    temp_dir.child("pubspec.yaml").assert(predicates::path::exists());
    temp_dir.child("analysis_options.yaml").assert(predicates::path::exists());
    temp_dir.child(".gitignore").assert(predicates::path::exists());
    temp_dir.child("justfile").assert(predicates::path::exists());

    let pubspec = std::fs::read_to_string(temp_dir.child("pubspec.yaml").path()).unwrap();
    assert!(pubspec.contains("name: my_package"));

    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(!gitignore.contains("pubspec.lock"));

    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("Dart Project Justfile"));
}

#[tokio::test]
async fn test_generate_dart_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_dart_with_template("cli").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("bin/main.dart"));

    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child("pubspec.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child("analysis_options.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();

    let force_generator =
        ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator.generate_dart_with_template("package").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test dart_justfile dart_config`
Expected: FAIL — `get_dart_justfile_content` and `generate_dart_with_template` do not exist yet.

- [ ] **Step 3: Add the test-helper justfile content getter to `mod.rs`**

In `src/generators/mod.rs`, insert immediately before the final closing `}`, after the `get_dart_pubspec_content` function added in Task 2:

```rust

    #[allow(dead_code)]
    pub fn get_dart_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "cli" => {
                r#"# Dart Project Justfile
default:
    @just --list

run *ARGS:
    @dart run bin/main.dart {{ARGS}}

build:
    @dart compile exe bin/main.dart -o bin/app

test:
    @dart test

fmt:
    @dart format .

fmt-check:
    @dart format --output=none --set-exit-if-changed .

analyze:
    @dart analyze

install:
    @dart pub get

update:
    @dart pub upgrade
"#
            }
            "package" => {
                r#"# Dart Project Justfile
default:
    @just --list

test:
    @dart test

test-coverage:
    @dart test --coverage=coverage
    @dart pub global run coverage:format_coverage --lcov --in=coverage --out=coverage/lcov.info --packages=.dart_tool/package_config.json --report-on=lib

fmt:
    @dart format .

fmt-check:
    @dart format --output=none --set-exit-if-changed .

analyze:
    @dart analyze

publish-check:
    @dart pub publish --dry-run

install:
    @dart pub get

update:
    @dart pub upgrade
"#
            }
            _ => {
                r#"# Dart Project Justfile
default:
    @just --list

run:
    @dart run

test:
    @dart test

fmt:
    @dart format .

fmt-check:
    @dart format --output=none --set-exit-if-changed .

analyze:
    @dart analyze

install:
    @dart pub get

update:
    @dart pub upgrade
"#
            }
        }
    }
```

- [ ] **Step 4: Implement `generate_dart_justfile` and `generate_dart_with_template` in `dart.rs`**

Replace the full contents of `src/generators/dart.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_dart(&self) -> Result<(), InitiumError> {
        self.generate_dart_with_template("default").await
    }

    pub async fn generate_dart_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Dart-specific configs
        self.generate_dart_pubspec(template).await?;
        self.generate_dart_analysis_options().await?;
        self.generate_dart_gitignore(template).await?;

        // Overwrite the basic justfile with Dart-specific one
        self.generate_dart_justfile(template).await?;

        Ok(())
    }

    async fn generate_dart_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_dart_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }

    async fn generate_dart_analysis_options(&self) -> Result<(), InitiumError> {
        let content = r#"include: package:lints/recommended.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
"#;
        self.emit_file("analysis_options.yaml", content, false, false)
            .await
    }

    async fn generate_dart_gitignore(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "package" => {
                r#".dart_tool/
.packages
build/
*.iml
.idea/
.vscode/
pubspec.lock
"#
            }
            _ => {
                r#".dart_tool/
.packages
build/
*.iml
.idea/
.vscode/
"#
            }
        };
        self.emit_file(".gitignore", content, false, false).await
    }

    async fn generate_dart_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_dart_justfile_content(template);
        self.emit_file("justfile", content, false, true).await
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test dart`
Expected: PASS — all Dart-related tests across `unit_tests.rs`, `generators_tests.rs`, `integration_tests.rs` succeed.

- [ ] **Step 6: Run full test suite and lint to check for regressions**

Run: `cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
Expected: All pass (Flutter's `impl super::ConfigGenerator {}` placeholder in `flutter.rs` from Task 1 remains empty — this is expected until Task 5).

- [ ] **Step 7: Commit**

```bash
git add src/generators/dart.rs src/generators/mod.rs tests/unit_tests.rs tests/generators_tests.rs tests/integration_tests.rs
git commit -m "feat: add Dart justfile generation and generate_dart_with_template entry point"
```

---

## Task 5: Flutter generator — `pubspec.yaml`

**Files:**
- Modify: `src/generators/flutter.rs`
- Modify: `src/generators/mod.rs` (append test-helper content getter)
- Test: `tests/unit_tests.rs`, `tests/generators_tests.rs`

**Interfaces:**
- Consumes: `ConfigGenerator::emit_file` (from `src/generators/common.rs`).
- Produces: `ConfigGenerator::get_flutter_pubspec_content(&self, template: &str) -> &'static str` (test helper).

- [ ] **Step 1: Write the failing tests**

Add to `tests/unit_tests.rs`, after `test_dart_justfile_content_generation` (added in Task 4):

```rust
#[test]
fn test_flutter_pubspec_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template (app)
    let content = generator.get_flutter_pubspec_content("default");
    assert!(content.contains("name: my_app"));
    assert!(content.contains("sdk: flutter"));
    assert!(content.contains("cupertino_icons"));
    assert!(content.contains("uses-material-design: true"));
    assert!(!content.contains("plugin_platform_interface"));

    // Test package template
    let content = generator.get_flutter_pubspec_content("package");
    assert!(!content.contains("cupertino_icons"));
    assert!(!content.contains("uses-material-design"));
    assert!(content.contains("homepage:"));
    assert!(!content.contains("plugin_platform_interface"));

    // Test plugin template
    let content = generator.get_flutter_pubspec_content("plugin");
    assert!(content.contains("plugin_platform_interface"));
    assert!(content.contains("pluginClass: MyPluginPlugin"));
}
```

Add to `tests/generators_tests.rs`, after `test_dart_justfile_templates` (added in Task 4):

```rust
#[test]
fn test_all_flutter_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    let templates = ["default", "package", "plugin"];
    for template in templates {
        let content = generator.get_flutter_pubspec_content(template);
        assert!(!content.is_empty());
        assert!(content.contains("environment:"));
        assert!(content.contains("sdk: flutter"));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_flutter_pubspec_content_generation test_all_flutter_templates`
Expected: FAIL with `no method named 'get_flutter_pubspec_content' found for struct 'ConfigGenerator'`.

- [ ] **Step 3: Add the test-helper content getter to `mod.rs`**

In `src/generators/mod.rs`, insert immediately before the final closing `}`, after the `get_dart_justfile_content` function added in Task 4:

```rust

    #[allow(dead_code)]
    pub fn get_flutter_pubspec_content(&self, template: &str) -> &'static str {
        match template {
            "package" => {
                r#"name: my_package
description: A Flutter package.
version: 0.1.0
homepage: https://github.com/your-org/my_package
repository: https://github.com/your-org/my_package

environment:
  sdk: ^3.0.0

dependencies:
  flutter:
    sdk: flutter

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^4.0.0

flutter:
"#
            }
            "plugin" => {
                r#"name: my_plugin
description: A Flutter plugin.
version: 0.1.0
homepage: https://github.com/your-org/my_plugin
repository: https://github.com/your-org/my_plugin

environment:
  sdk: ^3.0.0

dependencies:
  flutter:
    sdk: flutter
  plugin_platform_interface: ^2.0.2

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^4.0.0

flutter:
  plugin:
    platforms:
      android:
        package: com.example.my_plugin
        pluginClass: MyPluginPlugin
      ios:
        pluginClass: MyPluginPlugin
"#
            }
            _ => {
                r#"name: my_app
description: A new Flutter project.
version: 0.1.0+1
publish_to: 'none'

environment:
  sdk: ^3.0.0

dependencies:
  flutter:
    sdk: flutter
  cupertino_icons: ^1.0.6

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^4.0.0

flutter:
  uses-material-design: true
"#
            }
        }
    }
```

- [ ] **Step 4: Implement `generate_flutter_pubspec` in `flutter.rs`**

Replace the full contents of `src/generators/flutter.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {
    async fn generate_flutter_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_flutter_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test test_flutter_pubspec_content_generation test_all_flutter_templates`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/generators/flutter.rs src/generators/mod.rs tests/unit_tests.rs tests/generators_tests.rs
git commit -m "feat: add Flutter pubspec.yaml generation"
```

---

## Task 6: Flutter generator — `analysis_options.yaml` + `.gitignore`

**Files:**
- Modify: `src/generators/flutter.rs`
- Test: `tests/integration_tests.rs` (new test)

**Interfaces:**
- Consumes: `ConfigGenerator::emit_file` (as Task 5).
- Produces: `generate_flutter_analysis_options(&self) -> Result<(), InitiumError>`, `generate_flutter_gitignore(&self, template: &str) -> Result<(), InitiumError>` — both private, called by Task 7's `generate_flutter_with_template`.

- [ ] **Step 1: Write the failing test**

Add to `tests/integration_tests.rs`, after `test_generate_dart_config_with_templates` (added in Task 4):

```rust
#[tokio::test]
async fn test_generate_flutter_analysis_options_and_gitignore() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_flutter_analysis_options().await.unwrap();
    let analysis_options =
        std::fs::read_to_string(temp_dir.child("analysis_options.yaml").path()).unwrap();
    assert!(analysis_options.contains("package:flutter_lints/flutter.yaml"));
    assert!(analysis_options.contains("*.freezed.dart"));

    // Default template: pubspec.lock is NOT ignored
    generator.generate_flutter_gitignore("default").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains(".flutter-plugins"));
    assert!(!gitignore.contains("pubspec.lock"));

    // Plugin template: pubspec.lock IS ignored
    std::fs::remove_file(temp_dir.child(".gitignore").path()).unwrap();
    generator.generate_flutter_gitignore("plugin").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test --test integration_tests test_generate_flutter_analysis_options_and_gitignore`
Expected: FAIL with `no method named 'generate_flutter_analysis_options' found for struct 'ConfigGenerator'`.

- [ ] **Step 3: Implement `generate_flutter_analysis_options` and `generate_flutter_gitignore`**

In `src/generators/flutter.rs`, add these two methods inside the existing `impl super::ConfigGenerator` block (after `generate_flutter_pubspec`):

```rust

    async fn generate_flutter_analysis_options(&self) -> Result<(), InitiumError> {
        let content = r#"include: package:flutter_lints/flutter.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
    - "**/*.g.dart"
    - "**/*.freezed.dart"
"#;
        self.emit_file("analysis_options.yaml", content, false, false)
            .await
    }

    async fn generate_flutter_gitignore(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "package" | "plugin" => {
                r#"*.class
*.log
*.pyc
*.swp
.DS_Store
.buildlog/
.history
.svn/
migrate_working_dir/

*.iml
*.ipr
*.iws
.idea/

.dart_tool/
.flutter-plugins
.flutter-plugins-dependencies
.pub-cache/
.pub/
/build/

app.*.symbols
app.*.map.json
pubspec.lock
"#
            }
            _ => {
                r#"*.class
*.log
*.pyc
*.swp
.DS_Store
.buildlog/
.history
.svn/
migrate_working_dir/

*.iml
*.ipr
*.iws
.idea/

.dart_tool/
.flutter-plugins
.flutter-plugins-dependencies
.pub-cache/
.pub/
/build/

app.*.symbols
app.*.map.json
"#
            }
        };
        self.emit_file(".gitignore", content, false, false).await
    }
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test --test integration_tests test_generate_flutter_analysis_options_and_gitignore`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/generators/flutter.rs tests/integration_tests.rs
git commit -m "feat: add Flutter analysis_options.yaml and .gitignore generation"
```

---

## Task 7: Flutter generator — `justfile` + `generate_flutter_with_template` wiring

**Files:**
- Modify: `src/generators/flutter.rs`
- Modify: `src/generators/mod.rs` (append test-helper content getter)
- Test: `tests/unit_tests.rs`, `tests/generators_tests.rs`, `tests/integration_tests.rs`

**Interfaces:**
- Consumes: `ConfigGenerator::generate_basic_with_template`, `generate_flutter_pubspec`/`generate_flutter_analysis_options`/`generate_flutter_gitignore` (Tasks 5-6).
- Produces: `pub async fn generate_flutter_with_template(&self, template: &str) -> Result<(), InitiumError>` — the entry point Task 10's `handle_flutter` and `handle_auto_generate` call. `pub async fn generate_flutter(&self) -> Result<(), InitiumError>`. `ConfigGenerator::get_flutter_justfile_content(&self, template: &str) -> &'static str`.

- [ ] **Step 1: Write the failing tests**

Add to `tests/unit_tests.rs`, after `test_flutter_pubspec_content_generation` (added in Task 5):

```rust
#[test]
fn test_flutter_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_flutter_justfile_content("default");
    assert!(content.contains("Flutter Project Justfile"));
    assert!(content.contains("flutter run"));
    assert!(content.contains("flutter test"));
    assert!(content.contains("flutter build apk"));
    assert!(!content.contains("publish-check"));
    assert!(!content.contains("run-example"));

    // Test package template
    let content = generator.get_flutter_justfile_content("package");
    assert!(content.contains("publish-check"));
    assert!(!content.contains("flutter run"));
    assert!(!content.contains("run-example"));

    // Test plugin template
    let content = generator.get_flutter_justfile_content("plugin");
    assert!(content.contains("run-example"));
    assert!(content.contains("test-example"));
}
```

Add to `tests/generators_tests.rs`, after `test_all_flutter_templates` (added in Task 5):

```rust
#[test]
fn test_flutter_justfile_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    let content = generator.get_flutter_justfile_content("default");
    assert!(content.contains("flutter pub get"));

    let content = generator.get_flutter_justfile_content("package");
    assert!(content.contains("flutter pub publish --dry-run"));

    let content = generator.get_flutter_justfile_content("plugin");
    assert!(content.contains("cd example && flutter run"));
}
```

Add to `tests/integration_tests.rs`, after `test_generate_flutter_analysis_options_and_gitignore` (added in Task 6):

```rust
#[tokio::test]
async fn test_generate_flutter_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_flutter_with_template("default").await.unwrap();

    temp_dir.child(".editorconfig").assert(predicates::path::exists());
    temp_dir.child(".prettierrc").assert(predicates::path::exists());
    temp_dir.child("pubspec.yaml").assert(predicates::path::exists());
    temp_dir.child("analysis_options.yaml").assert(predicates::path::exists());
    temp_dir.child(".gitignore").assert(predicates::path::exists());
    temp_dir.child("justfile").assert(predicates::path::exists());

    let pubspec = std::fs::read_to_string(temp_dir.child("pubspec.yaml").path()).unwrap();
    assert!(pubspec.contains("name: my_app"));

    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(!gitignore.contains("pubspec.lock"));

    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("Flutter Project Justfile"));
}

#[tokio::test]
async fn test_generate_flutter_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_flutter_with_template("plugin").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("run-example"));

    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child("pubspec.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child("analysis_options.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();

    let force_generator =
        ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator.generate_flutter_with_template("package").await.unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test flutter_justfile flutter_config`
Expected: FAIL — `get_flutter_justfile_content` and `generate_flutter_with_template` do not exist yet.

- [ ] **Step 3: Add the test-helper justfile content getter to `mod.rs`**

In `src/generators/mod.rs`, insert immediately before the final closing `}`, after the `get_flutter_pubspec_content` function added in Task 5:

```rust

    #[allow(dead_code)]
    pub fn get_flutter_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "package" => {
                r#"# Flutter Project Justfile
default:
    @just --list

test:
    @flutter test

fmt:
    @dart format .

analyze:
    @flutter analyze

publish-check:
    @flutter pub publish --dry-run

install:
    @flutter pub get

clean:
    @flutter clean
"#
            }
            "plugin" => {
                r#"# Flutter Project Justfile
default:
    @just --list

test:
    @flutter test

fmt:
    @dart format .

analyze:
    @flutter analyze

publish-check:
    @flutter pub publish --dry-run

run-example:
    @cd example && flutter run

test-example:
    @cd example && flutter test

install:
    @flutter pub get

clean:
    @flutter clean
"#
            }
            _ => {
                r#"# Flutter Project Justfile
default:
    @just --list

run:
    @flutter run

test:
    @flutter test

fmt:
    @dart format .

analyze:
    @flutter analyze

build-apk:
    @flutter build apk

build-ios:
    @flutter build ios

install:
    @flutter pub get

clean:
    @flutter clean
"#
            }
        }
    }
```

- [ ] **Step 4: Implement `generate_flutter_justfile` and `generate_flutter_with_template` in `flutter.rs`**

Replace the full contents of `src/generators/flutter.rs`:

```rust
use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_flutter(&self) -> Result<(), InitiumError> {
        self.generate_flutter_with_template("default").await
    }

    pub async fn generate_flutter_with_template(
        &self,
        template: &str,
    ) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Flutter-specific configs
        self.generate_flutter_pubspec(template).await?;
        self.generate_flutter_analysis_options().await?;
        self.generate_flutter_gitignore(template).await?;

        // Overwrite the basic justfile with Flutter-specific one
        self.generate_flutter_justfile(template).await?;

        Ok(())
    }

    async fn generate_flutter_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_flutter_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }

    async fn generate_flutter_analysis_options(&self) -> Result<(), InitiumError> {
        let content = r#"include: package:flutter_lints/flutter.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
    - "**/*.g.dart"
    - "**/*.freezed.dart"
"#;
        self.emit_file("analysis_options.yaml", content, false, false)
            .await
    }

    async fn generate_flutter_gitignore(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "package" | "plugin" => {
                r#"*.class
*.log
*.pyc
*.swp
.DS_Store
.buildlog/
.history
.svn/
migrate_working_dir/

*.iml
*.ipr
*.iws
.idea/

.dart_tool/
.flutter-plugins
.flutter-plugins-dependencies
.pub-cache/
.pub/
/build/

app.*.symbols
app.*.map.json
pubspec.lock
"#
            }
            _ => {
                r#"*.class
*.log
*.pyc
*.swp
.DS_Store
.buildlog/
.history
.svn/
migrate_working_dir/

*.iml
*.ipr
*.iws
.idea/

.dart_tool/
.flutter-plugins
.flutter-plugins-dependencies
.pub-cache/
.pub/
/build/

app.*.symbols
app.*.map.json
"#
            }
        };
        self.emit_file(".gitignore", content, false, false).await
    }

    async fn generate_flutter_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_flutter_justfile_content(template);
        self.emit_file("justfile", content, false, true).await
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test flutter`
Expected: PASS — all Flutter-related tests across `unit_tests.rs`, `generators_tests.rs`, `integration_tests.rs` succeed.

- [ ] **Step 6: Run full test suite and lint to check for regressions**

Run: `cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
Expected: All pass.

- [ ] **Step 7: Commit**

```bash
git add src/generators/flutter.rs src/generators/mod.rs tests/unit_tests.rs tests/generators_tests.rs tests/integration_tests.rs
git commit -m "feat: add Flutter justfile generation and generate_flutter_with_template entry point"
```

---

## Task 8: Git hooks — Dart

**Files:**
- Modify: `src/generators/hooks.rs`

**Interfaces:**
- Consumes: `GitHooksGenerator::write_hook_file(&self, path: &PathBuf, content: &str, force: bool) -> Result<(), InitiumError>`, `GitHooksGenerator::get_commit_msg_hook(&self) -> String` (both already exist in `hooks.rs`).
- Produces: `pub async fn generate_dart_hooks(&self, template: &str, force: bool) -> Result<(), InitiumError>` — consumed by Task 10's `handle_dart`.

- [ ] **Step 1: Write the failing test**

Add to `tests/integration_tests.rs`, after `test_generate_flutter_config_with_templates` (added in Task 7):

```rust
#[tokio::test]
async fn test_generate_dart_hooks() {
    let temp_dir = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("git init");

    let hooks_generator =
        initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    hooks_generator.generate_dart_hooks("default", false).await.unwrap();

    let hooks_dir = temp_dir.path().join(".git").join("hooks");
    assert!(hooks_dir.join("pre-commit").exists());
    assert!(hooks_dir.join("pre-push").exists());
    assert!(hooks_dir.join("commit-msg").exists());

    let pre_commit = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(pre_commit.contains("dart format"));
    assert!(pre_commit.contains("dart analyze"));

    let pre_push = std::fs::read_to_string(hooks_dir.join("pre-push")).unwrap();
    assert!(pre_push.contains("dart test"));
}

#[tokio::test]
async fn test_generate_dart_hooks_without_git_fails() {
    let temp_dir = TempDir::new().unwrap();
    let hooks_generator =
        initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    let result = hooks_generator.generate_dart_hooks("default", false).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::GitNotInitialized
    ));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test integration_tests test_generate_dart_hooks`
Expected: FAIL with `no method named 'generate_dart_hooks' found for struct 'GitHooksGenerator'`.

- [ ] **Step 3: Implement `generate_dart_hooks` and its content getters**

In `src/generators/hooks.rs`, add a `generate_dart_hooks` method immediately after `generate_go_hooks` (which ends at line 116, right before `generate_rust_hooks` starts at line 118):

```rust

    pub async fn generate_dart_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), InitiumError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(InitiumError::GitNotInitialized);
        }

        let pre_commit_content = self.get_dart_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_dart_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }
```

Then, add the two content-getter methods in the private-methods section of `hooks.rs` — insert them immediately after the closing brace of `get_go_pre_push_hook` (the sibling function to `get_go_pre_commit_hook` seen at line 1000; both live in the "Go hooks" section of the file). Since hook content does not vary by template (spec: "Git hooks" — "Hook script content does not vary by template"), `_template` is intentionally unused:

```rust

    // Dart hooks (content does not vary by template)
    fn get_dart_pre_commit_hook(&self, _template: &str) -> String {
        r#"#!/bin/bash
# Dart Pre-commit Hook
set -e

echo "🔍 Running Dart pre-commit checks..."

if ! command -v dart &> /dev/null; then
    echo "❌ Dart not found. Please install the Dart SDK."
    exit 1
fi

echo "🎨 Checking formatting..."
dart format --output=none --set-exit-if-changed .

echo "🔍 Running dart analyze..."
dart analyze

echo "✅ Dart pre-commit checks passed!"
"#
        .to_string()
    }

    fn get_dart_pre_push_hook(&self, _template: &str) -> String {
        r#"#!/bin/bash
# Dart Pre-push Hook
set -e

echo "🚀 Running Dart pre-push checks..."

echo "🧪 Running Dart tests..."
dart test

echo "✅ Dart pre-push checks passed!"
"#
        .to_string()
    }
```

To find the exact insertion point, run `rg -n "fn get_go_pre_push_hook" src/generators/hooks.rs` and insert immediately after that function's closing `}`.

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test integration_tests test_generate_dart_hooks`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/generators/hooks.rs tests/integration_tests.rs
git commit -m "feat: add Dart git hooks generation"
```

---

## Task 9: Git hooks — Flutter

**Files:**
- Modify: `src/generators/hooks.rs`

**Interfaces:**
- Consumes: `GitHooksGenerator::write_hook_file`, `get_commit_msg_hook` (as Task 8).
- Produces: `pub async fn generate_flutter_hooks(&self, template: &str, force: bool) -> Result<(), InitiumError>` — consumed by Task 10's `handle_flutter`.

- [ ] **Step 1: Write the failing test**

Add to `tests/integration_tests.rs`, after `test_generate_dart_hooks_without_git_fails` (added in Task 8):

```rust
#[tokio::test]
async fn test_generate_flutter_hooks() {
    let temp_dir = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("git init");

    let hooks_generator =
        initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    hooks_generator.generate_flutter_hooks("default", false).await.unwrap();

    let hooks_dir = temp_dir.path().join(".git").join("hooks");
    assert!(hooks_dir.join("pre-commit").exists());
    assert!(hooks_dir.join("pre-push").exists());
    assert!(hooks_dir.join("commit-msg").exists());

    let pre_commit = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(pre_commit.contains("flutter analyze"));
    assert!(pre_commit.contains("dart format"));

    let pre_push = std::fs::read_to_string(hooks_dir.join("pre-push")).unwrap();
    assert!(pre_push.contains("flutter test"));
}

#[tokio::test]
async fn test_generate_flutter_hooks_without_git_fails() {
    let temp_dir = TempDir::new().unwrap();
    let hooks_generator =
        initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    let result = hooks_generator.generate_flutter_hooks("default", false).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::GitNotInitialized
    ));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test integration_tests test_generate_flutter_hooks`
Expected: FAIL with `no method named 'generate_flutter_hooks' found for struct 'GitHooksGenerator'`.

- [ ] **Step 3: Implement `generate_flutter_hooks` and its content getters**

In `src/generators/hooks.rs`, add a `generate_flutter_hooks` method immediately after the `generate_dart_hooks` method added in Task 8:

```rust

    pub async fn generate_flutter_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), InitiumError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(InitiumError::GitNotInitialized);
        }

        let pre_commit_content = self.get_flutter_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_flutter_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }
```

Then add the two content-getter methods immediately after the `get_dart_pre_push_hook` function added in Task 8:

```rust

    // Flutter hooks (content does not vary by template)
    fn get_flutter_pre_commit_hook(&self, _template: &str) -> String {
        r#"#!/bin/bash
# Flutter Pre-commit Hook
set -e

echo "🔍 Running Flutter pre-commit checks..."

if ! command -v flutter &> /dev/null; then
    echo "❌ Flutter not found. Please install the Flutter SDK."
    exit 1
fi

echo "🎨 Checking formatting..."
dart format --output=none --set-exit-if-changed .

echo "🔍 Running flutter analyze..."
flutter analyze

echo "✅ Flutter pre-commit checks passed!"
"#
        .to_string()
    }

    fn get_flutter_pre_push_hook(&self, _template: &str) -> String {
        r#"#!/bin/bash
# Flutter Pre-push Hook
set -e

echo "🚀 Running Flutter pre-push checks..."

echo "🧪 Running Flutter tests..."
flutter test

echo "✅ Flutter pre-push checks passed!"
"#
        .to_string()
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test --test integration_tests test_generate_flutter_hooks`
Expected: PASS.

- [ ] **Step 5: Run full test suite and lint to check for regressions**

Run: `cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
Expected: All pass.

- [ ] **Step 6: Commit**

```bash
git add src/generators/hooks.rs tests/integration_tests.rs
git commit -m "feat: add Flutter git hooks generation"
```

---

## Task 10: CLI wiring — `main.rs` + `commands.rs`

**Files:**
- Modify: `src/main.rs:44-94` (`Commands` enum), `src/main.rs:120-131` (dispatch match)
- Modify: `src/commands.rs` (add `handle_dart`/`handle_flutter`, extend `handle_auto_generate`, extend `handle_list`)
- Test: `tests/cli_tests.rs` (new tests)

**Interfaces:**
- Consumes: `ConfigGenerator::generate_dart_with_template`/`generate_flutter_with_template` (Tasks 4, 7), `GitHooksGenerator::generate_dart_hooks`/`generate_flutter_hooks` (Tasks 8, 9), `ProjectType::Dart`/`ProjectType::Flutter` (Task 1).
- Produces: `Commands::Dart { template: Option<String> }`, `Commands::Flutter { template: Option<String> }` (main.rs), `CommandHandler::handle_dart`, `CommandHandler::handle_flutter` (public, called from `main()`).

- [ ] **Step 1: Write the failing tests**

Add to `tests/cli_tests.rs`, after `test_cli_go_command_with_template` (line 217):

```rust
#[test]
fn test_cli_dart_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("dart");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Dart project configuration"));
}

#[test]
fn test_cli_dart_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("dart")
        .arg("--template")
        .arg("cli");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: cli"));
}

#[test]
fn test_cli_flutter_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("flutter");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Flutter project configuration"));
}

#[test]
fn test_cli_flutter_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("flutter")
        .arg("--template")
        .arg("plugin");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: plugin"));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test --test cli_tests test_cli_dart test_cli_flutter`
Expected: FAIL — `error: unrecognized subcommand 'dart'` (clap rejects the unknown subcommand since `Commands::Dart` doesn't exist yet).

- [ ] **Step 3: Add `Commands::Dart` / `Commands::Flutter` variants to `main.rs`**

In `src/main.rs`, replace lines 82-94:

```rust
    /// Generate configuration files for a Bash project
    Bash {
        /// Template to use (e.g., 'default', 'devops', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Automatically detect project type and generate appropriate configs
    Auto,
    /// Interactive mode - guided configuration setup
    Interactive,
    /// List all available configuration files
    List,
}
```

with:

```rust
    /// Generate configuration files for a Bash project
    Bash {
        /// Template to use (e.g., 'default', 'devops', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Dart project
    Dart {
        /// Template to use (e.g., 'default', 'cli', 'package')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Flutter project
    Flutter {
        /// Template to use (e.g., 'default', 'package', 'plugin')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Automatically detect project type and generate appropriate configs
    Auto,
    /// Interactive mode - guided configuration setup
    Interactive,
    /// List all available configuration files
    List,
}
```

- [ ] **Step 4: Add dispatch arms in `main()`**

In `src/main.rs`, replace lines 120-131:

```rust
    match cli.command {
        Commands::Basic { template } => handler.handle_basic(template).await?,
        Commands::Ruby { template } => handler.handle_ruby(template).await?,
        Commands::Python { template } => handler.handle_python(template).await?,
        Commands::Node { template } => handler.handle_node(template).await?,
        Commands::Go { template } => handler.handle_go(template).await?,
        Commands::Rust { template } => handler.handle_rust(template).await?,
        Commands::Bash { template } => handler.handle_bash(template).await?,
        Commands::Auto => handler.handle_auto().await?,
        Commands::Interactive => handler.handle_interactive().await?,
        Commands::List => handler.handle_list(),
    }

    Ok(())
}
```

with:

```rust
    match cli.command {
        Commands::Basic { template } => handler.handle_basic(template).await?,
        Commands::Ruby { template } => handler.handle_ruby(template).await?,
        Commands::Python { template } => handler.handle_python(template).await?,
        Commands::Node { template } => handler.handle_node(template).await?,
        Commands::Go { template } => handler.handle_go(template).await?,
        Commands::Rust { template } => handler.handle_rust(template).await?,
        Commands::Bash { template } => handler.handle_bash(template).await?,
        Commands::Dart { template } => handler.handle_dart(template).await?,
        Commands::Flutter { template } => handler.handle_flutter(template).await?,
        Commands::Auto => handler.handle_auto().await?,
        Commands::Interactive => handler.handle_interactive().await?,
        Commands::List => handler.handle_list(),
    }

    Ok(())
}
```

- [ ] **Step 5: Add `handle_dart` and `handle_flutter` to `CommandHandler`**

In `src/commands.rs`, insert immediately after `handle_bash` (which ends at line 456, right before `handle_auto` starts at line 458):

```rust

    pub async fn handle_dart(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🎯 [DRY RUN] Would generate Dart project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🎯 Generating Dart project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_dart_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Dart project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Dart configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Dart project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_dart_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_flutter(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🦋 [DRY RUN] Would generate Flutter project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🦋 Generating Flutter project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_flutter_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Flutter project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Flutter configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Flutter project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_flutter_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }
```

- [ ] **Step 6: Add `ProjectType::Dart` / `ProjectType::Flutter` arms to `handle_auto_generate`**

In `src/commands.rs`, the `handle_auto_generate` function's `ProjectType::Bash` arm currently reads (lines 578-597):

```rust
            ProjectType::Bash => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🐚 [DRY RUN] Would generate Bash project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🐚 Detected Bash project, generating configuration...".green()
                    );
                }
                generator.generate_bash_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Bash configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Basic => {
```

Replace it with (inserting Dart/Flutter arms between Bash and Basic):

```rust
            ProjectType::Bash => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🐚 [DRY RUN] Would generate Bash project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🐚 Detected Bash project, generating configuration...".green()
                    );
                }
                generator.generate_bash_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Bash configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Dart => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🎯 [DRY RUN] Would generate Dart project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🎯 Detected Dart project, generating configuration...".green()
                    );
                }
                generator.generate_dart_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Dart configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Flutter => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🦋 [DRY RUN] Would generate Flutter project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🦋 Detected Flutter project, generating configuration...".green()
                    );
                }
                generator.generate_flutter_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Flutter configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Basic => {
```

- [ ] **Step 7: Update `handle_list`**

In `src/commands.rs`, replace the `handle_list` function body (lines 638-692):

```rust
    pub fn handle_list(&self) {
        println!("{}", "📋 Available configuration files:".blue());
        println!("  • .editorconfig");
        println!("  • .prettierrc");
        println!("  • .prettierignore");
        println!("  • .ruby-version (Ruby projects)");
        println!("  • .node-version (Ruby projects, for frontend tooling)");
        println!("  • .rubocop.yml (Ruby projects)");
        println!("  • package.json (Ruby projects)");
        println!("  • .python-version (Python projects)");
        println!("  • pyproject.toml (Python projects)");
        println!("  • .flake8 (Python projects)");
        println!("  • requirements-dev.txt (Python projects)");
        println!("  • .nvmrc (Node.js projects)");
        println!("  • .eslintrc.json (Node.js projects)");
        println!("  • go.mod (Go projects)");
        println!("  • .golangci.yml (Go projects)");
        println!("  • rustfmt.toml (Rust projects)");
        println!("  • .clippy.toml (Rust projects)");
        println!("  • .cargo/config.toml (Rust projects)");
        println!("  • .shellcheckrc (Bash projects)");
        println!("  • justfile (all projects)");
        println!();
        println!("🪝 Available git hooks (with --hooks flag):");
        println!("  • pre-commit - Run linters, formatters, tests before commit");
        println!("  • pre-push - Run full test suite before push");
        println!("  • commit-msg - Validate commit message format");
        println!();
        println!("📋 Available templates:");
        println!("  • Basic: default, google, airbnb");
        println!("  • Ruby: default, rails, sinatra, gem");
        println!("  • Python: default, django, flask");
        println!("  • Node.js: default, express, react");
        println!("  • Go: default, web, cli");
        println!("  • Rust: default, web, cli");
        println!("  • Bash: default, devops, cli");
        println!();
        println!("🚀 Available commands:");
        println!("  • basic - Generate basic project configs");
        println!("  • ruby - Generate Ruby project configs");
        println!("  • python - Generate Python project configs");
        println!("  • node - Generate Node.js project configs");
        println!("  • go - Generate Go project configs");
        println!("  • rust - Generate Rust project configs");
        println!("  • bash - Generate Bash project configs");
        println!("  • auto - Auto-detect project type");
        println!("  • interactive - Guided setup");
        println!("  • list - Show this help");
        println!();
        println!("⚙️  Global options:");
        println!("  • --force - Overwrite existing files");
        println!("  • --dry-run - Show what would be created");
        println!("  • --hooks - Generate git hooks for the project");
        println!("  • --target DIR - Specify target directory");
    }
```

with:

```rust
    pub fn handle_list(&self) {
        println!("{}", "📋 Available configuration files:".blue());
        println!("  • .editorconfig");
        println!("  • .prettierrc");
        println!("  • .prettierignore");
        println!("  • .ruby-version (Ruby projects)");
        println!("  • .node-version (Ruby projects, for frontend tooling)");
        println!("  • .rubocop.yml (Ruby projects)");
        println!("  • package.json (Ruby projects)");
        println!("  • .python-version (Python projects)");
        println!("  • pyproject.toml (Python projects)");
        println!("  • .flake8 (Python projects)");
        println!("  • requirements-dev.txt (Python projects)");
        println!("  • .nvmrc (Node.js projects)");
        println!("  • .eslintrc.json (Node.js projects)");
        println!("  • go.mod (Go projects)");
        println!("  • .golangci.yml (Go projects)");
        println!("  • rustfmt.toml (Rust projects)");
        println!("  • .clippy.toml (Rust projects)");
        println!("  • .cargo/config.toml (Rust projects)");
        println!("  • .shellcheckrc (Bash projects)");
        println!("  • pubspec.yaml (Dart/Flutter projects)");
        println!("  • analysis_options.yaml (Dart/Flutter projects)");
        println!("  • justfile (all projects)");
        println!();
        println!("🪝 Available git hooks (with --hooks flag):");
        println!("  • pre-commit - Run linters, formatters, tests before commit");
        println!("  • pre-push - Run full test suite before push");
        println!("  • commit-msg - Validate commit message format");
        println!();
        println!("📋 Available templates:");
        println!("  • Basic: default, google, airbnb");
        println!("  • Ruby: default, rails, sinatra, gem");
        println!("  • Python: default, django, flask");
        println!("  • Node.js: default, express, react");
        println!("  • Go: default, web, cli");
        println!("  • Rust: default, web, cli");
        println!("  • Bash: default, devops, cli");
        println!("  • Dart: default, cli, package");
        println!("  • Flutter: default, package, plugin");
        println!();
        println!("🚀 Available commands:");
        println!("  • basic - Generate basic project configs");
        println!("  • ruby - Generate Ruby project configs");
        println!("  • python - Generate Python project configs");
        println!("  • node - Generate Node.js project configs");
        println!("  • go - Generate Go project configs");
        println!("  • rust - Generate Rust project configs");
        println!("  • bash - Generate Bash project configs");
        println!("  • dart - Generate Dart project configs");
        println!("  • flutter - Generate Flutter project configs");
        println!("  • auto - Auto-detect project type");
        println!("  • interactive - Guided setup");
        println!("  • list - Show this help");
        println!();
        println!("⚙️  Global options:");
        println!("  • --force - Overwrite existing files");
        println!("  • --dry-run - Show what would be created");
        println!("  • --hooks - Generate git hooks for the project");
        println!("  • --target DIR - Specify target directory");
    }
```

- [ ] **Step 8: Run tests to verify they pass**

Run: `cargo test --test cli_tests test_cli_dart test_cli_flutter`
Expected: PASS.

- [ ] **Step 9: Run full test suite and lint to check for regressions**

Run: `cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
Expected: All pass.

- [ ] **Step 10: Commit**

```bash
git add src/main.rs src/commands.rs tests/cli_tests.rs
git commit -m "feat: wire up dart and flutter CLI commands"
```

---

## Task 11: Integration tests — auto-detect + `--fail-on-exists` for Dart/Flutter

**Files:**
- Modify: `tests/integration_tests.rs`, `tests/fail_on_exists_tests.rs`

**Interfaces:**
- Consumes: `ConfigGenerator::detect_project_type` (Task 1), `generate_dart_with_template`/`generate_flutter_with_template` (Tasks 4, 7).
- Produces: nothing new — test-only task closing out the fail-on-exists/language-specific-override coverage the spec calls for.

- [ ] **Step 1: Write the failing tests**

Add to `tests/fail_on_exists_tests.rs`, after `test_fail_on_exists_rust_project` (line 109):

```rust
#[tokio::test]
async fn test_fail_on_exists_dart_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_dart_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_dart_with_template("default").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fail_on_exists_flutter_project() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // First generation should succeed
    let result = generator.generate_flutter_with_template("default").await;
    assert!(result.is_ok());

    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_flutter_with_template("default").await;
    assert!(result.is_ok());
}
```

Also update `test_fail_on_exists_file_specific_behavior` (lines 169-200) to include `pubspec.yaml` in its file list. Replace:

```rust
    // Test that each file type respects fail_on_exists
    let files = vec![
        ".editorconfig",
        ".prettierrc",
        ".prettierignore",
        "justfile",
        ".ruby-version",
        ".node-version",
        ".python-version",
        "go.mod",
        "rustfmt.toml",
    ];
```

with:

```rust
    // Test that each file type respects fail_on_exists
    let files = vec![
        ".editorconfig",
        ".prettierrc",
        ".prettierignore",
        "justfile",
        ".ruby-version",
        ".node-version",
        ".python-version",
        "go.mod",
        "rustfmt.toml",
        "pubspec.yaml",
    ];
```

Add to `tests/integration_tests.rs`, after `test_generate_flutter_hooks_without_git_fails` (added in Task 9):

```rust
#[tokio::test]
async fn test_auto_detect_generates_dart_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    std::fs::write(
        temp_dir.child("pubspec.yaml").path(),
        "name: my_package\nenvironment:\n  sdk: ^3.0.0\n",
    )
    .unwrap();

    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Dart));

    generator.generate_dart_with_template("default").await.unwrap();
    temp_dir.child("analysis_options.yaml").assert(predicates::path::exists());
}

#[tokio::test]
async fn test_auto_detect_generates_flutter_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    std::fs::write(
        temp_dir.child("pubspec.yaml").path(),
        "name: my_app\ndependencies:\n  flutter:\n    sdk: flutter\n",
    )
    .unwrap();

    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Flutter));

    generator.generate_flutter_with_template("default").await.unwrap();
    temp_dir.child("analysis_options.yaml").assert(predicates::path::exists());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test test_fail_on_exists_dart test_fail_on_exists_flutter test_auto_detect_generates_dart test_auto_detect_generates_flutter`
Expected: These should actually PASS immediately since Tasks 1-9 already implemented every method they call — there is no new production code in this task. Run them first to confirm this (if any fail, it indicates a regression in an earlier task that must be fixed before proceeding).

- [ ] **Step 3: N/A — no implementation step**

This task is test-only; all production code already exists from Tasks 1-10.

- [ ] **Step 4: Run full test suite to confirm no regressions**

Run: `cargo test`
Expected: All tests pass, including the ones added in this task and `test_fail_on_exists_file_specific_behavior`.

- [ ] **Step 5: Commit**

```bash
git add tests/fail_on_exists_tests.rs tests/integration_tests.rs
git commit -m "test: add fail-on-exists and auto-detect coverage for Dart/Flutter"
```

---

## Task 12: E2E tests — Dart/Flutter

**Files:**
- Modify: `tests/e2e_tests.rs`

**Interfaces:**
- Consumes: the full `initium` binary via `assert_cmd` (no new interfaces — this task only exercises the CLI end-to-end).
- Produces: nothing new — closes out the spec's "e2e_tests.rs — end-to-end scenarios, mirroring existing Go/Rust e2e cases" requirement.

- [ ] **Step 1: Write the failing tests**

Add to `tests/e2e_tests.rs`, after `e2e_rust_cli` (which ends right before the `// --- Auto ---` section, i.e. after line 313 based on the existing `e2e_rust_cli` at line 303):

```rust
// --- Dart ---

#[test]
fn e2e_dart_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["dart"]);
    let p = temp.path();
    let pubspec = p.join("pubspec.yaml");
    assert!(pubspec.exists(), "missing pubspec.yaml");
    let content = std::fs::read_to_string(&pubspec).unwrap();
    assert!(
        content.contains("name: my_package") && content.contains("publish_to: 'none'"),
        "unexpected pubspec.yaml content:\n{}",
        content
    );
    assert!(p.join("analysis_options.yaml").exists());
    let gitignore = std::fs::read_to_string(p.join(".gitignore")).unwrap();
    assert!(!gitignore.contains("pubspec.lock"));
    assert_valid_json(p.join(".prettierrc").as_path(), "dart default .prettierrc");
}

#[test]
fn e2e_dart_cli() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["dart", "--template", "cli"]);
    let justfile = std::fs::read_to_string(temp.path().join("justfile")).unwrap();
    assert!(justfile.contains("bin/main.dart"));
}

#[test]
fn e2e_dart_package() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["dart", "--template", "package"]);
    let gitignore = std::fs::read_to_string(temp.path().join(".gitignore")).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
    let pubspec = std::fs::read_to_string(temp.path().join("pubspec.yaml")).unwrap();
    assert!(!pubspec.contains("publish_to: 'none'"));
}

// --- Flutter ---

#[test]
fn e2e_flutter_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["flutter"]);
    let p = temp.path();
    let pubspec = p.join("pubspec.yaml");
    assert!(pubspec.exists(), "missing pubspec.yaml");
    let content = std::fs::read_to_string(&pubspec).unwrap();
    assert!(
        content.contains("name: my_app") && content.contains("sdk: flutter"),
        "unexpected pubspec.yaml content:\n{}",
        content
    );
    assert!(p.join("analysis_options.yaml").exists());
    let gitignore = std::fs::read_to_string(p.join(".gitignore")).unwrap();
    assert!(!gitignore.contains("pubspec.lock"));
}

#[test]
fn e2e_flutter_package() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["flutter", "--template", "package"]);
    let justfile = std::fs::read_to_string(temp.path().join("justfile")).unwrap();
    assert!(justfile.contains("publish-check"));
    let gitignore = std::fs::read_to_string(temp.path().join(".gitignore")).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}

#[test]
fn e2e_flutter_plugin() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["flutter", "--template", "plugin"]);
    let pubspec = std::fs::read_to_string(temp.path().join("pubspec.yaml")).unwrap();
    assert!(pubspec.contains("plugin_platform_interface"));
    let justfile = std::fs::read_to_string(temp.path().join("justfile")).unwrap();
    assert!(justfile.contains("run-example"));
}

// --- Auto: Dart / Flutter ---

#[test]
fn e2e_auto_dart() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("pubspec.yaml"),
        "name: my_package\nenvironment:\n  sdk: ^3.0.0\n",
    )
    .unwrap();
    run_ok(&temp, &["auto"]);
    assert!(temp.path().join("analysis_options.yaml").exists());
}

#[test]
fn e2e_auto_flutter() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("pubspec.yaml"),
        "name: my_app\ndependencies:\n  flutter:\n    sdk: flutter\n",
    )
    .unwrap();
    run_ok(&temp, &["auto"]);
    let justfile = std::fs::read_to_string(temp.path().join("justfile")).unwrap();
    assert!(justfile.contains("Flutter Project Justfile"));
}

// --- Hooks: Dart / Flutter ---

#[test]
fn e2e_hooks_dart() {
    let temp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp.path())
        .output()
        .expect("git init");
    run_ok(&temp, &["--hooks", "dart"]);
    let hooks_dir = temp.path().join(".git").join("hooks");
    assert!(
        hooks_dir.join("pre-commit").exists() && hooks_dir.join("pre-push").exists(),
        "expected Dart git hooks in {:?}",
        hooks_dir
    );
}

#[test]
fn e2e_hooks_flutter() {
    let temp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp.path())
        .output()
        .expect("git init");
    run_ok(&temp, &["--hooks", "flutter"]);
    let hooks_dir = temp.path().join(".git").join("hooks");
    assert!(
        hooks_dir.join("pre-commit").exists() && hooks_dir.join("pre-push").exists(),
        "expected Flutter git hooks in {:?}",
        hooks_dir
    );
}
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test --test e2e_tests e2e_dart e2e_flutter`
Expected: PASS immediately — like Task 11, this is test-only; every command these tests exercise (`initium dart`, `initium flutter`, `initium auto`, `initium --hooks dart/flutter`) already works after Task 10. If any fail, it indicates a regression in an earlier task that must be fixed before proceeding.

- [ ] **Step 3: Run full test suite and lint to check for regressions**

Run: `cargo test && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --all -- --check`
Expected: All pass.

- [ ] **Step 4: Commit**

```bash
git add tests/e2e_tests.rs
git commit -m "test: add end-to-end coverage for dart and flutter commands"
```

---

## Task 13: Final verification — full CI parity check

**Files:**
- None modified — verification-only task.

**Interfaces:**
- Consumes: everything built in Tasks 1-12.
- Produces: nothing — confirms the feature is complete and CI-green before considering the plan done.

- [ ] **Step 1: Run the full local CI pipeline**

Run: `just ci-local`
Expected: Lint (rustfmt + clippy) and the full test suite both pass with no warnings or failures.

- [ ] **Step 2: Manually smoke-test the CLI**

Run:
```bash
cd /tmp && rm -rf initium-smoke-dart initium-smoke-flutter
mkdir initium-smoke-dart && cargo run --manifest-path /home/zrk/code/initium/Cargo.toml -- --target /tmp/initium-smoke-dart dart --template package
mkdir initium-smoke-flutter && cargo run --manifest-path /home/zrk/code/initium/Cargo.toml -- --target /tmp/initium-smoke-flutter flutter --template plugin
ls -la /tmp/initium-smoke-dart /tmp/initium-smoke-flutter
cat /tmp/initium-smoke-dart/pubspec.yaml
cat /tmp/initium-smoke-flutter/pubspec.yaml
rm -rf /tmp/initium-smoke-dart /tmp/initium-smoke-flutter
```
Expected: Both directories contain `.editorconfig`, `.prettierrc`, `.prettierignore`, `pubspec.yaml`, `analysis_options.yaml`, `.gitignore`, `justfile`; the printed `pubspec.yaml` contents match the `package`/`plugin` template content from Tasks 2 and 5.

- [ ] **Step 3: Check test coverage meets the 80% target**

Run: `cargo tarpaulin --out Stdout 2>/dev/null | tail -30` (or the project's configured coverage tool per `.github/workflows/ci.yml`)
Expected: Coverage percentage for `src/generators/dart.rs`, `src/generators/flutter.rs`, and the new sections of `src/generators/mod.rs`/`src/generators/hooks.rs`/`src/commands.rs`/`src/main.rs` is at or above 80%, consistent with the spec's testing target.

- [ ] **Step 4: No commit**

This task is verification-only; nothing is modified, so there is nothing to commit. If Step 1, 2, or 3 surfaces a problem, fix it as a follow-up commit against the specific task that introduced it, then re-run this task's verification steps.

---

## Self-Review Notes

**Spec coverage:** Every section of `docs/superpowers/specs/2026-07-12-dart-flutter-support-design.md` maps to a task — `ProjectType`/auto-detection → Task 1; `initium dart` (pubspec.yaml, analysis_options.yaml, .gitignore, justfile) → Tasks 2-4; `initium flutter` → Tasks 5-7; Git hooks → Tasks 8-9; CLI wiring (`Commands` enum, `handle_dart`/`handle_flutter`, auto-detect arms, `handle_list`) → Task 10; Testing (unit/generators/integration/cli/fail_on_exists/e2e) → Tasks 1-12, spread across each task's own test additions plus Tasks 11-12 for the cross-cutting cases. Non-goals (no YAML dependency, no platform-folder generation) are respected throughout — no new `Cargo.toml` dependency is added, and no android/ios/web/example scaffolding is created.

**Placeholder scan:** Every step contains complete, concrete code — no `TODO`, `TBD`, or "similar to Task N" references. Every generator function, hook getter, and CLI handler is fully written out per task rather than summarized.

**Type consistency:** `generate_dart_with_template(&self, template: &str) -> Result<(), InitiumError>` (defined Task 4) is the exact signature called by `handle_dart` in Task 10 and `handle_auto_generate`'s `ProjectType::Dart` arm in Task 10. `generate_flutter_with_template` follows the same shape (Task 7 → Task 10). `GitHooksGenerator::generate_dart_hooks(&self, template: &str, force: bool) -> Result<(), InitiumError>` (Task 8) matches the call in Task 10's `handle_dart`; `generate_flutter_hooks` (Task 9) matches `handle_flutter` (Task 10). `get_dart_pubspec_content`/`get_dart_justfile_content`/`get_flutter_pubspec_content`/`get_flutter_justfile_content` are each defined once in `mod.rs` (Tasks 2, 4, 5, 7 respectively) and referenced by matching names in both their own task's generator code and later tasks' tests — no renaming drift.
