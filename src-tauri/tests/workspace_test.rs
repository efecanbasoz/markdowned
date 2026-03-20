use markdowned_lib::commands::workspace::scan_directory_impl;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_scan_directory_returns_tree() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("README.md"), "# Readme").unwrap();
    fs::create_dir(dir.path().join("docs")).unwrap();
    fs::write(dir.path().join("docs/guide.md"), "# Guide").unwrap();

    let result = scan_directory_impl(dir.path().to_str().unwrap());
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert!(entries.len() >= 2); // docs/ and README.md
}

#[test]
fn test_scan_directory_respects_gitignore() {
    let dir = TempDir::new().unwrap();

    // Initialize a git repo so the ignore crate respects .gitignore
    Command::new("git")
        .args(["init"])
        .current_dir(dir.path())
        .output()
        .expect("git init failed");

    fs::write(dir.path().join(".gitignore"), "node_modules/\n*.log").unwrap();
    fs::create_dir(dir.path().join("node_modules")).unwrap();
    fs::write(dir.path().join("node_modules/pkg.js"), "").unwrap();
    fs::write(dir.path().join("app.log"), "log").unwrap();
    fs::write(dir.path().join("readme.md"), "# Hi").unwrap();

    let result = scan_directory_impl(dir.path().to_str().unwrap());
    assert!(result.is_ok());
    let entries = result.unwrap();
    let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"readme.md"));
    assert!(!names.contains(&"node_modules"));
    assert!(!names.contains(&"app.log"));
}

#[test]
fn test_scan_directory_sorts_dirs_first() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("zebra.md"), "").unwrap();
    fs::create_dir(dir.path().join("alpha")).unwrap();
    fs::write(dir.path().join("alpha/file.md"), "").unwrap();

    let result = scan_directory_impl(dir.path().to_str().unwrap());
    assert!(result.is_ok());
    let entries = result.unwrap();
    // Directory should come before file
    assert_eq!(entries[0].name, "alpha");
    assert_eq!(entries[1].name, "zebra.md");
}

#[test]
fn test_scan_nonexistent_directory() {
    let result = scan_directory_impl("/nonexistent/path");
    assert!(result.is_err());
}
