use crate::error::InitiumError;
use std::fs;
use std::path::PathBuf;

#[allow(async_fn_in_trait)]
pub trait FileGenerator {
    fn target_dir(&self) -> &PathBuf;

    async fn write_file_if_not_exists(
        &self,
        filename: &str,
        content: &str,
        force: bool,
        fail_on_exists: bool,
    ) -> Result<(), InitiumError> {
        let file_path = self.target_dir().join(filename);

        if file_path.exists() && !force {
            if fail_on_exists {
                return Err(InitiumError::FileExists(file_path));
            } else {
                // Skip writing if the file exists and we're not forcing or failing
                return Ok(());
            }
        }

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)
            .map_err(|e| InitiumError::WriteFileError(file_path.clone(), e))?;
        Ok(())
    }
}

impl FileGenerator for super::ConfigGenerator {
    fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }
}

impl super::ConfigGenerator {
    pub async fn emit_file(
        &self,
        filename: &str,
        content: &str,
        fail_on_exists: bool,
        force_override: bool,
    ) -> Result<(), InitiumError> {
        use colored::*;

        if self.dry_run {
            let file_path = self.target_dir.join(filename);
            let effective_force = self.force || force_override;

            if file_path.exists() {
                if effective_force {
                    println!("  {} {}", "[OVERWRITE]".yellow(), filename);
                } else {
                    println!("  {} {} (already exists)", "[SKIP]".dimmed(), filename);
                    return Ok(());
                }
            } else {
                println!("  {} {}", "[CREATE]".green(), filename);
            }

            println!("  {}", "───────────────────────".dimmed());
            for line in content.lines() {
                println!("  {}", line.dimmed());
            }
            println!("  {}", "───────────────────────".dimmed());
            println!();

            return Ok(());
        }

        let effective_force = self.force || force_override;
        self.write_file_if_not_exists(filename, content, effective_force, fail_on_exists)
            .await
    }
}
