use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum ProjectKind {
    Ide,
    Rust,
    Python,
    Java,
    Node,
    Go,
    CSharp,
    Cpp,
    Php,
    Ruby,
}

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(about = "Cleaning build directories to save disk space. Supports multiple languages and IDEs.", long_about = "Recursively finds and deletes build, cache, and temporary directories for supported project types (Rust, Python, Java, Node, Go, C#, C++, PHP, Ruby, and common IDEs). Use --kind to target a specific project type, or clean all by default. Use --dirs to override the directory list.")]
pub struct Args {
    /// The root directory to start cleaning from. All subdirectories will be searched recursively.
    /// Example: /home/user/projects or .
    #[clap(value_parser)]
    pub path: String,

    /// Comma-separated list of directory names to clean. Overrides the default for the selected kind.
    /// Example: --dirs target,out,build,node_modules
    #[clap(short, long, default_value = "target,out,build")]
    pub dirs: String,

    /// Project type/kind to target for cleaning. Supported values: ide, rust, python, java, node, go, csharp, cpp, php, ruby.
    /// If not specified, all known build/cache/temp directories for all supported kinds will be cleaned.
    /// Example: --kind python
    #[clap(long, value_enum)]
    pub kind: Option<ProjectKind>,

    /// Set custom log level for output verbosity. Supported: info, debug, trace.
    /// Example: --log debug
    #[clap(short, long, default_value = "info")]
    pub log: String,
}
