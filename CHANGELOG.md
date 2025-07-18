# Changelog

## 0.3.0
- Added --dry-run to preview deletions without removing anything
- Added --exclude for skipping directories (supports patterns)
- Added glob/pattern matching for --dirs and --exclude
- Added --max-depth to limit recursion
- Added TOML config file support (--config)
- Added summary report (number of directories, total space freed)
- Parallel deletion for faster cleaning
- Added --interactive to confirm each deletion
- Added --log-file for logging output to a file
- Added --ci for CI/CD mode (JSON summary, no prompts)
- Platform-specific cleaning (.DS_Store, Thumbs.db, etc.)
- Comprehensive integration tests for all features
- Updated README and documentation

## 0.2.0 
- Add support for multiple project types: Rust, Python, Java, Node.js, Go, C#, C++, PHP, Ruby, and common IDEs.
- New CLI argument `--kind` to select project type for cleaning.
- If `--kind` is not specified, all known build/cache/temp directories are cleaned by default.
- Allow custom directory list with `--dirs`.
- Improved CLI help and argument descriptions.
- Refactored code to use `clap::ValueEnum::value_variants()` for maintainability.
- Added detailed logging with configurable verbosity.
- Added safety notes and improved documentation.
- Added comprehensive README.
- Added this changelog.

### Dependency Upgrades
- Upgraded to latest versions (as of June 2024):
    - `clap` 4.x
    - `env_logger` 0.11.x
    - `itertools` 0.14.x
    - `thiserror` 2.x
    - and all other dependencies to their latest compatible versions.

---

## 0.1.0 -2021 - initial version of project