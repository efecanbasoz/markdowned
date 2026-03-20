use crate::models::config::AppConfig;

#[tauri::command]
pub async fn load_config() -> Result<AppConfig, String> {
    Ok(AppConfig::load())
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    config.save()
}
