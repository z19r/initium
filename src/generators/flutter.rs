use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_flutter(&self) -> Result<(), InitiumError> {
        self.generate_flutter_with_template("default").await
    }

    pub async fn generate_flutter_with_template(&self, template: &str) -> Result<(), InitiumError> {
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
