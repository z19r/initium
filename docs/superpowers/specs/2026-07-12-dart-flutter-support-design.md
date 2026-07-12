# Dart & Flutter Support — Design

Date: 2026-07-12
Status: Approved

## Purpose

Add `initium dart` and `initium flutter` commands so the tool can bootstrap
config files for Dart and Flutter projects, following the exact pattern
already established by the Ruby/Python/Node/Go/Rust/Bash generators.

## Scope

Two new, separate commands (not one `dart` command with a `flutter` template),
because Dart and Flutter target different project shapes (pure Dart
package/CLI vs. a Flutter mobile/web app) even though they share tooling
(pubspec.yaml, analysis_options.yaml, `dart format`).

Out of scope: platform folder scaffolding (`android/`, `ios/`, `web/`, etc.),
anything `flutter create`/`dart create` already generates beyond config files,
and any FFI/native build tooling.

## Architecture

Mirrors the existing per-language generator pattern:

- `src/generators/dart.rs` and `src/generators/flutter.rs` — new files, same
  shape as `src/generators/go.rs` (a `generate_<lang>_with_template` entry
  point calling `generate_basic_with_template` first, then language-specific
  file emitters, then a justfile overwrite).
- `ProjectType::Dart` and `ProjectType::Flutter` added to the enum in
  `src/generators/mod.rs`, plus detection logic in `detect_project_type`.
- `Commands::Dart { template }` and `Commands::Flutter { template }` added to
  the `Commands` enum in `src/main.rs`.
- `handle_dart` and `handle_flutter` added to `CommandHandler` in
  `src/commands.rs` (copy-paste-adapt of `handle_go`), plus dispatch arms in
  `main.rs` and auto-detect arms in `handle_auto_generate`.
- `generate_dart_hooks` / `generate_flutter_hooks` added to
  `src/generators/hooks.rs`.
- `handle_list` in `commands.rs` updated with the new files/templates/commands.

## Auto-detection

Added to `ConfigGenerator::detect_project_type` in `mod.rs`, after the Go
check and before Rust:

```
if pubspec.yaml exists:
    read it as text (no YAML parser dependency — this is a yes/no signal,
    not structured consumption)
    if it contains "sdk: flutter" or a top-level "flutter:" section
        -> ProjectType::Flutter
    else
        -> ProjectType::Dart
```

Plain substring checks are sufficient and avoid adding a new dependency
(e.g. `serde_yaml`) for something that only needs to answer "is this a
Flutter project."

## `initium dart`

