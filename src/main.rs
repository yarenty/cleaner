mod args;
mod utils;
use clap::Parser;
use clap::ValueEnum;
use std::fs;
use walkdir::WalkDir;
use color_eyre::eyre::{eyre, Result};
use log::info;

use crate::args::Args;
use crate::utils::{setup_logger, default_dirs_for_kind};

/// Walk through specified directories and ...
/// - delete /target
/// - delete /out
/// - delete /build
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    setup_logger(true, Some(&args.log));
    
    let path = args.path;

    info!("Cleaning directory: {}", &path);
    if fs::metadata(&path).is_err() {
        return Err(eyre!(format!("Path {} do not exist", &path)));
    }

    // Determine which directories to clean
    let default_dirs = match &args.kind {
        Some(kind) => default_dirs_for_kind(kind),
        None => args::ProjectKind::value_variants()
            .iter()
            .flat_map(default_dirs_for_kind)
            .collect(),
    };
    let user_dirs: Vec<_> = args.dirs.split(',').collect();
    let dirs: Vec<&str> = if user_dirs == default_dirs {
        default_dirs
    } else {
        user_dirs
    };

    info!("Cleaning all directories that finished with either: {:?}", dirs);
    for file in WalkDir::new(&path).into_iter().filter_map(|file| {
        let f = file.unwrap();
        if f.file_type().is_dir() && dirs.iter().any(|v| f.path().ends_with(v)) {
            Some(f)
        } else {
            None
        }
    }) {
        let path = file.path();
        info!("removing: {}", path.display());
        fs::remove_dir_all(path).unwrap();
    }

    info!("DONE.");
    Ok(())
}
