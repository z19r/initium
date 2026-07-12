use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)] // Used by Task 4 (generate_dart_with_template entry point)
    async fn generate_dart_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_dart_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }
}
