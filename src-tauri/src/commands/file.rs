use std::fs;
use std::path::Path;

/// SEC-001: Validate path is within workspace and return the CANONICAL path.
/// All I/O must use the returned canonical path to prevent TOCTOU races
/// where a symlink is swapped between validation and file operation.
pub fn resolve_safe_path(path: &str, workspace_roots: &[String]) -> Result<String, String> {
    if workspace_roots.is_empty() {
        return Err("No workspace open".to_string());
    }

    // For new files, canonicalize parent directory + filename
    let canonical_path = fs::canonicalize(path)
        .or_else(|_| {
            Path::new(path)
                .parent()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no parent"))
                .and_then(|p| fs::canonicalize(p))
                .map(|p| p.join(Path::new(path).file_name().unwrap_or_default()))
        })
        .map_err(|e| format!("Invalid path: {e}"))?;

    // Check if path falls within any workspace root
    let in_workspace = workspace_roots.iter().any(|root| {
        fs::canonicalize(root)
            .map(|canonical_root| canonical_path.starts_with(&canonical_root))
            .unwrap_or(false)
    });

    if !in_workspace {
        return Err("Path is outside workspace boundary".to_string());
    }

    // Reject symlinks (the canonical path itself should not be a symlink)
    if path_is_symlink(Path::new(path)) {
        return Err("Symlinks are not allowed".to_string());
    }

    Ok(canonical_path.to_string_lossy().to_string())
}

fn path_is_symlink(path: &Path) -> bool {
    path.symlink_metadata()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn open_file_impl(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {e}"))
}

pub fn save_file_impl(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write file: {e}"))
}

pub fn create_file_impl(path: &str) -> Result<(), String> {
    if Path::new(path).exists() {
        return Err("File already exists".to_string());
    }
    fs::write(path, "").map_err(|e| format!("Failed to create file: {e}"))
}

pub fn delete_file_impl(path: &str) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| format!("Failed to delete file: {e}"))
}

pub fn rename_file_impl(old_path: &str, new_path: &str) -> Result<(), String> {
    fs::rename(old_path, new_path).map_err(|e| format!("Failed to rename file: {e}"))
}

#[tauri::command]
pub async fn open_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<String, String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("Workspace state error: {e}"))?
        .clone();
    let safe_path = resolve_safe_path(&path, &roots)?;
    tokio::task::spawn_blocking(move || open_file_impl(&safe_path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn save_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("Workspace state error: {e}"))?
        .clone();
    let safe_path = resolve_safe_path(&path, &roots)?;
    tokio::task::spawn_blocking(move || save_file_impl(&safe_path, &content))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn create_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<(), String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("Workspace state error: {e}"))?
        .clone();
    let safe_path = resolve_safe_path(&path, &roots)?;
    tokio::task::spawn_blocking(move || create_file_impl(&safe_path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn delete_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<(), String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("Workspace state error: {e}"))?
        .clone();
    let safe_path = resolve_safe_path(&path, &roots)?;
    tokio::task::spawn_blocking(move || delete_file_impl(&safe_path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn rename_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("Workspace state error: {e}"))?
        .clone();
    let safe_old = resolve_safe_path(&old_path, &roots)?;
    let safe_new = resolve_safe_path(&new_path, &roots)?;
    tokio::task::spawn_blocking(move || rename_file_impl(&safe_old, &safe_new))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}
