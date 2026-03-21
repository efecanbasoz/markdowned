pub mod commands;
pub mod models;
pub mod services;

use std::sync::{Arc, Mutex};

pub struct WorkspaceState {
    pub roots: Arc<Mutex<Vec<String>>>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self { roots: Arc::new(Mutex::new(Vec::new())) }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(WorkspaceState::default())
        .manage(crate::services::watcher::WatcherState::default())
        .invoke_handler(tauri::generate_handler![
            commands::file::open_file,
            commands::file::save_file,
            commands::file::create_file,
            commands::file::delete_file,
            commands::file::rename_file,
            commands::workspace::scan_directory,
            commands::workspace::watch_workspace,
            commands::workspace::search_workspace,
            commands::workspace::unwatch_workspace,
            commands::preview::render_preview,
            commands::config::load_config,
            commands::config::save_config,
            commands::completion::request_completion,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
