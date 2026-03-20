use crate::models::config::AppConfig;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub async fn request_completion(app: AppHandle, context: String) -> Result<(), String> {
    let config = AppConfig::load();
    crate::services::llm::stream_completion(&config.completion, &context, |chunk| {
        let _ = app.emit("completion-chunk", &chunk);
    })
    .await?;
    let _ = app.emit("completion-done", ());
    Ok(())
}
