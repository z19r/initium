use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_bash(&self) -> Result<(), InitiumError> {
        self.generate_bash_with_template("default").await
    }

    pub async fn generate_bash_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Bash-specific configs
        self.generate_shellcheck_config().await?;

        // Overwrite the basic justfile with Bash-specific one
        self.generate_bash_justfile(template).await?;

        Ok(())
    }

    async fn generate_shellcheck_config(&self) -> Result<(), InitiumError> {
        let content = r#"# ShellCheck configuration
# See https://www.shellcheck.net/wiki/

# Default shell dialect
shell=bash

# Disable specific checks
# SC1091: Not following sourced files
# SC2034: Variable appears unused
disable=SC1091

# Enable optional checks
enable=require-variable-braces
enable=deprecate-which
enable=avoid-nullary-conditions
"#;
        self.emit_file(".shellcheckrc", content, false, false).await
    }

    async fn generate_bash_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "devops" => {
                r#"# Bash DevOps project justfile
default:
    @echo "Available Bash DevOps commands:"
    @just --list

# Run ShellCheck on all scripts
lint:
    @echo "Running ShellCheck..."
    @find . -name '*.sh' -not -path './vendor/*' -exec shellcheck {} +

# Format scripts with shfmt
fmt:
    @echo "Formatting Bash scripts..."
    @shfmt -w -i 2 -ci .

# Check formatting
fmt-check:
    @echo "Checking Bash formatting..."
    @shfmt -d -i 2 -ci .

# Run BATS tests
test:
    @echo "Running BATS tests..."
    @bats test/

# Validate scripts for syntax errors
check:
    @echo "Checking Bash syntax..."
    @find . -name '*.sh' -not -path './vendor/*' -exec bash -n {} +

# Deploy scripts (customize as needed)
deploy:
    @echo "Deploying scripts..."
    @echo "Configure deployment in justfile"
"#
            }
            "cli" => {
                r#"# Bash CLI project justfile
default:
    @echo "Available Bash CLI commands:"
    @just --list

# Run the CLI tool
run *ARGS:
    @echo "Running CLI tool..."
    @bash main.sh {{ARGS}}

# Run ShellCheck on all scripts
lint:
    @echo "Running ShellCheck..."
    @find . -name '*.sh' -not -path './vendor/*' -exec shellcheck {} +

# Format scripts with shfmt
fmt:
    @echo "Formatting Bash scripts..."
    @shfmt -w -i 2 -ci .

# Check formatting
fmt-check:
    @echo "Checking Bash formatting..."
    @shfmt -d -i 2 -ci .

# Run BATS tests
test:
    @echo "Running BATS tests..."
    @bats test/

# Validate scripts for syntax errors
check:
    @echo "Checking Bash syntax..."
    @find . -name '*.sh' -not -path './vendor/*' -exec bash -n {} +

# Install the CLI tool
install:
    @echo "Installing CLI tool..."
    @install -m 755 main.sh /usr/local/bin/
"#
            }
            _ => {
                r#"# Bash project justfile
default:
    @echo "Available Bash commands:"
    @just --list

# Run ShellCheck on all scripts
lint:
    @echo "Running ShellCheck..."
    @find . -name '*.sh' -not -path './vendor/*' -exec shellcheck {} +

# Format scripts with shfmt
fmt:
    @echo "Formatting Bash scripts..."
    @shfmt -w -i 2 -ci .

# Check formatting
fmt-check:
    @echo "Checking Bash formatting..."
    @shfmt -d -i 2 -ci .

# Run BATS tests
test:
    @echo "Running BATS tests..."
    @bats test/

# Validate scripts for syntax errors
check:
    @echo "Checking Bash syntax..."
    @find . -name '*.sh' -not -path './vendor/*' -exec bash -n {} +
"#
            }
        };
        self.emit_file("justfile", content, false, true).await
    }
}
