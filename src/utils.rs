//! Utility functions for the Cleaner project.
//!
//! This module provides:
//! - Logger setup with custom formatting for thread and log level information.
//! - Logic to determine which build/cache/temp directories should be cleaned for each supported project kind.
//! - Helper functions used throughout the project.
//!
//! Supported project kinds include Rust, Python, Java, Node.js, Go, C#, C++, PHP, Ruby, and common IDEs.
//!
//! The logger setup function allows for colored, timestamped, and optionally thread-aware log output.
//! The directory selection logic is used by the main cleaning routine to determine which directories to remove.

use crate::args::ProjectKind;
use chrono::prelude::*;
use env_logger::fmt::Formatter;
use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter, Record};
use std::io::Write;
use std::thread;

/// Sets up the logger with custom formatting.
///
/// # Arguments
/// * `log_thread` - If true, includes the thread name in log output.
/// * `rust_log` - Optional log level filter string (e.g., "info", "debug").
///
/// The logger outputs colored, timestamped log messages with optional thread info.
pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    // Custom output format closure for env_logger
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        // Optionally include thread name
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        // Format log level as a string
        let level = match record.level() {
            Level::Error => "[ERROR]",
            Level::Warn => "[WARN]",
            Level::Info => "[INFO]",
            Level::Debug => "[DEBUG]",
            Level::Trace => "[TRACE]",
        };

        // Get current local time for timestamp
        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        // Write formatted log message
        writeln!(
            formatter,
            "{} {}{} - {} - {}",
            time_str,
            thread_name,
            level,
            record.target(),
            record.args()
        )
    };

    // Build and initialize the logger
    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);
    builder.write_style(WriteStyle::Always);

    // Optionally parse log level filter
    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}

/// Returns the default list of build/cache/temp directories for a given project kind.
///
/// # Arguments
/// * `kind` - The project kind (Rust, Python, Java, etc.)
///
/// # Returns
/// A vector of directory names (as &str) that should be cleaned for the given kind.
pub fn default_dirs_for_kind(kind: &ProjectKind) -> Vec<&'static str> {
    match kind {
        ProjectKind::Rust => vec!["target", "out", "build"],
        ProjectKind::Python => vec![
            "__pycache__",
            ".venv",
            "venv",
            "env",
            ".mypy_cache",
            ".pytest_cache",
        ],
        ProjectKind::Java => vec![
            "build",
            "out",
            "target",
            "bin",
            "classes",
            "generated-sources",
            "generated-test-sources",
        ],
        ProjectKind::Ide => vec![
            ".idea",
            ".vs",
            ".vscode",
            ".DS_Store",
            ".history",
            ".classpath",
            ".project",
            ".settings",
            "xcuserdata",
            "*.iml",
        ],
        ProjectKind::Node => vec![
            "node_modules",
            "dist",
            "build",
            ".next",
            ".nuxt",
            ".angular",
            ".svelte-kit",
            "coverage",
        ],
        ProjectKind::Go => vec!["bin", "pkg", "out"],
        ProjectKind::CSharp => vec!["bin", "obj", "out"],
        ProjectKind::Cpp => vec![
            "build",
            "out",
            "bin",
            "CMakeFiles",
            "cmake-build-*",
            "Makefile",
            "*.o",
            "*.obj",
        ],
        ProjectKind::Php => vec!["vendor", "out", "build", "cache"],
        ProjectKind::Ruby => vec![".bundle", "vendor", "log", "tmp", "coverage"],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::ProjectKind;

    #[test]
    fn test_default_dirs_for_rust() {
        let dirs = default_dirs_for_kind(&ProjectKind::Rust);
        assert_eq!(dirs, vec!["target", "out", "build"]);
    }

    #[test]
    fn test_default_dirs_for_python() {
        let dirs = default_dirs_for_kind(&ProjectKind::Python);
        assert!(dirs.contains(&"__pycache__"));
        assert!(dirs.contains(&".venv"));
        assert!(dirs.contains(&"venv"));
    }

    #[test]
    fn test_default_dirs_for_java() {
        let dirs = default_dirs_for_kind(&ProjectKind::Java);
        assert!(dirs.contains(&"build"));
        assert!(dirs.contains(&"target"));
    }

    #[test]
    fn test_default_dirs_for_ide() {
        let dirs = default_dirs_for_kind(&ProjectKind::Ide);
        assert!(dirs.contains(&".idea"));
        assert!(dirs.contains(&".vscode"));
    }

    #[test]
    fn test_default_dirs_for_node() {
        let dirs = default_dirs_for_kind(&ProjectKind::Node);
        assert!(dirs.contains(&"node_modules"));
        assert!(dirs.contains(&"dist"));
    }

    #[test]
    fn test_default_dirs_for_go() {
        let dirs = default_dirs_for_kind(&ProjectKind::Go);
        assert!(dirs.contains(&"bin"));
        assert!(dirs.contains(&"pkg"));
    }

    #[test]
    fn test_default_dirs_for_csharp() {
        let dirs = default_dirs_for_kind(&ProjectKind::CSharp);
        assert!(dirs.contains(&"bin"));
        assert!(dirs.contains(&"obj"));
    }

    #[test]
    fn test_default_dirs_for_cpp() {
        let dirs = default_dirs_for_kind(&ProjectKind::Cpp);
        assert!(dirs.contains(&"build"));
        assert!(dirs.contains(&"CMakeFiles"));
    }

    #[test]
    fn test_default_dirs_for_php() {
        let dirs = default_dirs_for_kind(&ProjectKind::Php);
        assert!(dirs.contains(&"vendor"));
        assert!(dirs.contains(&"cache"));
    }

    #[test]
    fn test_default_dirs_for_ruby() {
        let dirs = default_dirs_for_kind(&ProjectKind::Ruby);
        assert!(dirs.contains(&".bundle"));
        assert!(dirs.contains(&"tmp"));
    }
}
