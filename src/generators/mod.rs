use crate::error::InitiumError;
use std::path::PathBuf;

// Module declarations
pub mod bash;
pub mod basic;
pub mod common;
pub mod dart;
pub mod flutter;
pub mod go;
pub mod hooks;
pub mod node;
pub mod python;
pub mod ruby;
pub mod rust;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Basic,
    Ruby,
    Python,
    Node,
    Go,
    Rust,
    Bash,
    Dart,
    Flutter,
}

pub struct ConfigGenerator {
    target_dir: PathBuf,
    dry_run: bool,
    force: bool,
}

impl ConfigGenerator {
    #[allow(dead_code)]
    pub fn new(target_dir: PathBuf) -> Self {
        Self {
            target_dir,
            dry_run: false,
            force: false,
        }
    }

    pub fn with_options(target_dir: PathBuf, dry_run: bool, force: bool) -> Self {
        Self {
            target_dir,
            dry_run,
            force,
        }
    }

    pub async fn detect_project_type(&self) -> Result<ProjectType, InitiumError> {
        // Check for Ruby project
        if self.target_dir.join("Gemfile").exists()
            || self.target_dir.join("Gemfile.lock").exists()
            || self.target_dir.join("Rakefile").exists()
            || self.target_dir.join("config.ru").exists()
            || self.target_dir.join(".ruby-version").exists()
            || self.target_dir.join("main.rb").exists()
        {
            return Ok(ProjectType::Ruby);
        }

        // Check for Python project
        if self.target_dir.join("requirements.txt").exists()
            || self.target_dir.join("pyproject.toml").exists()
            || self.target_dir.join("setup.py").exists()
            || self.target_dir.join("Pipfile").exists()
            || self.target_dir.join("poetry.lock").exists()
            || self.target_dir.join("__pycache__").exists()
            || self.target_dir.join("main.py").exists()
            || self.target_dir.join("app.py").exists()
        {
            return Ok(ProjectType::Python);
        }

        // Check for Node.js project
        if self.target_dir.join("package.json").exists()
            || self.target_dir.join("package-lock.json").exists()
            || self.target_dir.join("yarn.lock").exists()
            || self.target_dir.join("pnpm-lock.yaml").exists()
            || self.target_dir.join("node_modules").exists()
            || self.target_dir.join("index.js").exists()
            || self.target_dir.join("app.js").exists()
        {
            return Ok(ProjectType::Node);
        }

        // Check for Go project
        if self.target_dir.join("go.mod").exists()
            || self.target_dir.join("go.sum").exists()
            || self.target_dir.join("main.go").exists()
            || self.target_dir.join("cmd").exists()
            || self.target_dir.join("pkg").exists()
        {
            return Ok(ProjectType::Go);
        }

        // Check for Dart/Flutter project
        if self.target_dir.join("pubspec.yaml").exists() {
            let pubspec_content =
                std::fs::read_to_string(self.target_dir.join("pubspec.yaml")).unwrap_or_default();
            if pubspec_content.contains("sdk: flutter") || pubspec_content.contains("\nflutter:")
            {
                return Ok(ProjectType::Flutter);
            }
            return Ok(ProjectType::Dart);
        }

        // Check for Rust project
        if self.target_dir.join("Cargo.toml").exists()
            || self.target_dir.join("Cargo.lock").exists()
            || self.target_dir.join("main.rs").exists()
        {
            return Ok(ProjectType::Rust);
        }

        // Check for Bash project
        if self.target_dir.join(".shellcheckrc").exists()
            || self.target_dir.join(".bats").exists()
            || self.target_dir.join("main.sh").exists()
        {
            return Ok(ProjectType::Bash);
        }

        // Default to basic project
        Ok(ProjectType::Basic)
    }

    pub async fn interactive_setup(&self) -> Result<(), InitiumError> {
        // TODO: Implement interactive setup
        println!("🎯 Interactive setup not yet implemented, generating basic configuration...");

        self.generate_basic(false).await?;

        Ok(())
    }

