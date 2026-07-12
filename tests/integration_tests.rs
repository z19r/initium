use assert_fs::prelude::*;
use assert_fs::TempDir;
use initium::ConfigGenerator;

#[tokio::test]
async fn test_generate_basic_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test basic generation
    generator.generate_basic(true).await.unwrap();

    // Verify files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierignore")
        .assert(predicates::path::exists());
    // Verify .editorconfig content
    let editor_config = std::fs::read_to_string(temp_dir.child(".editorconfig").path()).unwrap();
    assert!(editor_config.contains("root = true"));
    assert!(editor_config.contains("charset = utf-8"));
    assert!(editor_config.contains("end_of_line = lf"));
    // Verify .prettierrc content
    let prettier_config = std::fs::read_to_string(temp_dir.child(".prettierrc").path()).unwrap();
    assert!(prettier_config.contains("\"semi\": true"));
    assert!(prettier_config.contains("\"singleQuote\": true"));

    let prettier_ignore =
        std::fs::read_to_string(temp_dir.child(".prettierignore").path()).unwrap();
    assert!(prettier_ignore.is_empty());
}

#[tokio::test]
async fn test_generate_ruby_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Ruby generation
    generator.generate_ruby().await.unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".ruby-version")
        .assert(predicates::path::exists());
    temp_dir
        .child(".node-version")
        .assert(predicates::path::exists());
    temp_dir
        .child(".rubocop.yml")
        .assert(predicates::path::exists());
    temp_dir
        .child("package.json")
        .assert(predicates::path::exists());
    // Verify Ruby-specific content
    let ruby_version = std::fs::read_to_string(temp_dir.child(".ruby-version").path()).unwrap();
    assert_eq!(ruby_version.trim(), "3.4.9");

    let node_version = std::fs::read_to_string(temp_dir.child(".node-version").path()).unwrap();
    assert_eq!(node_version.trim(), "25.9.0");

    let package_json = std::fs::read_to_string(temp_dir.child("package.json").path()).unwrap();
    assert!(package_json.contains("prettier-plugin-ruby"));
    assert!(package_json.contains("github:prettier/plugin-ruby"));

    let prettier_config = std::fs::read_to_string(temp_dir.child(".prettierrc").path()).unwrap();
    assert!(prettier_config.contains("@prettier/plugin-ruby"));
    assert!(prettier_config.contains("\"singleQuote\": false"));

    let rubocop_config = std::fs::read_to_string(temp_dir.child(".rubocop.yml").path()).unwrap();
    assert!(rubocop_config.contains("TargetRubyVersion: 3.3"));
    assert!(rubocop_config.contains("Max: 120"));
}

#[tokio::test]
async fn test_generate_python_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python generation
    generator
        .generate_python_with_template("default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".python-version")
        .assert(predicates::path::exists());
    temp_dir
        .child("pyproject.toml")
        .assert(predicates::path::exists());
    temp_dir.child(".flake8").assert(predicates::path::exists());
    temp_dir
        .child("requirements-dev.txt")
        .assert(predicates::path::exists());

    // Verify Python-specific content
    let python_version = std::fs::read_to_string(temp_dir.child(".python-version").path()).unwrap();
    assert_eq!(python_version.trim(), "3.12");

    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("line-length = 88"));
    assert!(pyproject_toml.contains("target-version = ['py312']"));
    assert!(pyproject_toml.contains("strict = true"));

    let flake8_config = std::fs::read_to_string(temp_dir.child(".flake8").path()).unwrap();
    assert!(flake8_config.contains("max-line-length = 88"));
    assert!(flake8_config.contains("extend-ignore = E203, W503"));

    let requirements_dev =
        std::fs::read_to_string(temp_dir.child("requirements-dev.txt").path()).unwrap();
    assert!(requirements_dev.contains("black=="));
    assert!(requirements_dev.contains("flake8=="));
    assert!(requirements_dev.contains("mypy=="));
    assert!(requirements_dev.contains("pytest=="));
}

#[tokio::test]
async fn test_generate_python_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Django template
    generator
        .generate_python_with_template("django")
        .await
        .unwrap();
    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("django_settings_module"));

    // Clean and test Flask template
    std::fs::remove_file(temp_dir.child("pyproject.toml").path()).unwrap();
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".python-version").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".flake8").path()).unwrap();
    std::fs::remove_file(temp_dir.child("requirements-dev.txt").path()).unwrap();
    generator
        .generate_python_with_template("flask")
        .await
        .unwrap();
    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("app_name = \"app\""));
}

