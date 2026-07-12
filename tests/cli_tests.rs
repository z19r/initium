use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("initium"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("initium"));
}

#[test]
fn test_cli_list_command() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available configuration files"))
        .stdout(predicate::str::contains("Available templates"))
        .stdout(predicate::str::contains("Available commands"));
}

#[test]
fn test_cli_basic_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("basic");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("basic project configuration"));
}

#[test]
fn test_cli_basic_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("basic")
        .arg("--template")
        .arg("google");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: google"));
}

#[test]
fn test_cli_ruby_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("ruby");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Ruby project configuration"));
}

#[test]
fn test_cli_ruby_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("ruby")
        .arg("--template")
        .arg("rails");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: rails"));
}

#[test]
fn test_cli_python_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("python");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Python project configuration"));
}

#[test]
fn test_cli_python_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("python")
        .arg("--template")
        .arg("django");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: django"));
}

#[test]
fn test_cli_node_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("node");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Node.js project configuration"));
}

#[test]
fn test_cli_node_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("node")
        .arg("--template")
        .arg("react");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: react"));
}

#[test]
fn test_cli_go_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("go");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Go project configuration"));
}

#[test]
fn test_cli_go_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("go")
        .arg("--template")
        .arg("web");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: web"));
}

#[test]
fn test_cli_rust_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("rust");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Rust project configuration"));
}

#[test]
fn test_cli_rust_command_with_template() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("rust")
        .arg("--template")
        .arg("cli");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("template: cli"));
}

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

#[test]
fn test_cli_auto_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("auto");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_cli_interactive_command() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("interactive");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"))
        .stdout(predicate::str::contains("Interactive configuration setup"));
}

#[test]
fn test_cli_force_flag() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--force")
        .arg("--dry-run")
        .arg("basic");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_cli_dry_run_flag() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("basic");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN"));
}

#[test]
fn test_cli_target_directory() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg(temp_dir.path())
        .arg("--dry-run")
        .arg("basic");

    cmd.assert().success();
}

#[test]
fn test_cli_invalid_directory() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target")
        .arg("/nonexistent/directory")
        .arg("basic");

    cmd.assert().failure();
}

#[test]
fn test_cli_file_as_target() {
    let temp_file = assert_fs::NamedTempFile::new("test.txt").unwrap();
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("--target").arg(temp_file.path()).arg("basic");

    cmd.assert().failure();
}

#[test]
fn test_cli_no_subcommand() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.assert().failure();
}

#[test]
fn test_cli_invalid_subcommand() {
    let mut cmd = Command::from_std(std::process::Command::new(assert_cmd::cargo::cargo_bin!(
        "initium"
    )));
    cmd.arg("invalid");

    cmd.assert().failure();
}
