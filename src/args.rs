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
#[clap(about = "Cleaning build directories.", long_about = None)]
pub struct Args {
    /// starting path to be cleaned
    #[clap(value_parser)]
    pub path: String,

    /// Directories to be cleaned - separated by ','
    #[clap(short, long, default_value = "target,out,build")]
    pub dirs: String,

    /// Project type/kind: ide, rust, python, java, node, go, csharp, cpp, php, ruby
    #[clap(long, value_enum, default_value = "rust")]
    pub kind: ProjectKind,

    ///Set custom log level: info, debug, trace
    #[clap(short, long, default_value = "info")]
    pub log: String,
}
