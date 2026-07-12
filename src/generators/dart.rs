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
