use crate::services::renderer;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResult {
    pub html: String,
    pub frontmatter: Option<String>,
}

/// QA-006: Offload CPU-heavy rendering to a blocking thread to avoid
/// starving the async runtime during typing.
#[tauri::command]
pub async fn render_preview(content: String) -> Result<PreviewResult, String> {
    tokio::task::spawn_blocking(move || {
        let (frontmatter, html) = renderer::render_markdown_with_frontmatter(&content);
        PreviewResult { html, frontmatter }
    })
    .await
    .map_err(|e| format!("Preview task failed: {e}"))
}
