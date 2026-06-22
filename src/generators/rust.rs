use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_rust(&self) -> Result<(), InitiumError> {
        self.generate_rust_with_template("default").await
    }

    pub async fn generate_rust_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Rust-specific configs
        self.generate_rustfmt_config().await?;
        self.generate_clippy_config().await?;
        self.generate_cargo_config().await?;

        // Overwrite the basic justfile with Rust-specific one
        self.generate_rust_justfile(template).await?;

        Ok(())
    }

    async fn generate_rustfmt_config(&self) -> Result<(), InitiumError> {
        let content = r#"# Rustfmt configuration
edition = "2021"
max_width = 100
tab_spaces = 2
newline_style = "Unix"
use_small_heuristics = "Default"
"#;
        self.emit_file("rustfmt.toml", content, false, false).await
    }

    async fn generate_clippy_config(&self) -> Result<(), InitiumError> {
        let content = r#"# Clippy configuration
# Threshold settings for common lints

too-many-arguments-threshold = 8
too-many-lines-threshold = 150
type-complexity-threshold = 300
cognitive-complexity-threshold = 30
enum-variant-name-threshold = 4
max-struct-bools = 3
max-fn-params-bools = 3
"#;
        self.emit_file(".clippy.toml", content, false, false).await
    }

    async fn generate_cargo_config(&self) -> Result<(), InitiumError> {
        let content = r#"[build]
# Set the target directory
target = "target"

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]

[profile.dev]
# Development profile settings
opt-level = 0
debug = true
split-debuginfo = "unpacked"

[profile.release]
# Release profile settings
opt-level = 3
debug = false
lto = true
codegen-units = 1

[profile.test]
# Test profile settings
opt-level = 0
debug = true

[profile.bench]
# Benchmark profile settings
opt-level = 3
debug = false
lto = true
codegen-units = 1
"#;
        self.emit_file(".cargo/config.toml", content, false, false)
            .await
    }

    async fn generate_rust_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "web" => {
                r#"# Rust web project justfile
default:
    @echo "Available Rust web commands:"
    @just --list

# Run the web server
cargo-web:
    @echo "Running Rust web server..."
    @cargo run

# Build the web server
build:
    @echo "Building Rust web server..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust web server for release..."
    @cargo build --release

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install:
    @echo "Installing Rust dependencies..."
    @cargo build
"#
            }
            "cli" => {
                r#"# Rust CLI project justfile
default:
    @echo "Available Rust CLI commands:"
    @just --list

# Run the CLI tool
cargo-cli:
    @echo "Running Rust CLI tool..."
    @cargo run

# Build the CLI tool
build:
    @echo "Building Rust CLI tool..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust CLI tool for release..."
    @cargo build --release

# Install the CLI tool
install:
    @echo "Installing Rust CLI tool..."
    @cargo install --path .

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install-deps:
    @echo "Installing Rust dependencies..."
    @cargo build
"#
            }
            _ => {
                r#"# Rust project justfile
default:
    @echo "Available Rust commands:"
    @just --list

# Run the application
run:
    @echo "Running Rust application..."
    @cargo run

# Build the application
build:
    @echo "Building Rust application..."
    @cargo build

# Build for release
build-release:
    @echo "Building Rust application for release..."
    @cargo build --release

# Run tests
test:
    @echo "Running Rust tests..."
    @cargo test

# Run tests with coverage
test-coverage:
    @echo "Running Rust tests with coverage..."
    @cargo test --no-run
    @cargo llvm-cov --html

# Run linting
lint:
    @echo "Running Clippy..."
    @cargo clippy

# Format code
fmt:
    @echo "Formatting Rust code..."
    @cargo fmt

# Install dependencies
install:
    @echo "Installing Rust dependencies..."
    @cargo build
"#
            }
        };
        self.emit_file("justfile", content, false, true).await
    }
}
