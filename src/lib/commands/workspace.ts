import { invoke } from "@tauri-apps/api/core";
import type { FileEntry, SearchMatch } from "../types";

export async function scanDirectory(path: string): Promise<FileEntry[]> {
  return invoke<FileEntry[]>("scan_directory", { path });
}

export async function searchWorkspace(query: string): Promise<SearchMatch[]> {
  return invoke<SearchMatch[]>("search_workspace", { query });
}
