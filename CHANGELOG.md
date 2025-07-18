# Changelog

All notable changes to this project will be documented in this file.

## 2025/07/18
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

## 2021 - initial version of project