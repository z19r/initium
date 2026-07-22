use crate::config::{EditorConfig, PrettierConfig};
use crate::error::InitiumError;

/// Paths Prettier should skip. Ruby/Rails projects carry framework-specific
/// build and vendor directories, so they get a richer ignore list than the
/// generic default.
fn prettier_ignore_content(template: &str) -> &'static str {
    match template {
        "ruby" | "rails" | "sinatra" | "gem" => {
            "node_modules\nvendor\ntmp\nlog\nstorage\ncoverage\npublic/assets\npublic/packs\n.gitignore\n*.min.js\n*.min.css\n"
        }
        _ => "node_modules\ndist\nbuild\ncoverage\n*.min.js\n*.min.css\n",
    }
}

impl super::ConfigGenerator {
    pub async fn generate_basic(&self, fail_on_exists: bool) -> Result<(), InitiumError> {
        self.generate_basic_with_template(fail_on_exists, "default")
            .await
    }

    pub async fn generate_basic_with_template(
        &self,
        fail_on_exists: bool,
        template: &str,
    ) -> Result<(), InitiumError> {
        let config = EditorConfig::default();
        self.emit_file(".editorconfig", &config.to_string(), fail_on_exists, false)
            .await?;

        let prettier = PrettierConfig::from_template(template);
        self.emit_file(".prettierrc", &prettier.to_string(), fail_on_exists, false)
            .await?;
        self.emit_file(
            ".prettierignore",
            prettier_ignore_content(template),
            fail_on_exists,
            false,
        )
        .await?;

        let justfile_content = r#"# Basic project justfile
# Add your project-specific commands here

# Default target
default:
    @echo "Available commands:"
    @just --list

# Install dependencies
install:
    @echo "Installing dependencies..."

# Run tests
test:
    @echo "Running tests..."

# Build project
build:
    @echo "Building project..."

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
"#;
        self.emit_file("justfile", justfile_content, fail_on_exists, false)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn generate_editor_config(&self, fail_on_exists: bool) -> Result<(), InitiumError> {
        let config = EditorConfig::default();
        self.emit_file(".editorconfig", &config.to_string(), fail_on_exists, false)
            .await
    }
}
