use initium::ConfigGenerator;
use std::path::PathBuf;

#[test]
fn test_all_python_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    // Test all Python templates
    let templates = ["default", "django", "flask"];
    for template in templates {
        let content = generator.get_pyproject_toml_content(template);
        assert!(!content.is_empty());

        let justfile = generator.get_python_justfile_content(template);
        assert!(!justfile.is_empty());
    }
}

#[test]
fn test_all_node_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    // Test all Node.js templates
    let templates = ["default", "express", "react"];
    for template in templates {
        let content = generator.get_eslint_config_content(template);
        assert!(!content.is_empty());

        let package_json = generator.get_node_package_json_content(template);
        assert!(!package_json.is_empty());

        let justfile = generator.get_node_justfile_content(template);
        assert!(!justfile.is_empty());
    }
}

#[test]
fn test_all_ruby_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    // Test all Ruby templates
    let templates = ["default", "rails", "sinatra", "gem"];
    for template in templates {
        let justfile = generator.get_ruby_justfile_content(template);
        assert!(!justfile.is_empty());
    }
}

#[test]
fn test_all_go_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    // Test all Go templates
    let templates = ["default", "web", "cli"];
    for template in templates {
        let justfile = generator.get_go_justfile_content(template);
        assert!(!justfile.is_empty());
    }
}

#[test]
fn test_all_rust_templates() {
    let temp_dir = std::env::temp_dir();
    let generator = ConfigGenerator::new(temp_dir);

    // Test all Rust templates
    let templates = ["default", "web", "cli"];
    for template in templates {
        let justfile = generator.get_rust_justfile_content(template);
        assert!(!justfile.is_empty());
    }
}

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

#[test]
fn test_pyproject_toml_templates() {
    let temp_dir = std::env::temp_dir();
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
fn test_eslint_config_templates() {
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
fn test_node_package_json_templates() {
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
fn test_python_justfile_templates() {
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
}

#[test]
fn test_node_justfile_templates() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_node_justfile_content("default");
    assert!(content.contains("Node.js Project Justfile"));
    assert!(content.contains("node --version"));
    assert!(content.contains("npm install"));
    assert!(content.contains("npm test"));
    assert!(content.contains("npm run lint"));

    // Test Express template
    let content = generator.get_node_justfile_content("express");
    assert!(content.contains("Node.js Project Justfile"));
    assert!(content.contains("npm-lint"));
    assert!(content.contains("npm-format"));

    // Test React template
    let content = generator.get_node_justfile_content("react");
    assert!(content.contains("Node.js Project Justfile"));
    assert!(content.contains("react-dev"));
    assert!(content.contains("react-build"));
}

#[test]
fn test_ruby_justfile_templates() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_ruby_justfile_content("default");
    assert!(content.contains("Ruby Project Justfile"));
    assert!(content.contains("ruby --version"));
    assert!(content.contains("bundle install"));

    // Test Rails template
    let content = generator.get_ruby_justfile_content("rails");
    assert!(content.contains("Ruby Project Justfile"));
    assert!(content.contains("rails-server"));
    assert!(content.contains("rails-console"));

    // Test Sinatra template
    let content = generator.get_ruby_justfile_content("sinatra");
    assert!(content.contains("Ruby Project Justfile"));
    assert!(content.contains("sinatra-server"));

    // Test Gem template
    let content = generator.get_ruby_justfile_content("gem");
    assert!(content.contains("Ruby Project Justfile"));
    assert!(content.contains("gem-build"));
}

#[test]
fn test_go_justfile_templates() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_go_justfile_content("default");
    assert!(content.contains("Go Project Justfile"));
    assert!(content.contains("go version"));
    assert!(content.contains("go mod tidy"));
    assert!(content.contains("go test"));
    assert!(content.contains("go build"));

    // Test Web template
    let content = generator.get_go_justfile_content("web");
    assert!(content.contains("Go Project Justfile"));
    assert!(content.contains("go-web"));

    // Test CLI template
    let content = generator.get_go_justfile_content("cli");
    assert!(content.contains("Go Project Justfile"));
    assert!(content.contains("go-cli"));
}

#[test]
fn test_rust_justfile_templates() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test default template
    let content = generator.get_rust_justfile_content("default");
    assert!(content.contains("Rust Project Justfile"));
    assert!(content.contains("rustc --version"));
    assert!(content.contains("cargo check"));
    assert!(content.contains("cargo test"));
    assert!(content.contains("cargo build"));

    // Test Web template
    let content = generator.get_rust_justfile_content("web");
    assert!(content.contains("Rust Project Justfile"));
    assert!(content.contains("cargo-web"));

    // Test CLI template
    let content = generator.get_rust_justfile_content("cli");
    assert!(content.contains("Rust Project Justfile"));
    assert!(content.contains("cargo-cli"));
}

#[test]
fn test_template_content_consistency() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test that all templates return non-empty content
    let templates = vec![
        ("python", vec!["default", "django", "flask"]),
        ("node", vec!["default", "express", "react"]),
        ("ruby", vec!["default", "rails", "sinatra", "gem"]),
        ("go", vec!["default", "web", "cli"]),
        ("rust", vec!["default", "web", "cli"]),
    ];

    for (lang, template_list) in templates {
        for template in template_list {
            match lang {
                "python" => {
                    let content = generator.get_python_justfile_content(template);
                    assert!(!content.is_empty(), "Python template {} is empty", template);
                }
                "node" => {
                    let content = generator.get_node_justfile_content(template);
                    assert!(!content.is_empty(), "Node template {} is empty", template);
                }
                "ruby" => {
                    let content = generator.get_ruby_justfile_content(template);
                    assert!(!content.is_empty(), "Ruby template {} is empty", template);
                }
                "go" => {
                    let content = generator.get_go_justfile_content(template);
                    assert!(!content.is_empty(), "Go template {} is empty", template);
                }
                "rust" => {
                    let content = generator.get_rust_justfile_content(template);
                    assert!(!content.is_empty(), "Rust template {} is empty", template);
                }
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn test_template_specific_content() {
    let temp_dir = PathBuf::from("/tmp");
    let generator = ConfigGenerator::new(temp_dir);

    // Test specific template content
    let django_content = generator.get_python_justfile_content("django");
    assert!(django_content.contains("django-server"));
    assert!(django_content.contains("django-migrate"));

    let react_content = generator.get_node_justfile_content("react");
    assert!(react_content.contains("npm run build"));

    let rails_content = generator.get_ruby_justfile_content("rails");
    assert!(rails_content.contains("rails server"));

    let web_content = generator.get_go_justfile_content("web");
    assert!(web_content.contains("go-web"));

    let cli_content = generator.get_rust_justfile_content("cli");
    assert!(cli_content.contains("cargo-cli"));
}
