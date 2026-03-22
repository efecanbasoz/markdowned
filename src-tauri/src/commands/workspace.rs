use crate::models::config::AppConfig;
use crate::models::workspace::{FileEntry, FileType, SearchMatch};
use crate::services::watcher;
use ignore::WalkBuilder;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use tauri::Manager;
#[cfg(desktop)]
use tauri_plugin_dialog::{DialogExt, FilePath};
#[cfg(desktop)]
use tokio::sync::oneshot;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSelection {
    pub root: String,
    pub name: String,
    pub entries: Vec<FileEntry>,
}

impl WorkspaceSelection {
    fn new(root: String, entries: Vec<FileEntry>) -> Self {
        let name = Path::new(&root)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Workspace".to_string());
        Self {
            root,
            name,
            entries,
        }
    }
}

fn canonicalize_workspace_root(path: &Path) -> Result<PathBuf, String> {
    let canonical =
        std::fs::canonicalize(path).map_err(|e| format!("Invalid workspace path: {e}"))?;

    if !canonical.is_dir() {
        return Err(format!("Not a directory: {}", canonical.display()));
    }

    let canonical_str = canonical.to_string_lossy();

    // Reject filesystem root as workspace.
    if canonical_str == "/" || canonical_str == "\\" {
        return Err("Cannot use filesystem root as workspace".to_string());
    }

    // Reject paths with too few components (e.g., /home or /Users).
    if canonical.components().count() < 3 {
        return Err("Workspace path is too broad. Choose a more specific directory.".to_string());
    }

    Ok(canonical)
}

fn canonicalize_registered_root(path: &str, workspace_roots: &[String]) -> Result<String, String> {
    let canonical = canonicalize_workspace_root(Path::new(path))?
        .to_string_lossy()
        .to_string();
    if workspace_roots.iter().any(|root| root == &canonical) {
        Ok(canonical)
    } else {
        Err("Workspace is not approved".to_string())
    }
}

fn remember_workspace_root(
    state: &crate::WorkspaceState,
    canonical_root: &str,
) -> Result<(), String> {
    let mut roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?;
    if !roots.contains(&canonical_root.to_string()) {
        roots.push(canonical_root.to_string());
    }
    Ok(())
}

fn persist_workspace_roots(workspace_roots: &[String]) -> Result<(), String> {
    let mut config = AppConfig::load();
    config.workspaces = workspace_roots.to_vec();
    config.save()
}

fn is_saved_workspace(canonical_root: &Path) -> bool {
    AppConfig::load().workspaces.iter().any(|saved| {
        std::fs::canonicalize(saved)
            .map(|saved_root| saved_root == canonical_root)
            .unwrap_or(false)
    })
}

fn is_symlink_path(path: &Path) -> bool {
    path.symlink_metadata()
        .map(|meta| meta.file_type().is_symlink())
        .unwrap_or(false)
}

fn is_path_within_root(path: &Path, root: &Path) -> bool {
    std::fs::canonicalize(path)
        .map(|canonical| canonical.starts_with(root))
        .unwrap_or(false)
}

