# Cleaner

Cleaner is a command-line tool for reclaiming disk space by recursively removing build, cache, and temporary directories from your projects. It supports a wide range of programming languages and IDEs, making it easy to keep your development environment tidy.

## Features
- **Multi-language support:** Rust, Python, Java, Node.js, Go, C#, C++, PHP, Ruby, and common IDEs.
- **Recursive cleaning:** Cleans all matching directories under a specified root.
- **Customizable:** Override default directories or target a specific project type.
- **Safe and informative:** Logs every action and supports different verbosity levels.

## Supported Project Types & Directories
| Kind    | Directories cleaned                                                                 |
|---------|-------------------------------------------------------------------------------------|
| rust    | `target`, `out`, `build`                                                           |
| python  | `__pycache__`, `.venv`, `venv`, `env`, `.mypy_cache`, `.pytest_cache`               |
| java    | `build`, `out`, `target`, `bin`, `classes`, `generated-sources`, `generated-test-sources` |
| node    | `node_modules`, `dist`, `build`, `.next`, `.nuxt`, `.angular`, `.svelte-kit`, `coverage` |
| go      | `bin`, `pkg`, `out`                                                                 |
| csharp  | `bin`, `obj`, `out`                                                                 |
| cpp     | `build`, `out`, `bin`, `CMakeFiles`, `cmake-build-*`, `Makefile`, `*.o`, `*.obj`    |
| php     | `vendor`, `out`, `build`, `cache`                                                   |
| ruby    | `.bundle`, `vendor`, `log`, `tmp`, `coverage`                                       |
| ide     | `.idea`, `.vs`, `.vscode`, `.DS_Store`, `.history`, `.classpath`, `.project`, `.settings`, `xcuserdata`, `*.iml` |

## Installation

1. **Clone the repository:**

```sh
git clone <repo-url>
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

### Clean all known build directories (default)

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

### Set log verbosity

```sh
cleaner /path/to/your/project --log debug
```

### See all options

```sh
cleaner --help
```

## Safety
- The tool will **recursively delete** directories matching the specified names. Use with care!
- Always double-check the path and directory patterns before running on important data.

## Contribution
Contributions are welcome! To add support for more languages or features:
1. Fork the repository and create a new branch.
2. Add your changes (e.g., update the `ProjectKind` enum and `default_dirs_for_kind` in `src/args.rs` and `src/utils.rs`).
3. Submit a pull request with a clear description.

## License
MIT License. See [LICENSE](LICENSE) for details. 