Templates: `default | cli | package`. Each runs
`generate_basic_with_template` first (`.editorconfig`, `.prettierrc`,
`.prettierignore` — consistent with every other language, even though
Prettier isn't Dart tooling), then:

### `pubspec.yaml`

```yaml
# default
name: my_package
description: A Dart project.
version: 0.1.0
publish_to: 'none'

environment:
  sdk: ^3.0.0

dev_dependencies:
  lints: ^4.0.0
  test: ^1.24.0
```

- `cli` template adds an `executables:` block.
- `package` template drops `publish_to: 'none'` (packages are meant to be
  published) and adds `homepage:`/`repository:` placeholder comments.

### `analysis_options.yaml`

```yaml
include: package:lints/recommended.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
```

Same content across all three templates.

### `.gitignore`

Real-world convention: applications commit `pubspec.lock`, libraries don't.

```
.dart_tool/
.packages
build/
*.iml
.idea/
.vscode/
```

- `default`/`cli` templates: `pubspec.lock` is NOT in the ignore list
  (it's an app, commit the lockfile).
- `package` template: `pubspec.lock` IS added to the ignore list.

### `justfile`

`default`:

```
default:
    @just --list
run:
    @dart run
test:
    @dart test
fmt:
    @dart format .
fmt-check:
    @dart format --output=none --set-exit-if-changed .
analyze:
    @dart analyze
install:
    @dart pub get
update:
    @dart pub upgrade
```

- `cli`: replaces `run` with `run *ARGS: dart run bin/main.dart {{ARGS}}`,
  adds `build: dart compile exe bin/main.dart -o bin/app`.
- `package`: drops `run`, adds `test-coverage` (`dart test
  --coverage=coverage` + `dart pub global run coverage:format_coverage`) and
  `publish-check: dart pub publish --dry-run`.

## `initium flutter`

Templates: `default | package | plugin`. Same basic-first flow, then:

### `pubspec.yaml`

```yaml
# default (app)
name: my_app
description: A new Flutter project.
version: 0.1.0+1
publish_to: 'none'

environment:
  sdk: ^3.0.0

dependencies:
  flutter:
    sdk: flutter
  cupertino_icons: ^1.0.6

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^4.0.0

flutter:
  uses-material-design: true
```

- `package` template: drops `cupertino_icons`, the `+1` build number, and
  `uses-material-design`.
- `plugin` template: everything `package` has, plus `plugin_platform_interface`
  dependency and a `flutter.plugin.platforms` stub with android/ios
  `pluginClass` entries.

### `analysis_options.yaml`

```yaml
include: package:flutter_lints/flutter.yaml

linter:
  rules:
    - prefer_single_quotes
    - always_declare_return_types
    - avoid_print

analyzer:
  exclude:
    - build/**
    - .dart_tool/**
    - "**/*.g.dart"
    - "**/*.freezed.dart"
```

Same content across all three templates.

### `.gitignore`

Stock `flutter create` app ignore list: `.dart_tool/`, `.flutter-plugins`,
`.flutter-plugins-dependencies`, `/build/`, plus standard IDE/OS cruft
(`.idea/`, `*.iml`, `.DS_Store`, etc.).

- `default` template: as above.
- `package`/`plugin` templates: additionally add `pubspec.lock` to the
  ignore list (library convention, same reasoning as Dart above).

### `justfile`

`default`:

```
default:
    @just --list
run:
    @flutter run
test:
    @flutter test
fmt:
    @dart format .
analyze:
    @flutter analyze
build-apk:
    @flutter build apk
build-ios:
    @flutter build ios
install:
    @flutter pub get
clean:
    @flutter clean
```

- `package`: drops `run`/`build-apk`/`build-ios`, adds `publish-check:
  flutter pub publish --dry-run`.
- `plugin`: everything `package` has, plus `run-example: cd example &&
  flutter run` and `test-example: cd example && flutter test`.

## Git hooks

Added to `hooks.rs`, following the existing `generate_go_hooks` shape
(pre-commit, pre-push, commit-msg via the shared `get_commit_msg_hook`):

- **pre-commit**: `dart format --output=none --set-exit-if-changed .` then
  `dart analyze` (Dart) or `flutter analyze` (Flutter).
- **pre-push**: `dart test` (Dart) or `flutter test` (Flutter).
- **commit-msg**: reuses the existing shared hook, no changes needed.

Both hook generators take `template: &str` and `force: bool`, matching
`generate_go_hooks`'s signature. Hook script content does not vary by
template — the same commands apply to `default`/`cli`/`package` and
`default`/`package`/`plugin` respectively.

## Testing

Following the existing per-language coverage in `tests/`:

- `unit_tests.rs` — auto-detection cases: pubspec.yaml with no flutter
  section → Dart; pubspec.yaml with `sdk: flutter` dependency → Flutter;
  pubspec.yaml with a `flutter:` top-level section → Flutter.
- `generators_tests.rs` — content assertions for all `dart` and `flutter`
  templates (pubspec.yaml, analysis_options.yaml, .gitignore, justfile),
  mirroring `test_all_go_templates`/`test_go_justfile_templates`. Explicit
  assertions that `pubspec.lock` is absent from Dart/Flutter app `.gitignore`
  and present in package/plugin `.gitignore`.
- `integration_tests.rs` — full CLI flow via `assert_cmd` for both commands
  and all templates, plus `--hooks` flag coverage.
- `cli_tests.rs` — argument parsing for `Commands::Dart`/`Commands::Flutter`.
- `fail_on_exists_tests.rs` — `--fail-on-exists` behavior for both commands.
- `e2e_tests.rs` — end-to-end scenarios (generate then verify file tree),
  mirroring existing Go/Rust e2e cases.

Target: 80%+ coverage on the new code, per project testing standards.

## Non-goals / deferred

- No YAML-parsing dependency — substring detection is sufficient for the
  auto-detect signal.
- No platform-folder generation (`android/`, `ios/`, `web/`, `example/` for
  plugins) — those are `flutter create`'s job, and initium only bootstraps
  config files, consistent with how it never generates `Cargo.toml` contents
  beyond what Rust's generator already skips.
