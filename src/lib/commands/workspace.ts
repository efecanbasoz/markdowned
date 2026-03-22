import { invoke } from "@tauri-apps/api/core";
import type { FileEntry, SearchMatch, WorkspaceSelection } from "../types";

export async function scanDirectory(path: string): Promise<FileEntry[]> {
  return invoke<FileEntry[]>("scan_directory", { path });
}

export async function addWorkspace(): Promise<WorkspaceSelection | null> {
  return invoke<WorkspaceSelection | null>("add_workspace");
}

export async function restoreWorkspace(path: string): Promise<WorkspaceSelection> {
  return invoke<WorkspaceSelection>("restore_workspace", { path });
}

export async function searchWorkspace(query: string): Promise<SearchMatch[]> {
  return invoke<SearchMatch[]>("search_workspace", { query });
}

export async function unwatchWorkspace(path: string): Promise<void> {
  return invoke<void>("unwatch_workspace", { path });
}
