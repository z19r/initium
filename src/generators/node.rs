use crate::config::PackageJson;
use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_node(&self) -> Result<(), InitiumError> {
        self.generate_node_with_template("default").await
    }

    pub async fn generate_node_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Node.js-specific configs
        self.generate_nvmrc().await?;
        self.generate_eslint_config(template).await?;
        self.generate_node_package_json(template).await?;

        // Overwrite the basic justfile with Node.js-specific one
        self.generate_node_justfile(template).await?;

        Ok(())
    }

    async fn generate_nvmrc(&self) -> Result<(), InitiumError> {
        let content = "20\n";
        self.emit_file(".nvmrc", content, false, false).await
    }

    async fn generate_eslint_config(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "express" => {
                r#"{
  "env": {
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {
    "no-console": "off"
  }
}
"#
            }
            "react" => {
                r#"{
  "env": {
    "browser": true,
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module",
    "ecmaFeatures": {
      "jsx": true
    }
  },
  "rules": {}
}
"#
            }
            _ => {
                r#"{
  "env": {
    "browser": true,
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {}
}
"#
            }
        };
        self.emit_file(".eslintrc.json", content, false, false)
            .await
    }

    async fn generate_node_package_json(&self, template: &str) -> Result<(), InitiumError> {
        let package_json = PackageJson::from_template(template);
        let content = package_json.to_string();
        self.emit_file("package.json", &content, false, false).await
    }

    async fn generate_node_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "express" => {
                r#"# Express.js project justfile
default:
    @echo "Available Express.js commands:"
    @just --list

# Start development server
dev:
    @echo "Starting Express.js development server..."
    @npm run dev

# Start production server
start:
    @echo "Starting Express.js production server..."
    @npm start

# Run tests
test:
    @echo "Running Express.js tests..."
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
    @echo "Building Express.js project..."
    @npm run build
"#
            }
            "react" => {
                r#"# React project justfile
default:
    @echo "Available React commands:"
    @just --list

# Start development server
dev:
    @echo "Starting React development server..."
    @npm start

# Build for production
build:
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
                r#"# Node.js project justfile
default:
    @echo "Available Node.js commands:"
    @just --list

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
        };
        self.emit_file("justfile", content, false, true).await
    }
}
