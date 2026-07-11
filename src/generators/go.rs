use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_go(&self) -> Result<(), InitiumError> {
        self.generate_go_with_template("default").await
    }

    pub async fn generate_go_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Go-specific configs
        self.generate_go_mod().await?;
        self.generate_golangci_config().await?;
        self.generate_go_gitignore().await?;

        // Overwrite the basic justfile with Go-specific one
        self.generate_go_justfile(template).await?;

        Ok(())
    }

    async fn generate_go_mod(&self) -> Result<(), InitiumError> {
        let content = r#"module myproject

go 1.21

require (
	// Add your Go dependencies here
)
"#;
        self.emit_file("go.mod", content, false, false).await
    }

    async fn generate_golangci_config(&self) -> Result<(), InitiumError> {
        let content = r#"run:
  timeout: 5m
  modules-download-mode: readonly

linters:
  enable:
    - gofmt
    - golint
    - govet
    - errcheck
    - staticcheck
    - gosimple
    - ineffassign
    - unused
    - misspell
    - gosec

linters-settings:
  govet:
    check-shadowing: true
  gocyclo:
    min-complexity: 15
  maligned:
    suggest-new: true
  dupl:
    threshold: 100
  goconst:
    min-len: 2
    min-occurrences: 3
  misspell:
    locale: US
  lll:
    line-length: 140

issues:
  exclude-rules:
    - path: _test\.go
      linters:
        - gomnd
        - dupl
        - gocyclo
        - goconst
        - gosec
"#;
        self.emit_file(".golangci.yml", content, false, false).await
    }

    async fn generate_go_gitignore(&self) -> Result<(), InitiumError> {
        let content = r#"# Binaries for programs and plugins
*.exe
*.exe~
*.dll
*.so
*.dylib

# Test binary, built with `go test -c`
*.test

# Output of the go coverage tool, specifically when used with LiteIDE
*.out

# Dependency directories (remove the comment below to include it)
# vendor/

# Go workspace file
go.work

# Go build cache
go-build/

# Go test cache
go-test/

# Go module cache
go-mod-cache/

# IDE files
.vscode/
.idea/
*.swp
*.swo
*~

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db
"#;
        self.emit_file(".gitignore", content, false, false).await
    }

    async fn generate_go_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "web" => {
                r#"# Go web project justfile
default:
    @echo "Available Go web commands:"
    @just --list

# Run the web server
go-web:
    @echo "Running Go web server..."
    @go run cmd/server/main.go

# Build the web server
build:
    @echo "Building Go web server..."
    @go build -o bin/server cmd/server/main.go

# Run tests
test:
    @echo "Running Go tests..."
    @go test ./...

# Run tests with coverage
test-coverage:
    @echo "Running Go tests with coverage..."
    @go test -coverprofile=coverage.out ./...
    @go tool cover -html=coverage.out

# Run linting
lint:
    @echo "Running golangci-lint..."
    @golangci-lint run

# Format code
fmt:
    @echo "Formatting Go code..."
    @go fmt ./...

# Install dependencies
install:
    @echo "Installing Go dependencies..."
    @go mod tidy
    @go mod download
"#
            }
            "cli" => {
                r#"# Go CLI project justfile
default:
    @echo "Available Go CLI commands:"
    @just --list

# Build the CLI tool
go-cli:
    @echo "Building Go CLI tool..."
    @go build -o bin/cli cmd/cli/main.go

# Install the CLI tool
install:
    @echo "Installing Go CLI tool..."
    @go install ./cmd/cli

# Run tests
test:
    @echo "Running Go tests..."
    @go test ./...

# Run tests with coverage
test-coverage:
    @echo "Running Go tests with coverage..."
    @go test -coverprofile=coverage.out ./...
    @go tool cover -html=coverage.out

# Run linting
lint:
    @echo "Running golangci-lint..."
    @golangci-lint run

# Format code
fmt:
    @echo "Formatting Go code..."
    @go fmt ./...

# Install dependencies
install-deps:
    @echo "Installing Go dependencies..."
    @go mod tidy
    @go mod download
"#
            }
            _ => {
                r#"# Go project justfile
default:
    @echo "Available Go commands:"
    @just --list

# Run the application
run:
    @echo "Running Go application..."
    @go run .

# Build the application
build:
    @echo "Building Go application..."
    @go build -o bin/app .

# Run tests
test:
    @echo "Running Go tests..."
    @go test ./...

# Run tests with coverage
test-coverage:
    @echo "Running Go tests with coverage..."
    @go test -coverprofile=coverage.out ./...
    @go tool cover -html=coverage.out

# Run linting
lint:
    @echo "Running golangci-lint..."
    @golangci-lint run

# Format code
fmt:
    @echo "Formatting Go code..."
    @go fmt ./...

# Install dependencies
install:
    @echo "Installing Go dependencies..."
    @go mod tidy
    @go mod download
"#
            }
        };
        self.emit_file("justfile", content, false, true).await
    }
}
