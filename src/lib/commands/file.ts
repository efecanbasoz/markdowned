import { invoke } from "@tauri-apps/api/core";

export async function openFile(path: string): Promise<string> {
  return invoke<string>("open_file", { path });
}

export async function saveFile(path: string, content: string): Promise<void> {
  return invoke<void>("save_file", { path, content });
}

export async function createFile(path: string): Promise<void> {
  return invoke<void>("create_file", { path });
}

export async function deleteFile(path: string): Promise<void> {
  return invoke<void>("delete_file", { path });
}

export async function renameFile(oldPath: string, newPath: string): Promise<void> {
  return invoke<void>("rename_file", { oldPath, newPath });
}
