use crate::models::config::AppConfig;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn request_completion(app: AppHandle, context: String) -> Result<(), String> {
    let mut config = AppConfig::load();
    // Resolve API key from keychain if stored there
    let resolved_key = config.completion.resolve_api_key();
    config.completion.api_key = resolved_key;

    crate::services::llm::stream_completion(&config.completion, &context, |chunk| {
        let _ = app.emit("completion-chunk", &chunk);
    })
    .await?;
    let _ = app.emit("completion-done", ());
    Ok(())
}
