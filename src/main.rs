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
use color_eyre::eyre::{eyre, Result};
use log::info;
use std::fs;
use walkdir::WalkDir;

use crate::args::Args;
use crate::utils::{default_dirs_for_kind, setup_logger};

/// Parse CLI arguments and check if the path exists.
fn parse_and_validate_args() -> Result<(Args, String)> {
    let args = Args::parse();
    let path = args.path.clone();
    if fs::metadata(&path).is_err() {
        return Err(eyre!(format!("Path {} do not exist", &path)));
    }
    Ok((args, path))
}

/// Determine which directories to clean based on kind or user override, deduplicated.
fn determine_dirs_to_clean(args: &Args) -> Vec<&str> {
    match &args.kind {
        Some(kind) => default_dirs_for_kind(kind),
        None => default_dirs_for_kind(&args::ProjectKind::All),
    }
    .into_iter()
    .collect::<Vec<_>>()
}

/// Prompt the user for confirmation unless force is set. Returns true if confirmed.
fn confirm_deletion(dirs: &[&str], force: bool) -> bool {
    if force {
        return true;
    }
    use std::io::{self, Write};
    println!("WARNING: The following directories will be deleted recursively:");
    for d in dirs {
        println!("  - {}", d);
    }
    print!("Are you sure you want to proceed? [y/N]: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();
    input == "y" || input == "yes"
}

/// Recursively walk the directory tree and remove matching directories, or just print if dry_run is true.
fn clean_directories(path: &str, dirs: &[&str], dry_run: bool) {
    info!(
        "Cleaning all directories that finished with either: {:?}",
        dirs
    );
    for file in WalkDir::new(path).into_iter().filter_map(|file| {
        let f = file.unwrap();
        if f.file_type().is_dir() && dirs.iter().any(|v| f.path().ends_with(v)) {
            Some(f)
        } else {
            None
        }
    }) {
        let path = file.path();
        if dry_run {
            println!("Would remove: {}", path.display());
        } else {
            info!("removing: {}", path.display());
            fs::remove_dir_all(path).unwrap();
        }
    }
}

/// Main entry point for the Cleaner CLI tool.
///
/// Parses command-line arguments, sets up logging, determines which directories to clean,
/// and recursively removes them from the specified root path.
///
/// # Returns
/// * `Result<()>` - Ok on success, error if the path does not exist or a removal fails.
#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments and validate path
    let (args, path) = parse_and_validate_args()?;
    // Set up logger with thread info and user-specified log level
    setup_logger(true, Some(&args.log));
    // Determine which directories to clean
    let dirs = determine_dirs_to_clean(&args);
    // Confirm deletion unless forced
    if !confirm_deletion(&dirs, args.force) {
        println!("Aborted by user.");
        return Ok(());
    }
    // Clean the directories
    clean_directories(&path, &dirs, args.dry_run);
    info!("DONE.");
    Ok(())
}
