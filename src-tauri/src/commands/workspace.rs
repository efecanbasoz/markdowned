use crate::models::workspace::{FileEntry, FileType, SearchMatch};
use crate::services::watcher;
use ignore::WalkBuilder;
use std::collections::BTreeMap;
use std::path::Path;

pub fn scan_directory_impl(path: &str) -> Result<Vec<FileEntry>, String> {
    let root = Path::new(path);
    if !root.is_dir() {
        return Err(format!("Not a directory: {path}"));
    }

    let walker = WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    let mut dir_children: BTreeMap<String, Vec<FileEntry>> = BTreeMap::new();
    let root_str = root.to_string_lossy().to_string();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if entry_path == root {
            continue;
        }

        let parent = entry_path
            .parent()
            .unwrap_or(root)
            .to_string_lossy()
            .to_string();

        let name = entry_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let full_path = entry_path.to_string_lossy().to_string();

        let file_entry = if entry_path.is_dir() {
            FileEntry {
                name,
                path: full_path,
                file_type: FileType::Directory,
                children: Some(Vec::new()),
            }
        } else {
            FileEntry::new_file(name, full_path)
        };

        dir_children.entry(parent).or_default().push(file_entry);
    }

    fn build_tree(path: &str, dir_children: &BTreeMap<String, Vec<FileEntry>>) -> Vec<FileEntry> {
        let mut entries = match dir_children.get(path) {
            Some(children) => children.clone(),
            None => return Vec::new(),
        };

        for entry in &mut entries {
            if entry.file_type == FileType::Directory {
                entry.children = Some(build_tree(&entry.path, dir_children));
            }
        }

        entries.sort_by(|a, b| match (&a.file_type, &b.file_type) {
            (FileType::Directory, FileType::File) => std::cmp::Ordering::Less,
            (FileType::File, FileType::Directory) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        entries
    }

    Ok(build_tree(&root_str, &dir_children))
}

#[tauri::command]
pub async fn scan_directory(
    state: tauri::State<'_, crate::WorkspaceState>,
    path: String,
) -> Result<Vec<FileEntry>, String> {
    // Set workspace root before spawn_blocking (state access must be on the async side)
    {
        let mut root = state.root.lock().unwrap();
        *root = Some(path.clone());
    }
    tokio::task::spawn_blocking(move || scan_directory_impl(&path))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn watch_workspace(
    app: tauri::AppHandle,
    watcher_state: tauri::State<'_, watcher::WatcherState>,
    path: String,
) -> Result<(), String> {
    watcher::start_watcher(app, path, &watcher_state)
}

const MAX_SEARCH_RESULTS: usize = 200;

/// Check if a file is likely binary by looking for null bytes in the first 512 bytes.
fn is_binary(data: &[u8]) -> bool {
    let check_len = data.len().min(512);
    data[..check_len].contains(&0)
}

pub fn search_workspace_impl(workspace_root: &str, query: &str) -> Result<Vec<SearchMatch>, String> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let root = Path::new(workspace_root);
    if !root.is_dir() {
        return Err(format!("Not a directory: {workspace_root}"));
    }

    let query_lower = query.to_lowercase();
    let walker = WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    let mut results = Vec::new();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if !entry_path.is_file() {
            continue;
        }

        let content = match std::fs::read(entry_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if is_binary(&content) {
            continue;
        }

        let text = match String::from_utf8(content) {
            Ok(t) => t,
            Err(_) => continue,
        };

        let file_name = entry_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_path = entry_path.to_string_lossy().to_string();

        for (line_idx, line) in text.lines().enumerate() {
            let line_lower = line.to_lowercase();
            if let Some(pos) = line_lower.find(&query_lower) {
                results.push(SearchMatch {
                    file_path: file_path.clone(),
                    file_name: file_name.clone(),
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: pos,
                    match_end: pos + query.len(),
                });

                if results.len() >= MAX_SEARCH_RESULTS {
                    return Ok(results);
                }
            }
        }
    }

    Ok(results)
}

#[tauri::command]
pub async fn search_workspace(
    state: tauri::State<'_, crate::WorkspaceState>,
    query: String,
) -> Result<Vec<SearchMatch>, String> {
    let root = state
        .root
        .lock()
        .unwrap()
        .clone()
        .ok_or_else(|| "No workspace open".to_string())?;
    tokio::task::spawn_blocking(move || search_workspace_impl(&root, &query))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}
