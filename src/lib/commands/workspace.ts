import { invoke } from "@tauri-apps/api/core";
import type { FileEntry } from "../types";

export async function scanDirectory(path: string): Promise<FileEntry[]> {
  return invoke<FileEntry[]>("scan_directory", { path });
}
