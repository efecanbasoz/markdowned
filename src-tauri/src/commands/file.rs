use std::fs;

pub fn open_file_impl(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read file: {e}"))
}

pub fn save_file_impl(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to write file: {e}"))
}

pub fn create_file_impl(path: &str) -> Result<(), String> {
    if std::path::Path::new(path).exists() {
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
pub async fn open_file(path: String) -> Result<String, String> {
    open_file_impl(&path)
}

#[tauri::command]
pub async fn save_file(path: String, content: String) -> Result<(), String> {
    save_file_impl(&path, &content)
}

#[tauri::command]
pub async fn create_file(path: String) -> Result<(), String> {
    create_file_impl(&path)
}

#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    delete_file_impl(&path)
}

#[tauri::command]
pub async fn rename_file(old_path: String, new_path: String) -> Result<(), String> {
    rename_file_impl(&old_path, &new_path)
}
