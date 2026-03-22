use markdowned_lib::commands::workspace::{scan_directory_impl, search_workspace_impl};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[cfg(unix)]
use std::os::unix::fs::symlink;

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

#[test]
fn test_search_finds_matches() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("test.md"),
        "# Hello World\nSome content\nHello again",
    )
    .unwrap();
    let results = search_workspace_impl(dir.path().to_str().unwrap(), "hello");
    assert!(results.is_ok());
    let results = results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].line_number, 1);
    assert_eq!(results[1].line_number, 3);
}

#[test]
fn test_search_caps_results() {
    let dir = TempDir::new().unwrap();
    let mut content = String::new();
    for i in 0..300 {
        content.push_str(&format!("match line {i}\n"));
    }
    fs::write(dir.path().join("big.md"), &content).unwrap();
    let results = search_workspace_impl(dir.path().to_str().unwrap(), "match");
    assert!(results.unwrap().len() <= 200);
}

#[test]
fn test_search_respects_gitignore() {
    let dir = TempDir::new().unwrap();
    // Need git init for ignore crate
    Command::new("git")
        .arg("init")
        .current_dir(dir.path())
        .output()
        .ok();
    fs::write(dir.path().join(".gitignore"), "ignored/").unwrap();
    fs::create_dir(dir.path().join("ignored")).unwrap();
    fs::write(dir.path().join("ignored/file.md"), "searchterm").unwrap();
    fs::write(dir.path().join("visible.md"), "searchterm here").unwrap();
    let results = search_workspace_impl(dir.path().to_str().unwrap(), "searchterm").unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].file_name.contains("visible"));
}

#[test]
fn test_search_empty_query() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("test.md"), "some content").unwrap();
    let results = search_workspace_impl(dir.path().to_str().unwrap(), "");
    assert!(results.is_ok());
    assert_eq!(results.unwrap().len(), 0);
}

#[cfg(unix)]
#[test]
fn test_scan_directory_skips_symlink_entries() {
    let dir = TempDir::new().unwrap();
    let outside = dir.path().join("outside.md");
    let link = dir.path().join("linked.md");
    fs::write(&outside, "outside").unwrap();
    symlink(&outside, &link).unwrap();

    let entries = scan_directory_impl(dir.path().to_str().unwrap()).unwrap();
    assert!(entries.iter().all(|entry| entry.name != "linked.md"));
}

#[cfg(unix)]
#[test]
fn test_search_does_not_follow_symlinked_files() {
    let dir = TempDir::new().unwrap();
    let outside_dir = TempDir::new().unwrap();
    let outside = outside_dir.path().join("secret.md");
    let link = dir.path().join("linked.md");

    fs::write(&outside, "super secret needle").unwrap();
    symlink(&outside, &link).unwrap();

    let results = search_workspace_impl(dir.path().to_str().unwrap(), "needle").unwrap();
    assert!(results.is_empty());
}
