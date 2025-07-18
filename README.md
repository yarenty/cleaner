# Cleaner

Cleaner is a command-line tool for reclaiming disk space by recursively removing build, cache, and temporary directories from your projects. It supports a wide range of programming languages and IDEs, making it easy to keep your development environment tidy.

## Features
- **Multi-language support:** Rust, Python, Java, Node.js, Go, C#, C++, PHP, Ruby, and common IDEs.
- **Recursive cleaning:** Cleans all matching directories under a specified root.
- **Glob/pattern matching:** Use wildcards in directory names (e.g., `build*`, `*.cache`).
- **Customizable:** Override default directories or target a specific project type.
- **Exclusions:** Skip directories with `--exclude` (supports patterns).
- **Dry run mode:** See what would be deleted with `--dry-run`.
- **Depth limit:** Restrict cleaning to a certain directory depth with `--max-depth`.
- **Config file:** Use a TOML config for custom rules.
- **Parallel deletion:** Fast cleaning for large projects.
- **Interactive mode:** Confirm each deletion with `--interactive`.
- **Logging:** Output to file with `--log-file`.
- **CI/CD mode:** Machine-readable JSON summary with `--ci`.
- **Platform-specific cleaning:** Handles `.DS_Store`, `Thumbs.db`, etc.
- **Summary report:** Shows number of directories and total space freed.
- **Safe and informative:** Logs every action and supports different verbosity levels.
- **Tested:** Comprehensive integration tests for all major features.

## Supported Project Types & Directories
| Kind    | Directories cleaned (default for `all`)                                                      |
|---------|----------------------------------------------------------------------------------------------|
| all     | `target`, `out`, `build`, `dist`, `node_modules`, `.idea`, `.vscode`, `.vs`, `coverage`, `.next`, `.nuxt`, `.angular`, `.svelte-kit`, `vendor`, `.DS_Store` (macOS), `Thumbs.db`, `desktop.ini` (Windows) |
| rust    | `target`, `out`, `build`                                                                     |
| python  | `__pycache__`, `.venv`, `venv`, `env`, `.mypy_cache`, `.pytest_cache`                        |
| java    | `build`, `out`, `target`, `bin`, `classes`, `generated-sources`, `generated-test-sources`    |
| node    | `node_modules`, `dist`, `build`, `.next`, `.nuxt`, `.angular`, `.svelte-kit`, `coverage`     |
| go      | `bin`, `pkg`, `out`                                                                          |
| csharp  | `bin`, `obj`, `out`                                                                          |
| cpp     | `build`, `out`, `bin`, `CMakeFiles`, `cmake-build-*`, `Makefile`, `*.o`, `*.obj`             |
| php     | `vendor`, `out`, `build`, `cache`                                                            |
| ruby    | `.bundle`, `vendor`, `log`, `tmp`, `coverage`                                                |
| ide     | `.idea`, `.vs`, `.vscode`, `.DS_Store`, `.history`, `.classpath`, `.project`, `.settings`, `xcuserdata`, `*.iml` |

## Installation

1. **Clone the repository:**

```sh
git clone https://github.com/yarenty/cleaner.git
cd cleaner
```

2. **Build with Cargo:**

```sh
cargo build --release
```

3. **(Optional) Install globally:**

```sh
cargo install --path .
```

## Usage

### Clean all known safe build directories (default)

```sh
cleaner /path/to/your/project
```

### Clean only Python-related directories

```sh
cleaner /path/to/your/project --kind python
```

### Clean custom directories

```sh
cleaner /path/to/your/project --dirs node_modules,dist,coverage
```

### Exclude certain directories (supports patterns)

```sh
cleaner /path/to/your/project --exclude .git,docs
```

### Use glob patterns for matching

```sh
cleaner /path/to/your/project --dirs 'build*,*.cache'
```

### Limit recursion depth

```sh
cleaner /path/to/your/project --max-depth 2
```

### Dry run (show what would be deleted)

```sh
cleaner /path/to/your/project --dry-run
```

### Interactive mode (confirm each deletion)

```sh
cleaner /path/to/your/project --interactive
```

### Use a config file (TOML)

```sh
cleaner /path/to/your/project --config cleaner.toml
```

**Example `cleaner.toml`:**

```toml
[kinds.rust]
dirs = ["target", "out", "build", "custom_rust_dir"]

[exclude]
patterns = [".git", "docs"]
```

### Log output to a file

```sh
cleaner /path/to/your/project --log-file cleaner.log
```

### CI/CD mode (no prompts, JSON summary)

```sh
cleaner /path/to/your/project --ci
```

### See all options

```sh
cleaner --help
```

## Safety
- The tool will **recursively delete** directories matching the specified names or patterns. Use with care!
- Always double-check the path and directory patterns before running on important data.
- Use `--dry-run` to preview what will be deleted.
- Interactive and exclusion options help prevent accidental data loss.

## Testing
- The project includes comprehensive integration tests for all major features.
- To run tests:
  ```sh
  cargo test
  ```

## Contribution
Contributions are welcome! To add support for more languages or features:
1. Fork the repository and create a new branch.
2. Add your changes (e.g., update the `ProjectKind` enum and `default_dirs_for_kind` in `src/args.rs` and `src/utils.rs`).
3. Submit a pull request with a clear description.

## License
MIT License. See [LICENSE](LICENSE) for details. 