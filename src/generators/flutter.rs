use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)] // Used in Task 10 (generate_flutter command)
    async fn generate_flutter_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_flutter_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }

    #[allow(dead_code)] // Used in Task 7 (generate_flutter_with_template)
    pub async fn generate_flutter_analysis_options(&self) -> Result<(), InitiumError> {
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

    #[allow(dead_code)] // Used in Task 7 (generate_flutter_with_template)
    pub async fn generate_flutter_gitignore(&self, template: &str) -> Result<(), InitiumError> {
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
}
