//! Argument parsing definitions for the Cleaner CLI tool.
//!
//! This module defines the command-line arguments, project kind enum, and their documentation for the Cleaner project.
//! It uses the `clap` crate for robust and user-friendly argument parsing.
//!
//! - `ProjectKind` enumerates all supported project types/languages/IDEs.
//! - `Args` struct defines all CLI arguments, their help text, and parsing rules.

use clap::{Parser, ValueEnum};
use std::fmt;

/// Supported project types/languages/IDEs for cleaning.
///
/// Each variant corresponds to a set of build/cache/temp directories that can be cleaned.
#[derive(ValueEnum, Debug, Clone)]
pub enum ProjectKind {
    /// Universally safe build and IDE directories (default)
    All,
    /// JetBrains, VSCode, Visual Studio, Xcode, and other IDE leftovers
    Ide,
    /// Rust projects (Cargo, etc.)
    Rust,
    /// Python projects (venv, __pycache__, etc.)
    Python,
    /// Java projects (Maven, Gradle, etc.)
    Java,
    /// Node.js/JavaScript/TypeScript projects (node_modules, dist, etc.)
    Node,
    /// Go projects (bin, pkg, etc.)
    Go,
    /// C#/.NET projects (bin, obj, etc.)
    CSharp,
    /// C/C++ projects (build, CMakeFiles, etc.)
    Cpp,
    /// PHP projects (vendor, cache, etc.)
    Php,
    /// Ruby projects (.bundle, tmp, etc.)
    Ruby,
}

impl fmt::Display for ProjectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ProjectKind::All => "all",
            ProjectKind::Ide => "ide",
            ProjectKind::Rust => "rust",
            ProjectKind::Python => "python",
            ProjectKind::Java => "java",
            ProjectKind::Node => "node",
            ProjectKind::Go => "go",
            ProjectKind::CSharp => "csharp",
            ProjectKind::Cpp => "cpp",
            ProjectKind::Php => "php",
            ProjectKind::Ruby => "ruby",
        };
        write!(f, "{}", s)
    }
}

/// Command-line arguments for the Cleaner CLI tool.
///
/// Uses `clap` for parsing and help generation.
#[derive(Parser, Debug)]
#[clap(version)]
#[clap(
    about = "Cleaning build directories to save disk space. Supports multiple languages and IDEs.",
    long_about = "Recursively finds and deletes build, cache, and temporary directories for supported project types (Rust, Python, Java, Node, Go, C#, C++, PHP, Ruby, and common IDEs). Use --kind to target a specific project type, or clean all by default. Use --dirs to override the directory list."
)]
pub struct Args {
    /// The root directory to start cleaning from. All subdirectories will be searched recursively.
    /// Example: /home/user/projects or .
    #[clap(value_parser)]
    pub path: String,

    /// Comma-separated list of directory names to clean. If not provided, uses defaults for the selected kind(s).
    /// Example: --dirs target,out,build,node_modules
    #[clap(short, long)]
    pub dirs: Option<String>,

    /// Comma-separated list of directory names or patterns to exclude from cleaning.
    /// Example: --exclude .git,docs
    #[clap(short, long)]
    pub exclude: Option<String>,

    /// Project type/kind to target for cleaning. Supported values: all (default), ide, rust, python, java, node, go, csharp, cpp, php, ruby.
    /// If not specified, only universally safe build and IDE directories will be cleaned.
    /// Example: --kind python
    #[clap(short, long, value_enum, default_value = "all")]
    pub kind: Option<ProjectKind>,

    /// Skip confirmation prompt and force deletion of directories.
    /// Example: --force
    #[clap(short, long, action)]
    pub force: bool,

    /// Show what would be deleted, but do not actually delete anything.
    /// Example: --dry-run
    #[clap(short = 'n', long, action)]
    pub dry_run: bool,

    /// Set custom log level for output verbosity. Supported: info, debug, trace.
    /// Example: --log debug
    #[clap(short, long, default_value = "info")]
    pub log: String,

    /// Maximum recursion depth for directory search. 0 means unlimited.
    /// Example: --max-depth 2
    #[clap(long, default_value = "0")]
    pub max_depth: usize,

    /// Path to a custom config file (TOML) for directory and exclusion settings.
    /// Example: --config cleaner.toml
    #[clap(long)]
    pub config: Option<String>,
}
