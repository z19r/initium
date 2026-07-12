//! Full end-to-end tests: run initium for every command/template/flags,
//! dump output, assert generated files exist and are valid (JSON, TOML).

use assert_cmd::Command;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::path::Path;

fn initium_cmd() -> Command {
    Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )))
}

/// Assert file exists and contains valid JSON. On failure, dump content.
fn assert_valid_json(path: &Path, context: &str) {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: failed to read {:?}: {}", context, path, e));
    let parse_result: Result<serde_json::Value, _> = serde_json::from_str(&content);
    assert!(
        parse_result.is_ok(),
        "{}: invalid JSON in {:?}\nerror: {:?}\n--- content ---\n{}---",
        context,
        path,
        parse_result.err(),
        content
    );
}

/// Assert file exists and contains valid TOML. On failure, dump content.
fn assert_valid_toml(path: &Path, context: &str) {
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: failed to read {:?}: {}", context, path, e));
    let parse_result: Result<toml::Value, _> = toml::from_str(&content);
    assert!(
        parse_result.is_ok(),
        "{}: invalid TOML in {:?}\nerror: {:?}\n--- content ---\n{}---",
        context,
        path,
        parse_result.err(),
        content
    );
}

/// Run initium with args, assert success, return (stdout, stderr).
fn run_ok(temp: &TempDir, args: &[&str]) -> (String, String) {
    let mut cmd = initium_cmd();
    cmd.arg("--target").arg(temp.path());
    for a in args {
        cmd.arg(a);
    }
    let out = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "initium failed.\nargs: {:?}\nstdout:\n{}stderr:\n{}",
        args,
        stdout,
        stderr
    );
    (stdout, stderr)
}

// --- Basic ---

#[test]
fn e2e_basic_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["basic"]);

    let p = temp.path();
    assert!(p.join(".editorconfig").exists(), "missing .editorconfig");
    assert!(p.join(".prettierrc").exists(), "missing .prettierrc");
    assert!(
        p.join(".prettierignore").exists(),
        "missing .prettierignore"
    );
    assert!(p.join("justfile").exists(), "missing justfile");

    assert_valid_json(p.join(".prettierrc").as_path(), "basic default .prettierrc");
}

#[test]
fn e2e_basic_google() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["basic", "--template", "google"]);
    assert_valid_json(
        temp.path().join(".prettierrc").as_path(),
        "basic google .prettierrc",
    );
}

#[test]
fn e2e_basic_airbnb() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["basic", "--template", "airbnb"]);
    assert_valid_json(
        temp.path().join(".prettierrc").as_path(),
        "basic airbnb .prettierrc",
    );
}

// --- Ruby ---

#[test]
fn e2e_ruby_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["ruby"]);

    let p = temp.path();
    assert!(p.join("package.json").exists());
    assert_valid_json(
        p.join("package.json").as_path(),
        "ruby default package.json",
    );
    assert_valid_json(p.join(".prettierrc").as_path(), "ruby default .prettierrc");
}

#[test]
fn e2e_ruby_rails() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["ruby", "--template", "rails"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "ruby rails package.json",
    );
}

#[test]
fn e2e_ruby_sinatra() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["ruby", "--template", "sinatra"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "ruby sinatra package.json",
    );
}

#[test]
fn e2e_ruby_gem() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["ruby", "--template", "gem"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "ruby gem package.json",
    );
}

// --- Python ---

#[test]
fn e2e_python_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["python"]);

    let p = temp.path();
    assert!(p.join("pyproject.toml").exists());
    assert_valid_toml(
        p.join("pyproject.toml").as_path(),
        "python default pyproject.toml",
    );
    assert_valid_json(
        p.join(".prettierrc").as_path(),
        "python default .prettierrc",
    );
}

#[test]
fn e2e_python_django() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["python", "--template", "django"]);
    assert_valid_toml(
        temp.path().join("pyproject.toml").as_path(),
        "python django pyproject.toml",
    );
}

#[test]
fn e2e_python_flask() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["python", "--template", "flask"]);
    assert_valid_toml(
        temp.path().join("pyproject.toml").as_path(),
        "python flask pyproject.toml",
    );
}

// --- Node ---

#[test]
fn e2e_node_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["node"]);

    let p = temp.path();
    assert!(p.join("package.json").exists());
    assert!(p.join(".eslintrc.json").exists());
    assert_valid_json(
        p.join("package.json").as_path(),
        "node default package.json",
    );
    assert_valid_json(
        p.join(".eslintrc.json").as_path(),
        "node default .eslintrc.json",
    );
    assert_valid_json(p.join(".prettierrc").as_path(), "node default .prettierrc");
}

#[test]
fn e2e_node_express() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["node", "--template", "express"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "node express package.json",
    );
    assert_valid_json(
        temp.path().join(".eslintrc.json").as_path(),
        "node express .eslintrc.json",
    );
}

#[test]
fn e2e_node_react() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["node", "--template", "react"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "node react package.json",
    );
    assert_valid_json(
        temp.path().join(".eslintrc.json").as_path(),
        "node react .eslintrc.json",
    );
}

// --- Go --- (go.mod is not TOML, just assert exists + content)

#[test]
fn e2e_go_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["go"]);

    let p = temp.path();
    let go_mod = p.join("go.mod");
    assert!(go_mod.exists(), "missing go.mod");
    let content = std::fs::read_to_string(&go_mod).unwrap();
    assert!(
        content.contains("module myproject") && content.contains("go 1.21"),
        "go.mod missing expected content: {}",
        content
    );
    assert_valid_json(p.join(".prettierrc").as_path(), "go default .prettierrc");
}