#[tokio::test]
async fn test_generate_node_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Node.js generation
    generator
        .generate_node_with_template("default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir.child(".nvmrc").assert(predicates::path::exists());
    temp_dir
        .child(".eslintrc.json")
        .assert(predicates::path::exists());
    temp_dir
        .child("package.json")
        .assert(predicates::path::exists());

    // Verify Node.js-specific content
    let nvmrc = std::fs::read_to_string(temp_dir.child(".nvmrc").path()).unwrap();
    assert_eq!(nvmrc.trim(), "20");

    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.json").path()).unwrap();
    assert!(eslint_config.contains("es2021"));
    assert!(eslint_config.contains("browser"));
    assert!(eslint_config.contains("node"));

    let package_json = std::fs::read_to_string(temp_dir.child("package.json").path()).unwrap();
    assert!(package_json.contains("node-app"));
    assert!(package_json.contains("0.1.0"));
}

#[tokio::test]
async fn test_generate_node_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Express template — server-side, so no "browser" env
    generator
        .generate_node_with_template("express")
        .await
        .unwrap();
    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.json").path()).unwrap();
    assert!(eslint_config.contains("es2021"));
    assert!(eslint_config.contains("node"));
    assert!(eslint_config.contains("no-console"));

    // Clean and test React template — client-side, has "browser" and jsx
    std::fs::remove_file(temp_dir.child(".eslintrc.json").path()).unwrap();
    std::fs::remove_file(temp_dir.child("package.json").path()).unwrap();
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".nvmrc").path()).unwrap();
    generator
        .generate_node_with_template("react")
        .await
        .unwrap();
    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.json").path()).unwrap();
    assert!(eslint_config.contains("es2021"));
    assert!(eslint_config.contains("browser"));
    assert!(eslint_config.contains("jsx"));
}

#[tokio::test]
async fn test_generate_go_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Go generation
    generator
        .generate_go_with_template("default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir.child("go.mod").assert(predicates::path::exists());
    temp_dir
        .child(".golangci.yml")
        .assert(predicates::path::exists());

    // Verify Go-specific content
    let go_mod = std::fs::read_to_string(temp_dir.child("go.mod").path()).unwrap();
    assert!(go_mod.contains("module myproject"));
    assert!(go_mod.contains("go 1.21"));

    let golangci_config = std::fs::read_to_string(temp_dir.child(".golangci.yml").path()).unwrap();
    assert!(golangci_config.contains("gofmt"));
    assert!(golangci_config.contains("golint"));
    assert!(golangci_config.contains("govet"));
    assert!(golangci_config.contains("errcheck"));
}

#[tokio::test]
async fn test_generate_go_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test web template
    generator.generate_go_with_template("web").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("go-web"));

    // Clean and test CLI template
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child("go.mod").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".golangci.yml").path()).unwrap();
    // Note: .gitignore is updated, not created, so we don't remove it
    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator
        .generate_go_with_template("cli")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("go-cli"));
}

#[tokio::test]
async fn test_generate_rust_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Rust generation
    generator
        .generate_rust_with_template("default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child("rustfmt.toml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".clippy.toml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".cargo/config.toml")
        .assert(predicates::path::exists());

    // Verify Rust-specific content
    let rustfmt_config = std::fs::read_to_string(temp_dir.child("rustfmt.toml").path()).unwrap();
    assert!(rustfmt_config.contains("edition = \"2021\""));
    assert!(rustfmt_config.contains("max_width = 100"));

    let cargo_config =
        std::fs::read_to_string(temp_dir.child(".cargo/config.toml").path()).unwrap();
    assert!(cargo_config.contains("target-cpu=native"));
}

#[tokio::test]
async fn test_generate_rust_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test web template
    generator.generate_rust_with_template("web").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("cargo-web"));

    // Clean and test CLI template
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child("rustfmt.toml").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".clippy.toml").path()).unwrap();
    std::fs::remove_dir_all(temp_dir.child(".cargo").path()).unwrap();
    generator.generate_rust_with_template("cli").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("cargo-cli"));
}

#[tokio::test]
async fn test_generate_bash_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Bash generation
    generator
        .generate_bash_with_template("default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".shellcheckrc")
        .assert(predicates::path::exists());
    temp_dir
        .child("justfile")
        .assert(predicates::path::exists());

    // Verify Bash-specific content
    let shellcheckrc = std::fs::read_to_string(temp_dir.child(".shellcheckrc").path()).unwrap();
    assert!(shellcheckrc.contains("shell=bash"));
    assert!(shellcheckrc.contains("enable=require-variable-braces"));
    assert!(shellcheckrc.contains("disable=SC1091"));

    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("ShellCheck"));
    assert!(justfile.contains("shfmt"));
    assert!(justfile.contains("bats"));
}