    // Test helper methods to get content without writing files
    #[allow(dead_code)]
    pub fn get_basic_justfile_content(&self) -> &'static str {
        r#"# Basic project justfile
# Add your project-specific commands here

# Default target
default:
    @echo "Available commands:"
    @just --list

# Install dependencies
install:
    @echo "Installing dependencies..."

# Run tests
test:
    @echo "Running tests..."

# Build project
build:
    @echo "Building project..."

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
"#
    }

    #[allow(dead_code)]
    pub fn get_ruby_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "rails" => {
                r#"# Ruby Project Justfile
default:
    @echo "Available Rails commands:"
    @just --list

# Start Rails server
rails-server:
    @echo "Starting Rails server..."
    @bundle exec rails server

# Run tests
test:
    @echo "Running Rails tests..."
    @bundle exec rails test

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Database operations
rails-db-migrate:
    @echo "Running database migrations..."
    @bundle exec rails db:migrate

# Rails console
rails-console:
    @echo "Starting Rails console..."
    @bundle exec rails console

# Rails routes
rails-routes:
    @echo "Showing Rails routes..."
    @bundle exec rails routes

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            "sinatra" => {
                r#"# Ruby Project Justfile
default:
    @echo "Available Sinatra commands:"
    @just --list

# Start Sinatra server
sinatra-server:
    @echo "Starting Sinatra server..."
    @bundle exec ruby app.rb

# Open Sinatra console
sinatra-console:
    @echo "Starting Sinatra console..."
    @bundle exec irb -r ./app.rb

# Run tests
test:
    @echo "Running Sinatra tests..."
    @bundle exec rspec

# Sinatra-specific test task
sinatra-test:
    @echo "Running Sinatra tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            "gem" => {
                r#"# Ruby Project Justfile
default:
    @echo "Available gem commands:"
    @just --list

# Build gem
gem-build:
    @echo "Building gem..."
    @gem build *.gemspec

# Install gem locally
gem-install:
    @echo "Installing gem locally..."
    @gem install *.gem

# Run tests
gem-test:
    @echo "Running gem tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Release gem
gem-release:
    @echo "Releasing gem to RubyGems..."
    @gem push *.gem

# Install dependencies
install-deps:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            _ => {
                r#"# Ruby Project Justfile
default:
    @echo "Available Ruby commands:"
    @just --list

# Check Ruby version
version:
    @echo "Ruby version:"
    @ruby --version

# Run tests
test:
    @echo "Running Ruby tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Format code with Prettier
prettier:
    @echo "Formatting code with Prettier..."
    @prettier --write .

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install

# Update dependencies
update:
    @echo "Updating Ruby dependencies..."
    @bundle update
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_python_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "django" => {
                r#"# Python Project Justfile
default:
    @echo "Available Django commands:"
    @just --list

# Start Django development server
django-server:
    @echo "Starting Django development server..."
    @python manage.py runserver

# Run Django tests
test:
    @echo "Running Django tests..."
    @python manage.py test

# Run migrations
django-migrate:
    @echo "Running Django migrations..."
    @python manage.py migrate

# Django shell
django-shell:
    @echo "Starting Django shell..."
    @python manage.py shell

# Create superuser
createsuperuser:
    @echo "Creating Django superuser..."
    @python manage.py createsuperuser

# Collect static files
collectstatic:
    @echo "Collecting static files..."
    @python manage.py collectstatic

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
            "flask" => {
                r#"# Python Project Justfile
default:
    @echo "Available Flask commands:"
    @just --list

# Start Flask development server
flask-run:
    @echo "Starting Flask development server..."
    @python app.py

# Flask shell
flask-shell:
    @echo "Starting Flask shell..."
    @python -i

# Run Flask tests
test:
    @echo "Running Flask tests..."
    @pytest

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
            _ => {
                r#"# Python Project Justfile
default:
    @echo "Available Python commands:"
    @just --list

# Check Python version
version:
    @echo "Python version:"
    @python --version

# Run tests
test:
    @echo "Running Python tests..."
    @pytest

# Format code
format:
    @echo "Formatting Python code..."
    @black .

# Lint code
lint:
    @echo "Linting Python code..."
    @flake8 .

# Type check
typecheck:
    @echo "Type checking Python code..."
    @mypy .

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_node_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "express" => {
                r#"# Node.js Project Justfile
default:
    @echo "Available Express.js commands:"
    @just --list

# Start development server
express-dev:
    @echo "Starting Express.js development server..."
    @npm run dev

# Start production server
express-server:
    @echo "Starting Express.js production server..."
    @npm start

# Run tests
test:
    @echo "Running Express.js tests..."
    @npm test

# Run linting
npm-lint:
    @echo "Running ESLint..."
    @npm run lint

# Format code
npm-format:
    @echo "Formatting code..."
    @npm run format

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Build project
build:
    @echo "Building Express.js project..."
    @npm run build
"#
            }
            "react" => {
                r#"# Node.js Project Justfile
default:
    @echo "Available React commands:"
    @just --list

# Start development server
react-dev:
    @echo "Starting React development server..."
    @npm start

# Build for production
react-build:
    @echo "Building React project for production..."
    @npm run build

# Run tests
test:
    @echo "Running React tests..."
    @npm test

# Run linting
lint:
    @echo "Running ESLint..."
    @npm run lint

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Eject (use with caution)
eject:
    @echo "Ejecting React app..."
    @npm run eject
"#
            }
            _ => {
                r#"# Node.js Project Justfile
default:
    @echo "Available Node.js commands:"
    @just --list

# Check Node.js version
version:
    @echo "Node.js version:"
    @node --version

# Start development server
dev:
    @echo "Starting Node.js development server..."
    @npm run dev

# Start production server
start:
    @echo "Starting Node.js production server..."
    @npm start

# Run tests
test:
    @echo "Running Node.js tests..."
    @npm test

# Run linting
lint:
    @echo "Running ESLint..."
    @npm run lint

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Build project
build:
    @echo "Building Node.js project..."
    @npm run build
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_go_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "web" => {
                r#"# Go Project Justfile
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
                r#"# Go Project Justfile
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
                r#"# Go Project Justfile
default:
    @echo "Available Go commands:"
    @just --list

# Check Go version
version:
    @echo "Go version:"
    @go version

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
        }
    }

    #[allow(dead_code)]
    pub fn get_rust_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "web" => {
                r#"# Rust Project Justfile
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
                r#"# Rust Project Justfile
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
                r#"# Rust Project Justfile
default:
    @echo "Available Rust commands:"
    @just --list

# Check Rust version
version:
    @echo "Rust version:"
    @rustc --version

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

# Check code
check:
    @echo "Checking Rust code..."
    @cargo check

# Install dependencies
install:
    @echo "Installing Rust dependencies..."
    @cargo build
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_bash_justfile_content(&self, template: &str) -> &'static str {
        match template {
            "devops" => {
                r#"# Bash Project Justfile
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
                r#"# Bash Project Justfile
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
                r#"# Bash Project Justfile
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
        }
    }

    // Additional test helper methods
    #[allow(dead_code)]
    pub fn get_pyproject_toml_content(&self, template: &str) -> &'static str {
        match template {
            "django" => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "django-project"
version = "0.1.0"
description = "A Django web application"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "django>=4.2",
    "djangorestframework>=3.14",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "pytest-django>=4.5",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true

[tool.django-stubs]
django_settings_module = "myproject.settings"
"#
            }
            "flask" => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "flask-project"
version = "0.1.0"
description = "A Flask web application"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "flask>=3.0",
    "flask-sqlalchemy>=3.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "pytest-flask>=1.2",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true

[tool.flask]
app_name = "app"
"#
            }
            _ => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "python-project"
version = "0.1.0"
description = "A Python project"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_eslint_config_content(&self, template: &str) -> &'static str {
        match template {
            "express" => {
                r#"module.exports = {
  env: {
    node: true,
    es2022: true,
  },
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  plugins: ['@typescript-eslint'],
  rules: {
    'indent': ['error', 2],
    'linebreak-style': ['error', 'unix'],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
    'no-unused-vars': 'warn',
    '@typescript-eslint/no-unused-vars': 'warn',
    'no-console': 'off',
  },
};
"#
            }
            "react" => {
                r#"module.exports = {
  env: {
    browser: true,
    es2022: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
    'plugin:react/recommended',
    'plugin:react-hooks/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: ['@typescript-eslint', 'react', 'react-hooks'],
  rules: {
    'indent': ['error', 2],
    'linebreak-style': ['error', 'unix'],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
    'react/react-in-jsx-scope': 'off',
    'react/prop-types': 'off',
  },
  settings: {
    react: {
      version: 'detect',
    },
  },
};
"#
            }
            _ => {
                r#"module.exports = {
  env: {
    node: true,
    es2022: true,
  },
  extends: [
    'eslint:recommended',
    '@typescript-eslint/recommended',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
  },
  plugins: ['@typescript-eslint'],
  rules: {
    'indent': ['error', 2],
    'linebreak-style': ['error', 'unix'],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
    'no-unused-vars': 'warn',
    '@typescript-eslint/no-unused-vars': 'warn',
  },
};
"#
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_node_package_json_content(&self, template: &str) -> &'static str {
        match template {
            "express" => {
                r#"{
  "name": "express-project",
  "version": "0.1.0",
  "description": "An Express.js web application",
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js",
    "dev": "nodemon src/index.js",
    "test": "jest",
    "lint": "eslint .",
    "build": "echo 'No build step required for Node.js'"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "helmet": "^7.0.0"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "jest": "^29.0.0",
    "nodemon": "^3.0.0",
    "typescript": "^5.0.0"
  }
}"#
            }
            "react" => {
                r#"{
  "name": "react-project",
  "version": "0.1.0",
  "description": "A React web application",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-scripts": "5.0.1"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "typescript": "^5.0.0"
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}"#
            }
            _ => {
                r#"{
  "name": "myproject",
  "version": "0.1.0",
  "description": "A Node.js project",
  "main": "src/index.js",
  "scripts": {
    "start": "node src/index.js",
    "dev": "nodemon src/index.js",
    "test": "jest",
    "lint": "eslint .",
    "build": "echo 'No build step required for Node.js'"
  },
  "dependencies": {},
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@typescript-eslint/eslint-plugin": "^6.0.0",
    "@typescript-eslint/parser": "^6.0.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "jest": "^29.0.0",
    "nodemon": "^3.0.0",
    "typescript": "^5.0.0"
  }
}"#
            }
        }
    }
}