#[test]
fn e2e_go_web() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["go", "--template", "web"]);
    assert!(temp.path().join("go.mod").exists());
}

#[test]
fn e2e_go_cli() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["go", "--template", "cli"]);
    assert!(temp.path().join("go.mod").exists());
}

// --- Rust ---

#[test]
fn e2e_rust_default() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["rust"]);

    let p = temp.path();
    assert!(p.join("rustfmt.toml").exists());
    assert!(p.join(".cargo/config.toml").exists());
    assert_valid_toml(
        p.join("rustfmt.toml").as_path(),
        "rust default rustfmt.toml",
    );
    assert_valid_toml(
        p.join(".cargo/config.toml").as_path(),
        "rust default .cargo/config.toml",
    );
    assert_valid_json(p.join(".prettierrc").as_path(), "rust default .prettierrc");
}

#[test]
fn e2e_rust_web() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["rust", "--template", "web"]);
    assert_valid_toml(
        temp.path().join("rustfmt.toml").as_path(),
        "rust web rustfmt.toml",
    );
}

#[test]
fn e2e_rust_cli() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["rust", "--template", "cli"]);
    assert_valid_toml(
        temp.path().join("rustfmt.toml").as_path(),
        "rust cli rustfmt.toml",
    );
}

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

// --- Auto ---

#[test]
fn e2e_auto_basic() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["auto"]);
    let p = temp.path();
    assert!(p.join(".editorconfig").exists());
    assert_valid_json(p.join(".prettierrc").as_path(), "auto basic .prettierrc");
}

#[test]
fn e2e_auto_ruby() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("Gemfile"), "gem 'rails'").unwrap();
    run_ok(&temp, &["auto"]);
    assert!(temp.path().join("package.json").exists());
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "auto ruby package.json",
    );
}

#[test]
fn e2e_auto_python() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("main.py"), "print(1)").unwrap();
    run_ok(&temp, &["auto"]);
    assert_valid_toml(
        temp.path().join("pyproject.toml").as_path(),
        "auto python pyproject.toml",
    );
}

#[test]
fn e2e_auto_node() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("package.json"), "{}").unwrap();
    run_ok(&temp, &["auto"]);
    assert_valid_json(
        temp.path().join("package.json").as_path(),
        "auto node package.json",
    );
    assert_valid_json(
        temp.path().join(".eslintrc.json").as_path(),
        "auto node .eslintrc.json",
    );
}

#[test]
fn e2e_auto_go() {
    let temp = TempDir::new().unwrap();
    std::fs::write(temp.path().join("main.go"), "package main").unwrap();
    run_ok(&temp, &["auto"]);
    assert!(temp.path().join("go.mod").exists());
}

#[test]
fn e2e_auto_rust() {
    let temp = TempDir::new().unwrap();
    std::fs::write(
        temp.path().join("Cargo.toml"),
        "[package]\nname=\"x\"\nversion=\"0.1\"",
    )
    .unwrap();
    run_ok(&temp, &["auto"]);
    assert_valid_toml(
        temp.path().join("rustfmt.toml").as_path(),
        "auto rust rustfmt.toml",
    );
}

// --- Global flags ---

#[test]
fn e2e_dry_run() {
    let temp = TempDir::new().unwrap();
    let (stdout, _) = run_ok(&temp, &["--dry-run", "basic"]);
    assert!(stdout.contains("DRY RUN"));
    assert!(!temp.path().join(".editorconfig").exists());
}

#[test]
fn e2e_force() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["basic"]);
    run_ok(&temp, &["--force", "basic"]);
    assert!(temp.path().join(".editorconfig").exists());
}

#[test]
fn e2e_fail_on_exists() {
    let temp = TempDir::new().unwrap();
    run_ok(&temp, &["basic"]);
    let mut cmd = initium_cmd();
    cmd.arg("--target")
        .arg(temp.path())
        .arg("--fail-on-exists")
        .arg("basic");
    let out = cmd.output().unwrap();
    assert!(
        !out.status.success(),
        "expected failure with --fail-on-exists"
    );
}

#[test]
fn e2e_target_short() {
    let temp = TempDir::new().unwrap();
    let mut cmd = initium_cmd();
    cmd.arg("-t").arg(temp.path()).arg("basic");
    cmd.assert().success();
    assert!(temp.path().join(".editorconfig").exists());
}

#[test]
fn e2e_list() {
    let mut cmd = initium_cmd();
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available configuration files"))
        .stdout(predicate::str::contains("Available templates"))
        .stdout(predicate::str::contains("basic"))
        .stdout(predicate::str::contains("ruby"))
        .stdout(predicate::str::contains("python"))
        .stdout(predicate::str::contains("node"))
        .stdout(predicate::str::contains("go"))
        .stdout(predicate::str::contains("rust"));
}

#[test]
fn e2e_invalid_dir_fails() {
    let mut cmd = initium_cmd();
    cmd.arg("--target")
        .arg("/nonexistent/path/xyz")
        .arg("basic");
    cmd.assert().failure();
}

#[test]
fn e2e_hooks_with_git() {
    let temp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp.path())
        .output()
        .expect("git init");
    run_ok(&temp, &["--hooks", "basic"]);
    let hooks_dir = temp.path().join(".git").join("hooks");
    assert!(
        hooks_dir.join("pre-commit").exists() || hooks_dir.join("pre-push").exists(),
        "expected git hooks in {:?}",
        hooks_dir
    );
}

#[test]
fn e2e_file_as_target_fails() {
    let f = assert_fs::NamedTempFile::new("x").unwrap();
    let mut cmd = initium_cmd();
    cmd.arg("--target").arg(f.path()).arg("basic");
    cmd.assert().failure();
}