#[tokio::test]
async fn test_generate_bash_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test devops template
    generator
        .generate_bash_with_template("devops")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("DevOps"));
    assert!(justfile.contains("deploy"));

    // Clean and test CLI template
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".shellcheckrc").path()).unwrap();
    generator.generate_bash_with_template("cli").await.unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("CLI"));
    assert!(justfile.contains("main.sh"));
    assert!(justfile.contains("install"));
}

#[tokio::test]
async fn test_project_type_detection() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python detection
    std::fs::write(temp_dir.child("main.py").path(), "print('Hello')").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Python));

    // Test Node.js detection
    std::fs::remove_file(temp_dir.child("main.py").path()).unwrap();
    std::fs::write(temp_dir.child("package.json").path(), "{}").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Node));

    // Test Go detection
    std::fs::remove_file(temp_dir.child("package.json").path()).unwrap();
    std::fs::write(temp_dir.child("main.go").path(), "package main").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Go));

    // Test Rust detection
    std::fs::remove_file(temp_dir.child("main.go").path()).unwrap();
    std::fs::write(temp_dir.child("main.rs").path(), "fn main() {}").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Rust));

    // Test Ruby detection
    std::fs::remove_file(temp_dir.child("main.rs").path()).unwrap();
    std::fs::write(
        temp_dir.child("Gemfile").path(),
        "source 'https://rubygems.org'",
    )
    .unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Ruby));

    // Test Bash detection
    std::fs::remove_file(temp_dir.child("Gemfile").path()).unwrap();
    std::fs::write(temp_dir.child(".shellcheckrc").path(), "shell=bash").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Bash));

    // Test basic detection (default)
    std::fs::remove_file(temp_dir.child(".shellcheckrc").path()).unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, initium::ProjectType::Basic));
}

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

#[tokio::test]
async fn test_generate_dart_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator
        .generate_dart_with_template("default")
        .await
        .unwrap();

    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child("pubspec.yaml")
        .assert(predicates::path::exists());
    temp_dir
        .child("analysis_options.yaml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".gitignore")
        .assert(predicates::path::exists());
    temp_dir
        .child("justfile")
        .assert(predicates::path::exists());

    let pubspec = std::fs::read_to_string(temp_dir.child("pubspec.yaml").path()).unwrap();
    assert!(pubspec.contains("name: my_package"));

    let analysis_options =
        std::fs::read_to_string(temp_dir.child("analysis_options.yaml").path()).unwrap();
    assert!(analysis_options.contains("package:lints/recommended.yaml"));
    assert!(analysis_options.contains("prefer_single_quotes"));

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

    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator
        .generate_dart_with_template("package")
        .await
        .unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}

#[tokio::test]
async fn test_generate_flutter_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator
        .generate_flutter_with_template("default")
        .await
        .unwrap();

    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child("pubspec.yaml")
        .assert(predicates::path::exists());
    temp_dir
        .child("analysis_options.yaml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".gitignore")
        .assert(predicates::path::exists());
    temp_dir
        .child("justfile")
        .assert(predicates::path::exists());

    let pubspec = std::fs::read_to_string(temp_dir.child("pubspec.yaml").path()).unwrap();
    assert!(pubspec.contains("name: my_app"));

    let analysis_options =
        std::fs::read_to_string(temp_dir.child("analysis_options.yaml").path()).unwrap();
    assert!(analysis_options.contains("package:flutter_lints/flutter.yaml"));
    assert!(analysis_options.contains("*.freezed.dart"));

    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(!gitignore.contains("pubspec.lock"));

    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("Flutter Project Justfile"));
}

#[tokio::test]
async fn test_generate_flutter_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator
        .generate_flutter_with_template("plugin")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("run-example"));

    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child("pubspec.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child("analysis_options.yaml").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();

    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator
        .generate_flutter_with_template("package")
        .await
        .unwrap();
    let gitignore = std::fs::read_to_string(temp_dir.child(".gitignore").path()).unwrap();
    assert!(gitignore.contains("pubspec.lock"));
}

