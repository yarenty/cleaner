//! Main entry point for the Cleaner project.
//!
//! This binary parses command-line arguments, sets up logging, determines which build/cache/temp directories to clean,
//! and recursively removes them from the specified root path.
//!
//! Key steps:
//! - Parse CLI arguments (path, kind, dirs, log level)
//! - Set up logger for colored, timestamped output
//! - Determine which directories to clean based on project kind or user override
//! - Recursively walk the directory tree and remove matching directories
//! - Log all actions and errors
//!
//! Supports cleaning for Rust, Python, Java, Node.js, Go, C#, C++, PHP, Ruby, and common IDEs.

mod args;
mod utils;
use clap::Parser;
use clap::ValueEnum;
use color_eyre::eyre::{eyre, Result};
use log::info;
use std::fs;
use walkdir::WalkDir;

use crate::args::Args;
use crate::utils::{default_dirs_for_kind, setup_logger};

/// Main entry point for the Cleaner CLI tool.
///
/// Parses command-line arguments, sets up logging, determines which directories to clean,
/// and recursively removes them from the specified root path.
///
/// # Returns
/// * `Result<()>` - Ok on success, error if the path does not exist or a removal fails.
#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();
    // Set up logger with thread info and user-specified log level
    setup_logger(true, Some(&args.log));

    let path = args.path;

    info!("Cleaning directory: {}", &path);
    // Check if the provided path exists
    if fs::metadata(&path).is_err() {
        return Err(eyre!(format!("Path {} do not exist", &path)));
    }

    // Determine which directories to clean based on kind or user override
    let default_dirs = match &args.kind {
        Some(kind) => default_dirs_for_kind(kind),
        None => {
            use std::collections::HashSet;
            args::ProjectKind::value_variants()
                .iter()
                .flat_map(default_dirs_for_kind)
                .collect::<HashSet<_>>() // deduplicate
                .into_iter()
                .collect::<Vec<_>>() // convert back to Vec
        }
    };
    // Use user-supplied dirs if provided, otherwise use defaults
    let dirs: Vec<&str> = match &args.dirs {
        Some(dirs) => dirs.split(',').collect(),
        None => default_dirs,
    };

    info!(
        "Cleaning all directories that finished with either: {:?}",
        dirs
    );

    // If not forced, prompt the user for confirmation before deleting
    if !args.force {
        use std::io::{self, Write};
        println!("WARNING: The following directories will be deleted recursively:");
        for d in &dirs {
            println!("  - {}", d);
        }
        print!("Are you sure you want to proceed? [y/N]: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Aborted by user.");
            return Ok(());
        }
    }

    // Recursively walk the directory tree and remove matching directories
    for file in WalkDir::new(&path).into_iter().filter_map(|file| {
        let f = file.unwrap();
        // Only consider directories that match any of the target names
        if f.file_type().is_dir() && dirs.iter().any(|v| f.path().ends_with(v)) {
            Some(f)
        } else {
            None
        }
    }) {
        let path = file.path();
        info!("removing: {}", path.display());
        // Remove the directory and all its contents
        fs::remove_dir_all(path).unwrap();
    }

    info!("DONE.");
    Ok(())
}
