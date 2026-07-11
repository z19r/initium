# Development Setup Guide

This guide will help you set up the development environment for Initium and understand the CI/CD pipeline.

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git
- GitHub account (for CI/CD)

### Local Development Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/zackkitzmiller/initium.git
   cd initium
   ```

2. **Install development tools**

   ```bash
   # Install cargo-tarpaulin for coverage
   cargo install cargo-tarpaulin --version 0.32.8

   # Install cargo-edit for version management
   cargo install cargo-edit
   ```

3. **Run tests and coverage**

   ```bash
   # Run all tests
   just test

   # Generate coverage report
   just test-coverage
   ```

## 🧪 Testing

### Test Structure

- **Unit Tests**: `tests/unit_tests.rs` - Test individual functions and methods
- **Integration Tests**: `tests/integration_tests.rs` - Test CLI functionality and file generation
- **Coverage**: Uses `cargo-tarpaulin` for comprehensive coverage reporting

### Running Tests

```bash
# All tests
just test

# Specific test file
cargo test --test integration_tests

# With coverage
just test-coverage

# Using justfile (preferred)
just test
just test-coverage
```

## 🔄 CI/CD Pipeline

### GitHub Actions Workflows

#### 1. **CI/CD Pipeline** (`.github/workflows/ci.yml`)

- **Triggers**: Push to main/develop, PRs, releases
- **Jobs**:
  - **Test**: Multi-platform testing (Ubuntu, macOS, Windows)
  - **Coverage**: Coverage reporting and Codecov integration
  - **Release**: Automated release asset creation
  - **Publish**: Automated crates.io publishing

#### 2. **Release Workflow** (`.github/workflows/release.yml`)

- **Manual trigger** with version input
- **Automated**:
  - Version bumping
  - Changelog generation
  - Git tagging
  - GitHub release creation

### Coverage Integration

#### Codecov Setup

1. **Sign up** at [codecov.io](https://codecov.io)
2. **Connect** your GitHub repository
3. **Add badge** to README (already done)
4. **Configure** coverage thresholds in `.codecov.yml`

#### Coverage Targets

- **Project**: 80% minimum
- **Patch**: 80% minimum
- **Threshold**: 5% decrease allowed

## 📦 Release Process

### Automated Release

1. **Go to** GitHub Actions → Release workflow
2. **Click** "Run workflow"
3. **Choose** release type (patch/minor/major)
4. **Enter** specific version (optional)
5. **Click** "Run workflow"

### Manual Release

```bash
# Using justfile
just release-patch    # 0.1.0 → 0.1.1
just release-minor    # 0.1.0 → 0.2.0
just release-major    # 0.1.0 → 1.0.0

# Or manually
cargo set-version --bump patch
git add Cargo.toml Cargo.lock
git commit -m "Bump version for patch release"
git tag -a "v$(cargo get version)" -m "Release v$(cargo get version)"
git push origin main
git push origin "v$(cargo get version)"
```

## 🔧 Development Commands

### Using Justfile

```bash
# Development
just cargo-build          # Build debug
just cargo-build-release  # Build release
just test                 # Run tests
just check                # Check compilation
just fmt                  # Format code
just fmt-check            # Check formatting
just clippy               # Run linter
just clean                # Clean build artifacts

# CI/CD
just ci-local             # Run local CI checks

# Release
just release-patch        # Patch release
just release-minor        # Minor release
just release-major        # Major release

# Info
just info                 # Project information
```

### Local CI Checks

```bash
# Run all CI checks locally
just ci-local

# This runs:
# - cargo fmt --all -- --check
# - cargo clippy --all-targets --all-features -- -D warnings
# - cargo test --all-features
# - cargo tarpaulin --out Xml --output-dir coverage
```

## 📊 Coverage Reporting

### Local Coverage

```bash
# Generate HTML report
cargo tarpaulin --out Html

# Generate XML for CI
cargo tarpaulin --out Xml --output-dir coverage

# View coverage
open target/tarpaulin/index.html
```

### Coverage Targets

- **Current**: 61.41%
- **Target**: 80%
- **Goal**: 90%+

### Improving Coverage

1. **Add tests** for untested code paths
2. **Test error conditions** and edge cases
3. **Mock external dependencies** where appropriate
4. **Test CLI interactions** and user flows

## 🚨 Troubleshooting

### Common Issues

#### CI Failures

- **Formatting**: Run `just fmt` locally
- **Clippy**: Fix warnings with `just clippy`
- **Tests**: Run `just test` locally
- **Coverage**: Ensure `cargo-tarpaulin` is installed (`just install-tools`)

#### Release Issues

- **Version conflicts**: Check `Cargo.toml` and `Cargo.lock`
- **Git issues**: Ensure proper authentication and permissions
- **Tag conflicts**: Delete conflicting tags locally and remotely

#### Coverage Issues

- **Low coverage**: Add tests for untested functions
- **Build failures**: Check Rust toolchain version
- **Report generation**: Run `just install-tools` for cargo-tarpaulin

### Getting Help

1. **Check** GitHub Actions logs
2. **Review** this setup guide
3. **Search** existing issues
4. **Create** new issue with detailed information

## 🔗 Useful Links

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Codecov](https://docs.codecov.io/)
- [Just](https://github.com/casey/just)
- [Cargo Tarpaulin](https://github.com/xd009642/tarpaulin)
