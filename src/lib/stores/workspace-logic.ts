import type { FileEntry, WorkspaceEntry } from "../types";

function countFiles(entries: FileEntry[]): number {
  return entries.reduce((acc, e) => {
    if (e.fileType === "file") return acc + 1;
    return acc + countFiles(e.children ?? []);
  }, 0);
}

export function createWorkspaceStore() {
  let workspaces: WorkspaceEntry[] = [];

  return {
    addWorkspace(entry: WorkspaceEntry) {
      if (workspaces.some((w) => w.root === entry.root)) return;
      workspaces = [...workspaces, entry];
    },

    removeWorkspace(root: string) {
      workspaces = workspaces.filter((w) => w.root !== root);
    },

    toggleCollapse(root: string) {
      workspaces = workspaces.map((w) =>
        w.root === root ? { ...w, collapsed: !w.collapsed } : w,
      );
    },

    updateEntries(root: string, entries: FileEntry[]) {
      workspaces = workspaces.map((w) =>
        w.root === root ? { ...w, entries } : w,
      );
    },

    getWorkspaces() {
      return workspaces;
    },

    getFileCount() {
      return workspaces.reduce((acc, w) => acc + countFiles(w.entries), 0);
    },
  };
}
