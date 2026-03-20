use crate::services::renderer;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResult {
    pub html: String,
    pub frontmatter: Option<String>,
}

#[tauri::command]
pub async fn render_preview(content: String) -> Result<PreviewResult, String> {
    let (frontmatter, html) = renderer::render_markdown_with_frontmatter(&content);
    Ok(PreviewResult { html, frontmatter })
}
