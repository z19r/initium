use crate::error::InitiumError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_python(&self) -> Result<(), InitiumError> {
        self.generate_python_with_template("default").await
    }

    pub async fn generate_python_with_template(&self, template: &str) -> Result<(), InitiumError> {
        // Generate basic configs first
        self.generate_basic_with_template(false, template).await?;

        // Generate Python-specific configs
        self.generate_python_version().await?;
        self.generate_pyproject_toml(template).await?;
        self.generate_flake8_config().await?;
        self.generate_requirements_dev().await?;

        // Overwrite the basic justfile with Python-specific one
        self.generate_python_justfile(template).await?;

        Ok(())
    }

    async fn generate_python_version(&self) -> Result<(), InitiumError> {
        let content = "3.12\n";
        self.emit_file(".python-version", content, false, false)
            .await
    }

    async fn generate_pyproject_toml(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
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
        };
        self.emit_file("pyproject.toml", content, false, false)
            .await
    }

    async fn generate_flake8_config(&self) -> Result<(), InitiumError> {
        let content = r#"[flake8]
max-line-length = 88
extend-ignore = E203, W503
exclude = .git,__pycache__,build,dist,.venv,venv
"#;
        self.emit_file(".flake8", content, false, false).await
    }

    async fn generate_requirements_dev(&self) -> Result<(), InitiumError> {
        let content = r#"# Development dependencies
pytest==7.4.4
black==23.12.1
flake8==6.1.0
mypy==1.8.0
pytest-cov==4.1.0
"#;
        self.emit_file("requirements-dev.txt", content, false, false)
            .await
    }

    async fn generate_python_justfile(&self, template: &str) -> Result<(), InitiumError> {
        let content = match template {
            "django" => {
                r#"# Django project justfile
default:
    @echo "Available Django commands:"
    @just --list

# Start Django development server
server:
    @echo "Starting Django development server..."
    @python manage.py runserver

# Run Django tests
test:
    @echo "Running Django tests..."
    @python manage.py test

# Run migrations
migrate:
    @echo "Running Django migrations..."
    @python manage.py migrate

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
                r#"# Flask project justfile
default:
    @echo "Available Flask commands:"
    @just --list

# Start Flask development server
server:
    @echo "Starting Flask development server..."
    @python app.py

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
                r#"# Python project justfile
default:
    @echo "Available Python commands:"
    @just --list

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
        };
        self.emit_file("justfile", content, false, true).await
    }
}
