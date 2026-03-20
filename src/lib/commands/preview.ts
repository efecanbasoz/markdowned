import { invoke } from "@tauri-apps/api/core";
import type { PreviewResult } from "../types";

export async function renderPreview(content: string): Promise<PreviewResult> {
  return invoke<PreviewResult>("render_preview", { content });
}
