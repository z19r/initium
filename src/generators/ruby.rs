use crate::config::PackageJson;
use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_ruby(&self) -> Result<(), InitiumError> {
        self.generate_ruby_with_template("default").await
    }

    pub async fn generate_ruby_with_template(&self, template: &str) -> Result<(), InitiumError> {
        let basic_template = if template == "default" {
            "ruby"
        } else {
            template
        };

        // Generate basic configs first (includes justfile)
        self.generate_basic_with_template(false, basic_template)
            .await?;

        // Generate Ruby-specific configs
        self.generate_ruby_version().await?;
        self.generate_node_version().await?;
        self.generate_rubocop_config_with_template(template).await?;
        self.generate_package_json_with_template(template).await?;

        // Overwrite the basic justfile with Ruby-specific one
        self.generate_ruby_justfile(template).await?;

        Ok(())
    }

    async fn generate_ruby_version(&self) -> Result<(), InitiumError> {
        let content = "3.4.9\n";
        self.emit_file(".ruby-version", content, false, false).await
    }

    async fn generate_node_version(&self) -> Result<(), InitiumError> {
        let content = "25.9.0\n";
        self.emit_file(".node-version", content, false, false).await
    }

    async fn generate_rubocop_config_with_template(
        &self,
        template: &str,
    ) -> Result<(), InitiumError> {
        let content = match template {
            "rails" => {
                r#"# Rails-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'db/schema.rb'
    - 'bin/**/*'
    - 'config/**/*'
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

# Prettier (@prettier/plugin-ruby) is the formatter source of truth.
# Disable RuboCop cops that fight it — never bend Prettier to satisfy RuboCop.
Style/StringLiterals:
  Enabled: false
Style/StringLiteralsInInterpolation:
  Enabled: false
Style/TrailingCommaInArguments:
  Enabled: false
Style/TrailingCommaInArrayLiteral:
  Enabled: false
Style/TrailingCommaInHashLiteral:
  Enabled: false
Layout/SpaceInsideArrayLiteralBrackets:
  Enabled: false

Layout/LineLength:
  Max: 120

Metrics/BlockLength:
  Exclude:
    - 'spec/**/*'
    - 'test/**/*'
"#
            }
            "sinatra" => {
                r#"# Sinatra-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

# Prettier (@prettier/plugin-ruby) is the formatter source of truth.
# Disable RuboCop cops that fight it — never bend Prettier to satisfy RuboCop.
Style/StringLiterals:
  Enabled: false
Style/StringLiteralsInInterpolation:
  Enabled: false
Style/TrailingCommaInArguments:
  Enabled: false
Style/TrailingCommaInArrayLiteral:
  Enabled: false
Style/TrailingCommaInHashLiteral:
  Enabled: false
Layout/SpaceInsideArrayLiteralBrackets:
  Enabled: false

Layout/LineLength:
  Max: 120
"#
            }
            "gem" => {
                r#"# Gem-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

# Prettier (@prettier/plugin-ruby) is the formatter source of truth.
# Disable RuboCop cops that fight it — never bend Prettier to satisfy RuboCop.
Style/StringLiterals:
  Enabled: false
Style/StringLiteralsInInterpolation:
  Enabled: false
Style/TrailingCommaInArguments:
  Enabled: false
Style/TrailingCommaInArrayLiteral:
  Enabled: false
Style/TrailingCommaInHashLiteral:
  Enabled: false
Layout/SpaceInsideArrayLiteralBrackets:
  Enabled: false

Layout/LineLength:
  Max: 120
"#
            }
            _ => {
                r#"# Default RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3

Style/Documentation:
  Enabled: false

# Prettier (@prettier/plugin-ruby) is the formatter source of truth.
# Disable RuboCop cops that fight it — never bend Prettier to satisfy RuboCop.
Style/StringLiterals:
  Enabled: false
Style/StringLiteralsInInterpolation:
  Enabled: false
Style/TrailingCommaInArguments:
  Enabled: false
Style/TrailingCommaInArrayLiteral:
  Enabled: false
Style/TrailingCommaInHashLiteral:
  Enabled: false
Layout/SpaceInsideArrayLiteralBrackets:
  Enabled: false

Layout/LineLength:
  Max: 120
"#
            }
        };
        self.emit_file(".rubocop.yml", content, false, false).await
    }

    async fn generate_package_json_with_template(
        &self,
        template: &str,
    ) -> Result<(), InitiumError> {
        let package_json = match template {
            "rails" | "sinatra" | "gem" => PackageJson::from_template(template),
            _ => PackageJson::default(),
        };
        let content = package_json.to_string();
        self.emit_file("package.json", &content, false, false).await
    }

    async fn generate_ruby_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "rails" => {
                r#"# Rails project justfile
default:
    @echo "Available Rails commands:"
    @just --list

# Start Rails server
server:
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

# Format with Prettier (source of truth for style)
fmt:
    @echo "Formatting with Prettier..."
    @npx prettier --write .

# Check formatting without writing
fmt-check:
    @echo "Checking formatting with Prettier..."
    @npx prettier --check .

# Database operations
db:migrate:
    @echo "Running database migrations..."
    @bundle exec rails db:migrate

db:seed:
    @echo "Seeding database..."
    @bundle exec rails db:seed

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            "sinatra" => {
                r#"# Sinatra project justfile
default:
    @echo "Available Sinatra commands:"
    @just --list

# Start Sinatra server
server:
    @echo "Starting Sinatra server..."
    @bundle exec ruby app.rb

# Run tests
test:
    @echo "Running Sinatra tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Format with Prettier (source of truth for style)
fmt:
    @echo "Formatting with Prettier..."
    @npx prettier --write .

# Check formatting without writing
fmt-check:
    @echo "Checking formatting with Prettier..."
    @npx prettier --check .

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            "gem" => {
                r#"# Ruby gem project justfile
default:
    @echo "Available gem commands:"
    @just --list

# Build gem
build:
    @echo "Building gem..."
    @gem build *.gemspec

# Install gem locally
install:
    @echo "Installing gem locally..."
    @gem install *.gem

# Run tests
test:
    @echo "Running gem tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Format with Prettier (source of truth for style)
fmt:
    @echo "Formatting with Prettier..."
    @npx prettier --write .

# Check formatting without writing
fmt-check:
    @echo "Checking formatting with Prettier..."
    @npx prettier --check .

# Install dependencies
install-deps:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
            _ => {
                r#"# Ruby project justfile
default:
    @echo "Available Ruby commands:"
    @just --list

# Run tests
test:
    @echo "Running Ruby tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Format with Prettier (source of truth for style)
fmt:
    @echo "Formatting with Prettier..."
    @npx prettier --write .

# Check formatting without writing
fmt-check:
    @echo "Checking formatting with Prettier..."
    @npx prettier --check .

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#
            }
        };
        self.emit_file("justfile", content, false, true).await
    }
}
