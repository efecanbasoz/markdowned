import type { FileEntry } from "../types";

let entries = $state<FileEntry[]>([]);
let activeFilePath = $state<string | null>(null);
let workspaceRoot = $state<string | null>(null);
let sidebarVisible = $state(true);

export const workspace = {
  get entries() { return entries; },
  set entries(v: FileEntry[]) { entries = v; },
  get activeFilePath() { return activeFilePath; },
  set activeFilePath(v: string | null) { activeFilePath = v; },
  get workspaceRoot() { return workspaceRoot; },
  set workspaceRoot(v: string | null) { workspaceRoot = v; },
  get sidebarVisible() { return sidebarVisible; },
  set sidebarVisible(v: boolean) { sidebarVisible = v; },
  get activeFileName(): string | null {
    if (!activeFilePath) return null;
    return activeFilePath.split("/").pop() ?? null;
  },
  get fileCount(): number {
    function count(entries: FileEntry[]): number {
      return entries.reduce((acc, e) => {
        if (e.fileType === "file") return acc + 1;
        return acc + count(e.children ?? []);
      }, 0);
    }
    return count(entries);
  },
};
