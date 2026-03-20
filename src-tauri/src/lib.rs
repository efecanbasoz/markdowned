pub mod commands;
mod models;
pub mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::file::open_file,
            commands::file::save_file,
            commands::file::create_file,
            commands::file::delete_file,
            commands::file::rename_file,
            commands::workspace::scan_directory,
            commands::preview::render_preview,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
