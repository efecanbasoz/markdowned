use std::fs;
use std::path::Path;

pub fn validate_within_workspace(path: &str, workspace_root: &Option<String>) -> Result<(), String> {
    let root = workspace_root.as_ref()
        .ok_or_else(|| "No workspace open".to_string())?;

    let canonical_root = fs::canonicalize(root)
        .map_err(|e| format!("Invalid workspace root: {e}"))?;

    // For new files, canonicalize parent directory
    let canonical_path = fs::canonicalize(path)
        .or_else(|_| {
            Path::new(path).parent()
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no parent"))
                .and_then(|p| fs::canonicalize(p))
                .map(|p| p.join(Path::new(path).file_name().unwrap_or_default()))
        })
        .map_err(|e| format!("Invalid path: {e}"))?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err("Path is outside workspace boundary".to_string());
    }
    Ok(())
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
    let root = state.root.lock().unwrap().clone();
    validate_within_workspace(&path, &root)?;
    tokio::task::spawn_blocking(move || open_file_impl(&path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn save_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let root = state.root.lock().unwrap().clone();
    validate_within_workspace(&path, &root)?;
    tokio::task::spawn_blocking(move || save_file_impl(&path, &content))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn create_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<(), String> {
    let root = state.root.lock().unwrap().clone();
    validate_within_workspace(&path, &root)?;
    tokio::task::spawn_blocking(move || create_file_impl(&path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn delete_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<(), String> {
    let root = state.root.lock().unwrap().clone();
    validate_within_workspace(&path, &root)?;
    tokio::task::spawn_blocking(move || delete_file_impl(&path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn rename_file(
    state: tauri::State<'_, crate::WorkspaceState>,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let root = state.root.lock().unwrap().clone();
    validate_within_workspace(&old_path, &root)?;
    validate_within_workspace(&new_path, &root)?;
    tokio::task::spawn_blocking(move || rename_file_impl(&old_path, &new_path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}
