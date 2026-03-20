use crate::models::config::AppConfig;
use crate::services::keychain;

#[tauri::command]
pub async fn load_config() -> Result<AppConfig, String> {
    tokio::task::spawn_blocking(|| {
        let mut config = AppConfig::load();

        // If the API key is stored in the keychain, resolve it for the frontend
        if config.completion.api_key_in_keychain {
            let provider_name = format!("{:?}", config.completion.provider).to_lowercase();
            if let Ok(Some(key)) = keychain::retrieve_api_key(&provider_name) {
                config.completion.api_key = key;
            }
        }

        config
    })
    .await
    .map_err(|e| format!("Task failed: {e}"))
}

#[tauri::command]
pub async fn save_config(mut config: AppConfig) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        // Attempt to store the API key in the OS keychain
        if !config.completion.api_key.is_empty() && keychain::is_available() {
            let provider_name = format!("{:?}", config.completion.provider).to_lowercase();
            if keychain::store_api_key(&provider_name, &config.completion.api_key).is_ok() {
                config.completion.api_key_in_keychain = true;
                // Clear the plaintext key before writing config to disk
                config.completion.api_key = String::new();
            }
        }

        config.save()
    })
    .await
    .map_err(|e| format!("Task failed: {e}"))?
}
