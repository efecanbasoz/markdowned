import type { FileEntry } from "../types";

let entries = $state<FileEntry[]>([]);
let workspaceRoot = $state<string | null>(null);
let sidebarVisible = $state(true);

export const workspace = {
  get entries() { return entries; },
  set entries(v: FileEntry[]) { entries = v; },
  get workspaceRoot() { return workspaceRoot; },
  set workspaceRoot(v: string | null) { workspaceRoot = v; },
  get sidebarVisible() { return sidebarVisible; },
  set sidebarVisible(v: boolean) { sidebarVisible = v; },
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
