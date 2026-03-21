use crate::models::config::AppConfig;
use tauri::{AppHandle, Emitter};

/// QA-002: Completion events include request_id so the frontend can
/// correlate chunks to the correct request and ignore stale ones.
#[derive(Clone, serde::Serialize)]
struct CompletionChunk {
    request_id: String,
    text: String,
}

#[derive(Clone, serde::Serialize)]
struct CompletionDone {
    request_id: String,
}

#[tauri::command]
pub async fn request_completion(app: AppHandle, context: String, request_id: String) -> Result<(), String> {
    let mut config = AppConfig::load();
    let resolved_key = config.completion.resolve_api_key();
    config.completion.api_key = resolved_key;

    let rid = request_id.clone();
    crate::services::llm::stream_completion(&config.completion, &context, move |chunk| {
        let _ = app.emit("completion-chunk", CompletionChunk {
            request_id: rid.clone(),
            text: chunk,
        });
    })
    .await?;
    let _ = app.emit("completion-done", CompletionDone { request_id });
    Ok(())
}
