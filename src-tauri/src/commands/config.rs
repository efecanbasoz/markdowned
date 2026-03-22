use crate::models::config::AppConfig;
use crate::services::keychain;

#[tauri::command]
pub async fn load_config() -> Result<AppConfig, String> {
    tokio::task::spawn_blocking(|| {
        let mut config = AppConfig::load();

        // SEC-002: Never send the actual API key to the frontend.
        // The frontend only needs to know whether a key is configured.
        // The actual key is resolved server-side in request_completion.
        config.completion.api_key = String::new();

        config
    })
    .await
    .map_err(|e| format!("Task failed: {e}"))
}

#[tauri::command]
pub async fn save_config(mut config: AppConfig) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        // Workspace approvals are managed server-side by workspace commands.
        // Ignore any frontend attempt to replace them wholesale.
        let existing = AppConfig::load();
        config.workspaces = existing.workspaces;
        config.last_workspace = existing.last_workspace;

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
