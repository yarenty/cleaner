use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use assert_cmd::Command;
use tempfile::tempdir;

// Helper to create a directory and a file inside it
fn create_dir_with_file(base: &Path, dir: &str, file: &str) {
    let dir_path = base.join(dir);
    fs::create_dir_all(&dir_path).unwrap();
    let file_path = dir_path.join(file);
    let mut f = File::create(file_path).unwrap();
    writeln!(f, "test").unwrap();
}

#[test]
fn cleans_build_dirs_and_leaves_normal_files() {
    let temp = tempdir().unwrap();
    let root = temp.path();

    // Create build dirs and normal dirs/files
    create_dir_with_file(root, "target", "should_delete.txt");
    create_dir_with_file(root, "node_modules", "should_delete.txt");
    create_dir_with_file(root, ".idea", "should_delete.txt");
    create_dir_with_file(root, "src", "main.rs"); // should NOT be deleted
    File::create(root.join("README.md")).unwrap(); // should NOT be deleted

    // Run the cleaner binary
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root);
    cmd.assert().success();

    // Assert build dirs are gone
    assert!(!root.join("target").exists());
    assert!(!root.join("node_modules").exists());
    assert!(!root.join(".idea").exists());
    // Assert normal dirs/files remain
    assert!(root.join("src").exists());
    assert!(root.join("src/main.rs").exists());
    assert!(root.join("README.md").exists());
} 