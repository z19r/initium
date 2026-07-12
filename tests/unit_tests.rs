use initium::ConfigGenerator;
use initium::PackageJson;
use std::path::PathBuf;

#[test]
fn test_pyproject_toml_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_pyproject_toml_content("default");
    assert!(content.contains("line-length = 88"));
    assert!(content.contains("target-version = ['py312']"));
    assert!(content.contains("strict = true"));
    assert!(!content.contains("django_settings_module"));
    assert!(!content.contains("app_name = \"app\""));

    // Test Django template
    let content = generator.get_pyproject_toml_content("django");
    assert!(content.contains("django_settings_module"));
    assert!(content.contains("myproject.settings"));

    // Test Flask template
    let content = generator.get_pyproject_toml_content("flask");
    assert!(content.contains("app_name = \"app\""));
}

#[test]
fn test_eslint_config_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_eslint_config_content("default");
    assert!(content.contains("es2022"));
    assert!(content.contains("eslint:recommended"));
    assert!(!content.contains("no-console"));
    assert!(!content.contains("plugin:react/recommended"));

    // Test Express template
    let content = generator.get_eslint_config_content("express");
    assert!(content.contains("no-console"));
    assert!(content.contains("'off'"));

    // Test React template
    let content = generator.get_eslint_config_content("react");
    assert!(content.contains("plugin:react/recommended"));
    assert!(content.contains("react/prop-types"));
    assert!(content.contains("'off'"));
}

#[test]
fn test_node_package_json_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_node_package_json_content("default");
    assert!(content.contains("myproject"));
    assert!(content.contains("eslint"));
    assert!(content.contains("prettier"));
    assert!(!content.contains("express"));
    assert!(!content.contains("react"));

    // Test Express template
    let content = generator.get_node_package_json_content("express");
    assert!(content.contains("express"));
    assert!(content.contains("^4.18.2"));

    // Test React template
    let content = generator.get_node_package_json_content("react");
    assert!(content.contains("react"));
    assert!(content.contains("react-dom"));
    assert!(content.contains("^18.2.0"));
}

#[test]
fn test_python_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_python_justfile_content("default");
    assert!(content.contains("Python Project Justfile"));
    assert!(content.contains("python --version"));
    assert!(content.contains("black ."));
    assert!(content.contains("flake8 ."));
    assert!(content.contains("mypy ."));
    assert!(content.contains("pytest"));
    assert!(!content.contains("django-server"));
    assert!(!content.contains("flask-run"));

    // Test Django template
    let content = generator.get_python_justfile_content("django");
    assert!(content.contains("django-server"));
    assert!(content.contains("django-migrate"));
    assert!(content.contains("django-shell"));

    // Test Flask template
    let content = generator.get_python_justfile_content("flask");
    assert!(content.contains("flask-run"));
    assert!(content.contains("flask-shell"));
}

#[test]
fn test_node_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_node_justfile_content("default");
    assert!(content.contains("Node.js Project Justfile"));
    assert!(content.contains("node --version"));
    assert!(content.contains("npm install"));
    assert!(content.contains("npm test"));
    assert!(!content.contains("express-server"));
    assert!(!content.contains("react-dev"));

    // Test Express template
    let content = generator.get_node_justfile_content("express");
    assert!(content.contains("express-server"));
    assert!(content.contains("express-dev"));

    // Test React template
    let content = generator.get_node_justfile_content("react");
    assert!(content.contains("react-dev"));
    assert!(content.contains("react-build"));
}

#[test]
fn test_go_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_go_justfile_content("default");
    assert!(content.contains("Go Project Justfile"));
    assert!(content.contains("go version"));
    assert!(content.contains("go build"));
    assert!(content.contains("go run ."));
    assert!(content.contains("go test ./..."));
    assert!(content.contains("golangci-lint run"));
    assert!(!content.contains("go-web"));
    assert!(!content.contains("go-cli"));

    // Test web template
    let content = generator.get_go_justfile_content("web");
    assert!(content.contains("go-web"));

    // Test CLI template
    let content = generator.get_go_justfile_content("cli");
    assert!(content.contains("go-cli"));
}

#[test]
fn test_rust_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_rust_justfile_content("default");

    assert!(content.contains("Rust Project Justfile"));
    assert!(content.contains("rustc --version"));
    assert!(content.contains("cargo build"));
    assert!(content.contains("cargo run"));
    assert!(content.contains("cargo test"));
    assert!(content.contains("cargo clippy"));
    assert!(content.contains("cargo fmt"));
    assert!(!content.contains("cargo-web"));
    assert!(!content.contains("cargo run --bin cli"));

    // Test web template
    let content = generator.get_rust_justfile_content("web");
    assert!(content.contains("cargo-web"));

    // Test CLI template
    let content = generator.get_rust_justfile_content("cli");
    assert!(content.contains("cargo-cli"));
}

#[test]
fn test_ruby_justfile_content_generation() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_ruby_justfile_content("default");
    assert!(content.contains("Ruby Project Justfile"));
    assert!(content.contains("ruby --version"));
    assert!(content.contains("bundle install"));
    assert!(content.contains("bundle update"));
    assert!(content.contains("rubocop"));
    assert!(content.contains("prettier"));
    assert!(!content.contains("rails-server"));
    assert!(!content.contains("sinatra-server"));
    assert!(!content.contains("gem-build"));

    // Test Rails template
    let content = generator.get_ruby_justfile_content("rails");
    assert!(content.contains("rails-server"));
    assert!(content.contains("rails-console"));
    assert!(content.contains("rails-routes"));
    assert!(content.contains("rails-db-migrate"));

    // Test Sinatra template
    let content = generator.get_ruby_justfile_content("sinatra");
    assert!(content.contains("sinatra-server"));
    assert!(content.contains("sinatra-console"));
    assert!(content.contains("sinatra-test"));

    // Test Gem template
    let content = generator.get_ruby_justfile_content("gem");
    assert!(content.contains("gem-build"));
    assert!(content.contains("gem-install"));
    assert!(content.contains("gem-test"));
    assert!(content.contains("gem-release"));
}

#[test]
fn test_ruby_package_json_valid_json() {
    // Test that all Ruby templates produce valid JSON with proper closing braces
    let templates = ["rails", "sinatra", "gem"];

    for template in templates {
        let package_json = PackageJson::from_template(template);
        let json_string = package_json.to_string();

        // Verify the JSON string ends with closing braces
        assert!(
            json_string.ends_with("}}"),
            "PackageJson for template '{}' should end with '}}', got: '{}'",
            template,
            json_string
        );

        // Verify the JSON is valid by parsing it
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
        assert!(
            parsed.is_ok(),
            "PackageJson for template '{}' should be valid JSON, got error: {:?}, content: '{}'",
            template,
            parsed.err(),
            json_string
        );

        // Verify it contains expected fields
        let value = parsed.unwrap();
        assert!(
            value.get("name").is_some(),
            "PackageJson for template '{}' should have 'name' field",
            template
        );
        assert!(
            value.get("version").is_some(),
            "PackageJson for template '{}' should have 'version' field",
            template
        );
        assert!(
            value.get("devDependencies").is_some(),
            "PackageJson for template '{}' should have 'devDependencies' field",
            template
        );
    }
}

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
