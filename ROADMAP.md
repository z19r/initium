# Initium Roadmap

## v1.0 – Core (Complete)

- Basic and Ruby project configuration generation
- Auto-detection of project types
- Interactive mode with guided setup
- Template system for different coding styles
- Dry run mode for previewing changes
- Justfile generation for project automation

## v1.1 – Language Expansion (Complete)

- **Python** – `.python-version`, `pyproject.toml`, `.flake8`, `requirements-dev.txt`
- **Node.js** – `.nvmrc`, `.eslintrc.js`, `.prettierrc`, `package.json`
- **Go** – `.golangci.yml`, `go.mod`, `.gitignore` additions
- **Rust** – `rustfmt.toml`, `.clippy.toml`, `.cargo/config.toml`

## v1.2 – Next Up

### Git Hooks

```bash
initium ruby --hooks
```

- Pre-commit hooks for RuboCop, Prettier
- Husky configuration for Node projects
- Git hooks for all supported languages

### CI/CD Templates

```bash
initium ruby --ci github
initium ruby --ci gitlab
```

- `.github/workflows/ci.yml`
- `.gitlab-ci.yml`
- Language-specific CI configurations

### Docker Support

```bash
initium ruby --docker
```

- `Dockerfile` template
- `docker-compose.yml`
- `.dockerignore`

### IDE Configuration

```bash
initium ruby --ide vscode
initium ruby --ide intellij
```

- VS Code settings and extensions
- IntelliJ IDEA configuration

## v1.3 – Polish

### Enhanced Dry Run

- Show file contents that would be generated
- Diff against existing files
- Interactive confirmation per file

### Backup Existing Files

```bash
initium ruby --backup
```

- Rename existing configs to `.backup` before overwriting
- Timestamped backups
- Restore functionality

### Configuration Validation

```bash
initium validate
```

- Check if existing configs are valid
- Suggest improvements
- Validate against language standards

### Update Mode

```bash
initium update
```

- Update existing configs to latest versions
- Merge with user customizations
- Preserve user-specific settings

## Future (Maybe)

- **Plugin system** – Community templates, custom generators (significant scope)
- **Web UI** – Different product; out of scope for core CLI
- **Smart detection** – Heuristic-based framework/version detection (no ML)

---

*This roadmap evolves based on feedback and priorities.*
