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
use glob::Pattern;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use rayon::prelude::*;

use crate::args::Args;
use crate::utils::{default_dirs_for_kind, setup_logger};

#[derive(Debug, Deserialize)]
struct Config {
    kinds: Option<std::collections::HashMap<String, KindConfig>>,
    exclude: Option<ExcludeConfig>,
}

#[derive(Debug, Deserialize)]
struct KindConfig {
    dirs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ExcludeConfig {
    patterns: Option<Vec<String>>,
}

/// Load config from a TOML file path, if provided.
fn load_config(path: &str) -> Option<Config> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    toml::from_str(&contents).ok()
}

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
fn determine_dirs_to_clean(args: &Args, config: &Option<Config>) -> Vec<String> {
    // CLI takes precedence, then config, then default
    if let Some(dirs) = &args.dirs {
        return dirs.split(',').map(|s| s.to_string()).collect();
    }
    if let Some(cfg) = config {
        if let Some(kinds) = &cfg.kinds {
            let kind_key = args.kind.as_ref().map(|k| format!("{}", k).to_lowercase()).unwrap_or("all".to_string());
            if let Some(kind_cfg) = kinds.get(&kind_key) {
                if let Some(dirs) = &kind_cfg.dirs {
                    return dirs.clone();
                }
            }
        }
    }
    // Fallback to built-in logic
    match &args.kind {
        Some(kind) => default_dirs_for_kind(kind),
        None => default_dirs_for_kind(&args::ProjectKind::All),
    }
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn determine_exclude(args: &Args, config: &Option<Config>) -> Vec<String> {
    // CLI takes precedence, then config, then empty
    if let Some(ex) = &args.exclude {
        return ex.split(',').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
    }
    if let Some(cfg) = config {
        if let Some(exclude) = &cfg.exclude {
            if let Some(patterns) = &exclude.patterns {
                return patterns.clone();
            }
        }
    }
    vec![]
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
/// Returns (number of directories, total bytes that would be or were deleted)
fn clean_directories(path: &str, dirs: &[&str], dry_run: bool, exclude: &[&str], max_depth: usize, interactive: bool, force: bool) -> (usize, u64) {
    info!(
        "Cleaning all directories that finished with either: {:?}, excluding: {:?}, max_depth: {}",
        dirs, exclude, max_depth
    );
    let mut walkdir = WalkDir::new(path);
    if max_depth > 0 {
        walkdir = walkdir.max_depth(max_depth);
    }
    // Compile glob patterns for dirs and exclude
    let dir_patterns: Vec<Pattern> = dirs.iter().filter_map(|p| Pattern::new(p).ok()).collect();
    let exclude_patterns: Vec<Pattern> = exclude.iter().filter_map(|p| Pattern::new(p).ok()).collect();
    // Collect all target directories first
    let targets: Vec<_> = walkdir
        .into_iter()
        .filter_map(|file| {
            let f = file.unwrap();
            let file_path = f.path();
            let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if f.file_type().is_dir()
                && dir_patterns.iter().any(|pat| pat.matches(file_name))
                && !exclude_patterns.iter().any(|pat| pat.matches(file_name))
            {
                Some(file_path.to_path_buf())
            } else {
                None
            }
        })
        .collect();
    let count = targets.len();
    let mut total_bytes = 0u64;
    if dry_run {
        for path in &targets {
            println!("Would remove: {}", path.display());
            if let Ok(meta) = fs::metadata(path) {
                total_bytes += meta.len();
            }
        }
    } else if interactive && !force {
        use std::io::{self, Write};
        for path in &targets {
            print!("Delete {}? [y/N]: ", path.display());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" {
                info!("removing: {}", path.display());
                let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                let _ = fs::remove_dir_all(path);
                total_bytes += size;
            } else {
                println!("Skipped: {}", path.display());
            }
        }
    } else {
        total_bytes = targets
            .par_iter()
            .map(|path| {
                info!("removing: {}", path.display());
                let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                let _ = fs::remove_dir_all(path);
                size
            })
            .sum();
    }
    (count, total_bytes)
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
    setup_logger(true, Some(&args.log), args.log_file.as_deref());
    // Load config if provided
    let config = args.config.as_deref().map(load_config).flatten();
    // Determine which directories to clean
    let dirs = determine_dirs_to_clean(&args, &config);
    // Parse exclude list
    let exclude = determine_exclude(&args, &config);
    // Confirm deletion unless forced
    if !confirm_deletion(&dirs.iter().map(|s| s.as_str()).collect::<Vec<_>>(), args.force) {
        println!("Aborted by user.");
        return Ok(());
    }
    // Clean the directories
    let (count, total_bytes) = clean_directories(
        &path,
        &dirs.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        args.dry_run,
        &exclude.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        args.max_depth,
        args.interactive,
        args.force,
    );
    if args.dry_run {
        println!("Dry run: {} directories would be removed.", count);
    } else {
        println!("Removed {} directories. (Total size: {:.2} MB)", count, total_bytes as f64 / 1_048_576.0);
    }
    info!("DONE.");
    Ok(())
}
