use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)] // Used by Task 4 (generate_dart_with_template entry point)
    async fn generate_dart_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_dart_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }

    #[allow(dead_code)] // Used by Task 4 (generate_dart_with_template entry point)
    pub async fn generate_dart_analysis_options(&self) -> Result<(), InitiumError> {
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

    #[allow(dead_code)] // Used by Task 4 (generate_dart_with_template entry point)
    pub async fn generate_dart_gitignore(&self, template: &str) -> Result<(), InitiumError> {
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
}