pub fn scan_directory_impl(path: &str) -> Result<Vec<FileEntry>, String> {
    let root = canonicalize_workspace_root(Path::new(path))?;

    let walker = WalkBuilder::new(&root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    let mut dir_children: BTreeMap<String, Vec<FileEntry>> = BTreeMap::new();
    let root_str = root.to_string_lossy().to_string();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if entry_path == root.as_path() {
            continue;
        }

        if is_symlink_path(entry_path) || !is_path_within_root(entry_path, &root) {
            continue;
        }

        let parent = entry_path
            .parent()
            .unwrap_or(root.as_path())
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
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    let canonical_str = canonicalize_registered_root(&path, &roots)?;

    tokio::task::spawn_blocking(move || scan_directory_impl(&canonical_str))
        .await
        .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn add_workspace(
    window: tauri::WebviewWindow,
    state: tauri::State<'_, crate::WorkspaceState>,
    watcher_state: tauri::State<'_, watcher::WatcherState>,
) -> Result<Option<WorkspaceSelection>, String> {
    let app = window.app_handle().clone();

    #[cfg(desktop)]
    let selected = pick_workspace_folder(window).await?;

    #[cfg(not(desktop))]
    let selected = {
        let _ = window;
        None
    };

    let Some(selected) = selected else {
        return Ok(None);
    };

    let selected_path = selected.into_path().map_err(|e| e.to_string())?;
    let canonical_root = canonicalize_workspace_root(&selected_path)?
        .to_string_lossy()
        .to_string();

    let scan_root = canonical_root.clone();
    let entries = tokio::task::spawn_blocking(move || scan_directory_impl(&scan_root))
        .await
        .map_err(|e| format!("Task failed: {e}"))??;

    watcher::start_watcher(app, canonical_root.clone(), &watcher_state)?;
    remember_workspace_root(&state, &canonical_root)?;
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    persist_workspace_roots(&roots)?;

    Ok(Some(WorkspaceSelection::new(canonical_root, entries)))
}

#[tauri::command]
pub async fn restore_workspace(
    app: tauri::AppHandle,
    state: tauri::State<'_, crate::WorkspaceState>,
    watcher_state: tauri::State<'_, watcher::WatcherState>,
    path: String,
) -> Result<WorkspaceSelection, String> {
    let canonical_root = canonicalize_workspace_root(Path::new(&path))?;
    if !is_saved_workspace(&canonical_root) {
        return Err("Workspace was not previously approved".to_string());
    }

    let canonical_root = canonical_root.to_string_lossy().to_string();

    let scan_root = canonical_root.clone();
    let entries = tokio::task::spawn_blocking(move || scan_directory_impl(&scan_root))
        .await
        .map_err(|e| format!("Task failed: {e}"))??;

    watcher::start_watcher(app, canonical_root.clone(), &watcher_state)?;
    remember_workspace_root(&state, &canonical_root)?;

    Ok(WorkspaceSelection::new(canonical_root, entries))
}

#[cfg(desktop)]
async fn pick_workspace_folder(window: tauri::WebviewWindow) -> Result<Option<FilePath>, String> {
    let (tx, rx) = oneshot::channel();

    window
        .dialog()
        .file()
        .set_title("Open Workspace")
        .set_parent(&window)
        .pick_folder(move |folder_path| {
            let _ = tx.send(folder_path);
        });

    rx.await
        .map_err(|e| format!("Workspace picker failed: {e}"))
}

const MAX_SEARCH_RESULTS: usize = 200;

/// Check if a file is likely binary by looking for null bytes in the first 512 bytes.
fn is_binary(data: &[u8]) -> bool {
    let check_len = data.len().min(512);
    data[..check_len].contains(&0)
}

/// QA-007: Case-insensitive search that returns byte offset in the original string.
/// This avoids Unicode case-folding length mismatches from to_lowercase().
fn find_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    let needle_lower = needle.to_lowercase();
    let needle_chars: Vec<char> = needle_lower.chars().collect();
    if needle_chars.is_empty() {
        return None;
    }

    for (byte_pos, _) in haystack.char_indices() {
        let candidate: String = haystack[byte_pos..]
            .chars()
            .take(needle_chars.len())
            .collect();
        if candidate.to_lowercase() == needle_lower {
            return Some(byte_pos);
        }
    }
    None
}

/// Return the byte length of the matched region in the original string.
fn find_match_len(haystack: &str, start: usize, needle: &str) -> usize {
    let query_char_count = needle.chars().count();
    haystack[start..]
        .char_indices()
        .nth(query_char_count)
        .map(|(offset, _)| offset)
        .unwrap_or(haystack.len() - start)
}

pub fn search_workspace_impl(
    workspace_root: &str,
    query: &str,
) -> Result<Vec<SearchMatch>, String> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    let root = canonicalize_workspace_root(Path::new(workspace_root))?;

    let walker = WalkBuilder::new(&root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    let mut results = Vec::new();

    for entry in walker.flatten() {
        let entry_path = entry.path();
        if is_symlink_path(entry_path)
            || !entry_path.is_file()
            || !is_path_within_root(entry_path, &root)
        {
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
            // QA-007: Use byte-offset-safe case-insensitive search on the original line
            // to avoid Unicode case-folding length mismatches between lowered and original strings
            if let Some(pos) = find_case_insensitive(line, query) {
                results.push(SearchMatch {
                    file_path: file_path.clone(),
                    file_name: file_name.clone(),
                    line_number: line_idx + 1,
                    line_content: line.to_string(),
                    match_start: pos,
                    match_end: pos + find_match_len(line, pos, query),
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
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    if roots.is_empty() {
        return Err("No workspace open".to_string());
    }
    tokio::task::spawn_blocking(move || {
        let mut all_results = Vec::new();
        for root in &roots {
            match search_workspace_impl(root, &query) {
                Ok(mut results) => {
                    all_results.append(&mut results);
                    if all_results.len() >= MAX_SEARCH_RESULTS {
                        all_results.truncate(MAX_SEARCH_RESULTS);
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
        Ok(all_results)
    })
    .await
    .map_err(|e| format!("Task failed: {e}"))?
}

#[tauri::command]
pub async fn unwatch_workspace(
    state: tauri::State<'_, crate::WorkspaceState>,
    watcher_state: tauri::State<'_, watcher::WatcherState>,
    path: String,
) -> Result<(), String> {
    let current_roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    let canonical = canonicalize_registered_root(&path, &current_roots)?;
    {
        let mut roots = state
            .roots
            .lock()
            .map_err(|e| format!("State error: {e}"))?;
        roots.retain(|r| r != &canonical);
        persist_workspace_roots(&roots)?;
    }
    watcher::stop_watcher(canonical, &watcher_state)
}
