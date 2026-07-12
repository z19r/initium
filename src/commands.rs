use crate::error::InitiumError;
use crate::generators::{hooks::GitHooksGenerator, ConfigGenerator, ProjectType};
use colored::*;
use std::path::PathBuf;

pub struct CommandHandler {
    target_dir: PathBuf,
    force: bool,
    fail_on_exists: bool,
    dry_run: bool,
    hooks: bool,
}

impl CommandHandler {
    pub fn new(
        target_dir: PathBuf,
        force: bool,
        fail_on_exists: bool,
        dry_run: bool,
        hooks: bool,
    ) -> Self {
        Self {
            target_dir,
            force,
            fail_on_exists,
            dry_run,
            hooks,
        }
    }

    fn make_generator(&self) -> ConfigGenerator {
        ConfigGenerator::with_options(self.target_dir.clone(), self.dry_run, self.force)
    }

    pub async fn handle_basic(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🚀 [DRY RUN] Would generate basic project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🚀 Generating basic project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator
            .generate_basic_with_template(self.fail_on_exists, template_name)
            .await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    "🪝 [DRY RUN] Would generate git hooks for basic project...".blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Basic configuration files generated successfully!".green()
            );

            if self.hooks {
                println!("{}", "🪝 Generating git hooks for basic project...".green());
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator.generate_basic_hooks(self.force).await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_ruby(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "💎 [DRY RUN] Would generate Ruby project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "💎 Generating Ruby project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_ruby_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Ruby project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Ruby configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Ruby project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_ruby_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_python(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🐍 [DRY RUN] Would generate Python project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🐍 Generating Python project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator
            .generate_python_with_template(template_name)
            .await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Python project (template: {})...",
                        template_name
                    ).blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Python configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Python project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_python_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_node(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🟢 [DRY RUN] Would generate Node.js project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🟢 Generating Node.js project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_node_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Node.js project (template: {})...",
                        template_name
                    ).blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Node.js configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Node.js project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_node_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_go(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🐹 [DRY RUN] Would generate Go project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🐹 Generating Go project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_go_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Go project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Go configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Go project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_go_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_rust(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🦀 [DRY RUN] Would generate Rust project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🦀 Generating Rust project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_rust_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Rust project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Rust configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Rust project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_rust_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_bash(&self, template: Option<String>) -> Result<(), InitiumError> {
        let template_name = template.as_deref().unwrap_or("default");
        let generator = self.make_generator();

        if self.dry_run {
            println!(
                "{}",
                format!(
                    "🐚 [DRY RUN] Would generate Bash project configuration (template: {})...",
                    template_name
                )
                .blue()
            );
        } else {
            println!(
                "{}",
                format!(
                    "🐚 Generating Bash project configuration (template: {})...",
                    template_name
                )
                .green()
            );
        }

        generator.generate_bash_with_template(template_name).await?;

        if self.dry_run {
            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 [DRY RUN] Would generate git hooks for Bash project (template: {})...",
                        template_name
                    )
                    .blue()
                );
            }
        } else {
            println!(
                "{}",
                "✅ Bash configuration files generated successfully!".green()
            );

            if self.hooks {
                println!(
                    "{}",
                    format!(
                        "🪝 Generating git hooks for Bash project (template: {})...",
                        template_name
                    )
                    .green()
                );
                let hooks_generator = GitHooksGenerator::new(self.target_dir.clone());
                hooks_generator
                    .generate_bash_hooks(template_name, self.force)
                    .await?;
                println!("{}", "✅ Git hooks generated successfully!".green());
            }
        }
        Ok(())
    }

    pub async fn handle_auto(&self) -> Result<(), InitiumError> {
        let generator = self.make_generator();

        if self.dry_run {
            println!("{}", "🔍 [DRY RUN] Auto-detecting project type...".blue());
        } else {
            println!("{}", "🔍 Auto-detecting project type...".blue());
        }

        let project_type = generator.detect_project_type().await?;
        self.handle_auto_generate(project_type, &generator).await?;
        Ok(())
    }

    async fn handle_auto_generate(
        &self,
        project_type: ProjectType,
        generator: &ConfigGenerator,
    ) -> Result<(), InitiumError> {
        match project_type {
            ProjectType::Ruby => {
                if self.dry_run {
                    println!(
                        "{}",
                        "💎 [DRY RUN] Would generate Ruby project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "💎 Detected Ruby project, generating configuration...".green()
                    );
                }
                generator.generate_ruby_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Ruby configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Python => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🐍 [DRY RUN] Would generate Python project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🐍 Detected Python project, generating configuration...".green()
                    );
                }
                generator.generate_python_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Python configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Node => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🟢 [DRY RUN] Would generate Node.js project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🟢 Detected Node.js project, generating configuration...".green()
                    );
                }
                generator.generate_node_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Node.js configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Go => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🐹 [DRY RUN] Would generate Go project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🐹 Detected Go project, generating configuration...".green()
                    );
                }
                generator.generate_go_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Go configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Rust => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🦀 [DRY RUN] Would generate Rust project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🦀 Detected Rust project, generating configuration...".green()
                    );
                }
                generator.generate_rust_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Rust configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Bash => {
                if self.dry_run {
                    println!(
                        "{}",
                        "🐚 [DRY RUN] Would generate Bash project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "🐚 Detected Bash project, generating configuration...".green()
                    );
                }
                generator.generate_bash_with_template("default").await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Bash configuration files generated successfully!".green()
                    );
                }
            }
            ProjectType::Basic => {
                if self.dry_run {
                    println!(
                        "{}",
                        "📁 [DRY RUN] Would generate basic project configuration...".blue()
                    );
                } else {
                    println!(
                        "{}",
                        "📁 Detected basic project, generating configuration...".green()
                    );
                }
                generator.generate_basic(self.fail_on_exists).await?;
                if !self.dry_run {
                    println!(
                        "{}",
                        "✅ Basic configuration files generated successfully!".green()
                    );
                }
            }
            // Full Dart/Flutter auto-generate wiring lands in Task 10
            ProjectType::Dart | ProjectType::Flutter => {}
        }
        Ok(())
    }

    pub async fn handle_interactive(&self) -> Result<(), InitiumError> {
        if self.dry_run {
            println!(
                "{}",
                "🎯 [DRY RUN] Interactive configuration setup...".blue()
            );
            println!("  Would launch interactive guided setup");
            return Ok(());
        }

        let generator = self.make_generator();
        println!("{}", "🎯 Interactive configuration setup...".blue());
        generator.interactive_setup().await?;
        Ok(())
    }

    pub fn handle_list(&self) {
        println!("{}", "📋 Available configuration files:".blue());
        println!("  • .editorconfig");
        println!("  • .prettierrc");
        println!("  • .prettierignore");
        println!("  • .ruby-version (Ruby projects)");
        println!("  • .node-version (Ruby projects, for frontend tooling)");
        println!("  • .rubocop.yml (Ruby projects)");
        println!("  • package.json (Ruby projects)");
        println!("  • .python-version (Python projects)");
        println!("  • pyproject.toml (Python projects)");
        println!("  • .flake8 (Python projects)");
        println!("  • requirements-dev.txt (Python projects)");
        println!("  • .nvmrc (Node.js projects)");
        println!("  • .eslintrc.json (Node.js projects)");
        println!("  • go.mod (Go projects)");
        println!("  • .golangci.yml (Go projects)");
        println!("  • rustfmt.toml (Rust projects)");
        println!("  • .clippy.toml (Rust projects)");
        println!("  • .cargo/config.toml (Rust projects)");
        println!("  • .shellcheckrc (Bash projects)");
        println!("  • justfile (all projects)");
        println!();
        println!("🪝 Available git hooks (with --hooks flag):");
        println!("  • pre-commit - Run linters, formatters, tests before commit");
        println!("  • pre-push - Run full test suite before push");
        println!("  • commit-msg - Validate commit message format");
        println!();
        println!("📋 Available templates:");
        println!("  • Basic: default, google, airbnb");
        println!("  • Ruby: default, rails, sinatra, gem");
        println!("  • Python: default, django, flask");
        println!("  • Node.js: default, express, react");
        println!("  • Go: default, web, cli");
        println!("  • Rust: default, web, cli");
        println!("  • Bash: default, devops, cli");
        println!();
        println!("🚀 Available commands:");
        println!("  • basic - Generate basic project configs");
        println!("  • ruby - Generate Ruby project configs");
        println!("  • python - Generate Python project configs");
        println!("  • node - Generate Node.js project configs");
        println!("  • go - Generate Go project configs");
        println!("  • rust - Generate Rust project configs");
        println!("  • bash - Generate Bash project configs");
        println!("  • auto - Auto-detect project type");
        println!("  • interactive - Guided setup");
        println!("  • list - Show this help");
        println!();
        println!("⚙️  Global options:");
        println!("  • --force - Overwrite existing files");
        println!("  • --dry-run - Show what would be created");
        println!("  • --hooks - Generate git hooks for the project");
        println!("  • --target DIR - Specify target directory");
    }
}