#[tokio::test]
async fn test_dry_run_modes() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), true, false);

    // Test Python dry run
    generator
        .generate_python_with_template("default")
        .await
        .unwrap();

    // Test Node.js dry run
    generator
        .generate_node_with_template("default")
        .await
        .unwrap();

    // Test Go dry run
    generator
        .generate_go_with_template("default")
        .await
        .unwrap();

    // Test Rust dry run
    generator
        .generate_rust_with_template("default")
        .await
        .unwrap();

    // Test Bash dry run
    generator
        .generate_bash_with_template("default")
        .await
        .unwrap();

    // Verify no files were actually created
    assert!(!temp_dir.child(".python-version").exists());
    assert!(!temp_dir.child(".nvmrc").exists());
    assert!(!temp_dir.child("go.mod").exists());
    assert!(!temp_dir.child("rustfmt.toml").exists());
    assert!(!temp_dir.child(".shellcheckrc").exists());
}

#[tokio::test]
async fn test_force_overwrite_for_new_languages() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python force overwrite - language projects use fail_on_exists=false, so they succeed on second generation
    generator
        .generate_python_with_template("default")
        .await
        .unwrap();
    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_python_with_template("default").await;
    assert!(result.is_ok());
    // Force overwrite should also succeed
    let force_gen = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_gen
        .generate_python_with_template("default")
        .await
        .unwrap();

    // Test Node.js force overwrite - language projects use fail_on_exists=false, so they succeed on second generation
    let _ = std::fs::remove_file(temp_dir.child("package.json").path());
    let _ = std::fs::remove_file(temp_dir.child(".nvmrc").path());
    let _ = std::fs::remove_file(temp_dir.child(".eslintrc.json").path());
    let _ = std::fs::remove_file(temp_dir.child("justfile").path());
    let _ = std::fs::remove_file(temp_dir.child(".editorconfig").path());
    let _ = std::fs::remove_file(temp_dir.child(".prettierrc").path());
    generator
        .generate_node_with_template("default")
        .await
        .unwrap();
    // Second generation should succeed (fail_on_exists=false for language projects)
    let result = generator.generate_node_with_template("default").await;
    assert!(result.is_ok());
    // Force overwrite should also succeed
    let force_gen2 = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_gen2
        .generate_node_with_template("default")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_force_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Create initial config
    generator.generate_basic(true).await.unwrap();

    // Try to generate again without force - should fail
    let result = generator.generate_basic(true).await;
    assert!(result.is_err());

    // Generate with force - should succeed
    let force_generator = ConfigGenerator::with_options(temp_dir.path().to_path_buf(), false, true);
    force_generator.generate_basic(true).await.unwrap();
}

#[tokio::test]
async fn test_editor_config_sections() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_basic(true).await.unwrap();

    let editor_config = std::fs::read_to_string(temp_dir.child(".editorconfig").path()).unwrap();

    // Check for default section
    assert!(editor_config.contains("[*]"));
    assert!(editor_config.contains("indent_style = space"));
    assert!(editor_config.contains("indent_size = 2"));

    // Check for Ruby section
    assert!(editor_config.contains("[*.{rb,erb,ru,rake,gemspec}]"));

    // Check for JavaScript/TypeScript section
    assert!(editor_config.contains("[*.{yml,yaml,haml,jbuilder,jsx,html,sls,tf}]"));

    // makefiles
    assert!(editor_config.contains("[{*[Mm]akefile*,*.mak,*.mk,depend}]"));

    // enc/
    assert!(editor_config.contains("[enc/*]"));

    // reg  *.[ch]
    assert!(editor_config.contains("[reg*.[ch]]"));
}

#[tokio::test]
async fn test_generate_dart_hooks() {
    let temp_dir = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("git init");

    let hooks_generator = initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    hooks_generator
        .generate_dart_hooks("default", false)
        .await
        .unwrap();

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
    let hooks_generator = initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    let result = hooks_generator.generate_dart_hooks("default", false).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::GitNotInitialized
    ));
}

#[tokio::test]
async fn test_generate_flutter_hooks() {
    let temp_dir = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .current_dir(temp_dir.path())
        .output()
        .expect("git init");

    let hooks_generator = initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    hooks_generator
        .generate_flutter_hooks("default", false)
        .await
        .unwrap();

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
    let hooks_generator = initium::GitHooksGenerator::new(temp_dir.path().to_path_buf());
    let result = hooks_generator
        .generate_flutter_hooks("default", false)
        .await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        initium::error::InitiumError::GitNotInitialized
    ));
}

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

    generator
        .generate_dart_with_template("default")
        .await
        .unwrap();
    temp_dir
        .child("analysis_options.yaml")
        .assert(predicates::path::exists());
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

    generator
        .generate_flutter_with_template("default")
        .await
        .unwrap();
    temp_dir
        .child("analysis_options.yaml")
        .assert(predicates::path::exists());
}
