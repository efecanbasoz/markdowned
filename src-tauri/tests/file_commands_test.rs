use std::fs;
use tempfile::TempDir;

use markdowned_lib::commands::file::*;

#[test]
fn test_open_file_returns_content() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.md");
    fs::write(&path, "# Hello\n\nWorld").unwrap();

    let result = open_file_impl(path.to_str().unwrap());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "# Hello\n\nWorld");
}

#[test]
fn test_open_nonexistent_file_returns_error() {
    let result = open_file_impl("/tmp/nonexistent_markdowned_test_file.md");
    assert!(result.is_err());
}

#[test]
fn test_save_file_writes_content() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("output.md");

    let result = save_file_impl(path.to_str().unwrap(), "# Saved\n\nContent");
    assert!(result.is_ok());

    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "# Saved\n\nContent");
}

#[test]
fn test_create_file_creates_new() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("new_file.md");

    let result = create_file_impl(path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(path.exists());

    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "");
}

#[test]
fn test_create_file_fails_if_exists() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("existing.md");
    fs::write(&path, "already here").unwrap();

    let result = create_file_impl(path.to_str().unwrap());
    assert!(result.is_err());
}

#[test]
fn test_delete_file_removes_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("to_delete.md");
    fs::write(&path, "delete me").unwrap();
    assert!(path.exists());

    let result = delete_file_impl(path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(!path.exists());
}

#[test]
fn test_rename_file_moves_file() {
    let dir = TempDir::new().unwrap();
    let old_path = dir.path().join("old_name.md");
    let new_path = dir.path().join("new_name.md");
    fs::write(&old_path, "# Renamed").unwrap();

    let result = rename_file_impl(old_path.to_str().unwrap(), new_path.to_str().unwrap());
    assert!(result.is_ok());
    assert!(!old_path.exists());
    assert!(new_path.exists());

    let content = fs::read_to_string(&new_path).unwrap();
    assert_eq!(content, "# Renamed");
}

#[test]
fn test_validate_rejects_path_outside_workspace() {
    let dir = TempDir::new().unwrap();
    let workspace = dir.path().to_str().unwrap().to_string();
    let result = markdowned_lib::commands::file::validate_within_workspace(
        "/etc/passwd",
        &Some(workspace),
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("outside workspace"));
}

#[test]
fn test_validate_accepts_path_inside_workspace() {
    let dir = TempDir::new().unwrap();
    let file = dir.path().join("test.md");
    fs::write(&file, "test").unwrap();
    let workspace = dir.path().to_str().unwrap().to_string();
    let result = markdowned_lib::commands::file::validate_within_workspace(
        file.to_str().unwrap(),
        &Some(workspace),
    );
    assert!(result.is_ok());
}

#[test]
fn test_validate_rejects_when_no_workspace() {
    let result = markdowned_lib::commands::file::validate_within_workspace(
        "/some/file.md",
        &None,
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No workspace"));
}
