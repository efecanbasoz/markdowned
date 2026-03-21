import { describe, it, expect } from "vitest";
import { createWorkspaceStore } from "../src/lib/stores/workspace-logic";
import type { WorkspaceEntry } from "../src/lib/types";

function makeEntry(root: string): WorkspaceEntry {
  return {
    root,
    name: root.split("/").pop() ?? root,
    entries: [],
    collapsed: false,
  };
}

describe("workspace store logic", () => {
  describe("addWorkspace", () => {
    it("adds a workspace entry", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/home/user/project-a"));
      expect(store.getWorkspaces()).toHaveLength(1);
      expect(store.getWorkspaces()[0].root).toBe("/home/user/project-a");
    });

    it("rejects duplicate root", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/home/user/project-a"));
      store.addWorkspace(makeEntry("/home/user/project-a"));
      expect(store.getWorkspaces()).toHaveLength(1);
    });

    it("preserves insertion order", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      store.addWorkspace(makeEntry("/b"));
      store.addWorkspace(makeEntry("/c"));
      expect(store.getWorkspaces().map((w) => w.root)).toEqual(["/a", "/b", "/c"]);
    });
  });

  describe("removeWorkspace", () => {
    it("removes the correct workspace", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      store.addWorkspace(makeEntry("/b"));
      store.removeWorkspace("/a");
      expect(store.getWorkspaces()).toHaveLength(1);
      expect(store.getWorkspaces()[0].root).toBe("/b");
    });

    it("no-op for unknown root", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      store.removeWorkspace("/unknown");
      expect(store.getWorkspaces()).toHaveLength(1);
    });
  });

  describe("toggleCollapse", () => {
    it("toggles collapsed state", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      expect(store.getWorkspaces()[0].collapsed).toBe(false);
      store.toggleCollapse("/a");
      expect(store.getWorkspaces()[0].collapsed).toBe(true);
      store.toggleCollapse("/a");
      expect(store.getWorkspaces()[0].collapsed).toBe(false);
    });
  });

  describe("updateEntries", () => {
    it("updates entries for the correct workspace", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      store.addWorkspace(makeEntry("/b"));
      const newEntries = [{ name: "f1.md", path: "/a/f1.md", fileType: "file" as const }];
      store.updateEntries("/a", newEntries);
      expect(store.getWorkspaces()[0].entries).toEqual(newEntries);
      expect(store.getWorkspaces()[1].entries).toEqual([]);
    });

    it("no-op for unknown root", () => {
      const store = createWorkspaceStore();
      store.addWorkspace(makeEntry("/a"));
      store.updateEntries("/unknown", []);
      expect(store.getWorkspaces()).toHaveLength(1);
    });
  });

  describe("fileCount", () => {
    it("sums files across workspaces", () => {
      const store = createWorkspaceStore();
      const entryA = makeEntry("/a");
      entryA.entries = [
        { name: "f1.md", path: "/a/f1.md", fileType: "file" },
        { name: "f2.md", path: "/a/f2.md", fileType: "file" },
      ];
      const entryB = makeEntry("/b");
      entryB.entries = [
        { name: "f3.md", path: "/b/f3.md", fileType: "file" },
      ];
      store.addWorkspace(entryA);
      store.addWorkspace(entryB);
      expect(store.getFileCount()).toBe(3);
    });

    it("counts nested directory files", () => {
      const store = createWorkspaceStore();
      const entry = makeEntry("/a");
      entry.entries = [
        {
          name: "src",
          path: "/a/src",
          fileType: "directory",
          children: [
            { name: "index.ts", path: "/a/src/index.ts", fileType: "file" },
          ],
        },
        { name: "README.md", path: "/a/README.md", fileType: "file" },
      ];
      store.addWorkspace(entry);
      expect(store.getFileCount()).toBe(2);
    });
  });
});
