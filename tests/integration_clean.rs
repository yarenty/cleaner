use assert_cmd::Command;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;

// Helper to create a directory and a file inside it
fn create_dir_with_file(base: &Path, dir: &str, file: &str) {
    let dir_path = base.join(dir);
    fs::create_dir_all(&dir_path).unwrap();
    let file_path = dir_path.join(file);
    let mut f = File::create(file_path).unwrap();
    writeln!(f, "test").unwrap();
}

/// Test that --dry-run does not actually delete directories and prints what would be removed.
#[test]
fn dry_run_does_not_delete() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(root, "target", "should_delete.txt");
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root).arg("--dry-run");
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("Would remove"));
    assert!(root.join("target").exists());
}

/// Test that --exclude skips the specified directory from deletion.
#[test]
fn exclude_skips_dir() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(root, "target", "should_delete.txt");
    create_dir_with_file(root, "keepme", "should_keep.txt");
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root)
        .arg("--force")
        .arg("--dirs=target,keepme")
        .arg("--exclude=keepme");
    cmd.assert().success();
    assert!(!root.join("target").exists());
    assert!(root.join("keepme").exists());
}

/// Test that --max-depth prevents deletion of directories deeper than the specified depth.
#[test]
fn max_depth_limits_search() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(&root.join("a/b"), "target", "should_delete.txt");
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root).arg("--force").arg("--max-depth=1");
    cmd.assert().success();
    // target is at depth 2, should not be deleted
    assert!(root.join("a/b/target").exists());
}

/// Test that --log-file creates a log file with output.
#[test]
fn log_file_is_created() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(root, "target", "should_delete.txt");
    let log_path = root.join("cleaner.log");
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root)
        .arg("--force")
        .arg("--log-file")
        .arg(&log_path);
    cmd.assert().success();
    assert!(log_path.exists());
}

/// Test that --ci outputs a JSON summary and suppresses prompts.
#[test]
fn ci_outputs_json() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(root, "target", "should_delete.txt");
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root).arg("--ci");
    let output = cmd.assert().success().get_output().stdout.clone();
    let s = String::from_utf8_lossy(&output);
    assert!(s.contains("directories"));
    assert!(s.contains("total_bytes"));
}

/// Test that a config file can specify custom directories to clean.
#[test]
fn config_file_dirs() {
    let temp = tempdir().unwrap();
    let root = temp.path();
    create_dir_with_file(root, "custom", "should_delete.txt");
    let config_path = root.join("cleaner.toml");
    let config = r#"
[kinds.all]
dirs = ["custom"]
"#;
    std::fs::write(&config_path, config).unwrap();
    let mut cmd = Command::cargo_bin("cleaner").unwrap();
    cmd.arg(root)
        .arg("--force")
        .arg("--config")
        .arg(&config_path);
    cmd.assert().success();
    assert!(!root.join("custom").exists());
}
