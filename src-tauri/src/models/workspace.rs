use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FileType {
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub file_type: FileType,
    pub children: Option<Vec<FileEntry>>,
}

impl FileEntry {
    pub fn new_file(name: String, path: String) -> Self {
        Self {
            name,
            path,
            file_type: FileType::File,
            children: None,
        }
    }

    pub fn new_directory(name: String, path: String, children: Vec<FileEntry>) -> Self {
        Self {
            name,
            path,
            file_type: FileType::Directory,
            children: Some(children),
        }
    }
}
