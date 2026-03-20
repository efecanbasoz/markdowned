use crate::models::workspace::{FileEntry, FileType};
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
pub async fn scan_directory(path: String) -> Result<Vec<FileEntry>, String> {
    scan_directory_impl(&path)
}
