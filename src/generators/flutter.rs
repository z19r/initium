use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)] // Used in Task 10 (generate_flutter command)
    async fn generate_flutter_pubspec(&self, template: &str) -> Result<(), InitiumError> {
        let content = self.get_flutter_pubspec_content(template);
        self.emit_file("pubspec.yaml", content, false, false).await
    }
}
