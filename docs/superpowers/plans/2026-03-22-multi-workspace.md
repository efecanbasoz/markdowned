# Multi-Workspace Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Zed-style multi-workspace support — multiple folders in sidebar, context menu removal, config persistence, multi-watcher backend.

**Architecture:** Transform single-workspace model to multi-workspace array. Rust backend gets multi-watcher HashMap, workspace-scoped events, and multi-root search. Frontend store becomes array-based with add/remove/collapse methods. Sidebar renders collapsible sections per workspace with context menu.

**Tech Stack:** Svelte 5 (runes), Tauri 2, Rust, TypeScript, Vitest

**Spec:** `docs/superpowers/specs/2026-03-22-multi-workspace-design.md`

**Note on serde naming:** The Rust `FileChangeEvent` struct has `#[serde(rename_all = "camelCase")]`, so `workspace_root` serializes to `workspaceRoot` in JSON. Frontend must use `workspaceRoot`.

**Note on config field naming:** The existing Rust `AppConfig` does NOT have `rename_all = "camelCase"`, so fields are serialized as snake_case. The TypeScript `AppConfig` type uses camelCase. This is a pre-existing mismatch that Tauri handles internally. Maintain the existing pattern: Rust uses snake_case, TypeScript uses camelCase.

---

## File Structure

| File | Role |
|------|------|
| `src/lib/types.ts` | Add `WorkspaceEntry` type, update `AppConfig` |
| `src/lib/stores/workspace-logic.ts` | Pure workspace logic (testable without Svelte) |
| `src/lib/stores/workspace.svelte.ts` | Reactive Svelte store wrapping workspace-logic |
| `src/lib/components/Sidebar.svelte` | Multi-workspace sections, [+] button, context menu |
| `src/routes/+page.svelte` | Multi-watcher, add/remove workspace orchestration |
| `src/lib/commands/workspace.ts` | Add `unwatchWorkspace` command binding |
| `src-tauri/src/lib.rs` | `WorkspaceState` multi-root, register commands |
| `src-tauri/src/services/watcher.rs` | Multi-watcher HashMap, `stop_watcher`, `workspace_root` in events |
| `src-tauri/src/models/config.rs` | Add `workspaces` field, migration in `load()` |
| `src-tauri/src/commands/workspace.rs` | Multi-root search, `unwatch_workspace` + root removal |
| `test/workspace.test.ts` | TDD tests for workspace store logic |

---

### Task 1: Rust Backend — Config Model + Migration

**Files:**
- Modify: `src-tauri/src/models/config.rs`

- [ ] **Step 1: Add `workspaces` field to Rust `AppConfig`**

In `src-tauri/src/models/config.rs`, add to the `AppConfig` struct after `last_workspace`:

```rust
#[serde(default)]
pub workspaces: Vec<String>,
```

And update `Default for AppConfig`:

```rust
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            completion: CompletionConfig::default(),
            last_workspace: None,
            workspaces: Vec::new(),
            split_direction: default_split_direction(),
            theme: default_theme(),
        }
    }
}
```

- [ ] **Step 2: Add migration logic in `AppConfig::load()`**

After `toml::from_str` succeeds, add migration before returning:

```rust
Ok(mut config) => {
    // Migrate last_workspace → workspaces
    if let Some(ref lw) = config.last_workspace {
        if config.workspaces.is_empty() {
            config.workspaces = vec![lw.clone()];
            config.last_workspace = None;
            let _ = config.save();
        }
    }
    config
}
```

- [ ] **Step 3: Verify it compiles**

Run: `cd src-tauri && cargo check 2>&1 | tail -5`

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models/config.rs
git commit -m "feat(backend): add workspaces field to config with migration"
```

---

### Task 2: Rust Backend — Multi-Watcher

**Files:**
- Modify: `src-tauri/src/services/watcher.rs`

- [ ] **Step 1: Change `WatcherState` to HashMap and add `workspace_root` to event**

Replace the full file with:

```rust
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChangeEvent {
    pub kind: String,
    pub path: String,
    pub workspace_root: String,
}

struct WatcherHandle {
    _watcher: RecommendedWatcher,
}

pub struct WatcherState {
    handles: Mutex<HashMap<String, WatcherHandle>>,
}

impl Default for WatcherState {
    fn default() -> Self {
        Self { handles: Mutex::new(HashMap::new()) }
    }
}

pub fn start_watcher(app: AppHandle, workspace_path: String, watcher_state: &WatcherState) -> Result<(), String> {
    let canonical = std::fs::canonicalize(&workspace_path)
        .map_err(|e| format!("Invalid path: {e}"))?
        .to_string_lossy()
        .to_string();

    // Remove existing watcher for this path if any
    {
        let mut guard = watcher_state.handles.lock().map_err(|e| format!("Watcher state error: {e}"))?;
        guard.remove(&canonical);
    }

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| format!("Failed to create watcher: {e}"))?;

    watcher
        .watch(Path::new(&canonical), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {e}"))?;

    {
        let mut guard = watcher_state.handles.lock().map_err(|e| format!("Watcher state error: {e}"))?;
        guard.insert(canonical.clone(), WatcherHandle { _watcher: watcher });
    }

    let ws_root = canonical;
    std::thread::spawn(move || {
        use std::time::{Duration, Instant};
        let debounce_ms = Duration::from_millis(200);
        let mut last_emit = Instant::now().checked_sub(debounce_ms).unwrap_or_else(Instant::now);

        while let Ok(event) = rx.recv() {
            if let Ok(Event { kind, paths, .. }) = event {
                let kind_str = match kind {
                    EventKind::Create(_) => "created",
                    EventKind::Modify(_) => "modified",
                    EventKind::Remove(_) => "deleted",
                    _ => continue,
                };

                let now = Instant::now();
                if now.duration_since(last_emit) < debounce_ms {
                    continue;
                }
                last_emit = now;

                for path in paths {
                    let _ = app.emit(
                        "file-changed",
                        FileChangeEvent {
                            kind: kind_str.to_string(),
                            path: path.to_string_lossy().to_string(),
                            workspace_root: ws_root.clone(),
                        },
                    );
                }
            }
        }
    });

    Ok(())
}

pub fn stop_watcher(workspace_path: String, watcher_state: &WatcherState) -> Result<(), String> {
    let canonical = std::fs::canonicalize(&workspace_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(&workspace_path))
        .to_string_lossy()
        .to_string();
    let mut guard = watcher_state.handles.lock().map_err(|e| format!("Watcher state error: {e}"))?;
    guard.remove(&canonical);
    Ok(())
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cd src-tauri && cargo check 2>&1 | tail -5`

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/services/watcher.rs
git commit -m "feat(backend): multi-watcher HashMap with workspace_root in events"
```

---

### Task 3: Rust Backend — WorkspaceState Multi-Root + Unwatch Command

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/src/commands/workspace.rs`

- [ ] **Step 1: Refactor `WorkspaceState` to multi-root**

In `src-tauri/src/lib.rs`, replace:

```rust
pub struct WorkspaceState {
    pub root: Arc<Mutex<Option<String>>>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self { root: Arc::new(Mutex::new(None)) }
    }
}
```

With:

```rust
pub struct WorkspaceState {
    pub roots: Arc<Mutex<Vec<String>>>,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        Self { roots: Arc::new(Mutex::new(Vec::new())) }
    }
}
```

- [ ] **Step 2: Register `unwatch_workspace` command**

Add `commands::workspace::unwatch_workspace` to the `invoke_handler` list.

- [ ] **Step 3: Update `scan_directory` to append root**

In `src-tauri/src/commands/workspace.rs`, replace the single-root mutation block (lines 100-104):

```rust
// Old: *root = Some(canonical_str.clone());
```

With:

```rust
{
    let mut roots = state.roots.lock().map_err(|e| format!("State error: {e}"))?;
    if !roots.contains(&canonical_str) {
        roots.push(canonical_str.clone());
    }
}
```

- [ ] **Step 4: Update `search_workspace` for multi-root**

Replace the existing `search_workspace` function:

```rust
#[tauri::command]
pub async fn search_workspace(
    state: tauri::State<'_, crate::WorkspaceState>,
    query: String,
) -> Result<Vec<SearchMatch>, String> {
    let roots = state
        .roots
        .lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    if roots.is_empty() {
        return Err("No workspace open".to_string());
    }
    tokio::task::spawn_blocking(move || {
        let mut all_results = Vec::new();
        for root in &roots {
            match search_workspace_impl(root, &query) {
                Ok(mut results) => {
                    all_results.append(&mut results);
                    if all_results.len() >= MAX_SEARCH_RESULTS {
                        all_results.truncate(MAX_SEARCH_RESULTS);
                        break;
                    }
                }
                Err(_) => continue, // Skip roots that fail (e.g., deleted folder)
            }
        }
        Ok(all_results)
    })
    .await
    .map_err(|e| format!("Task failed: {e}"))?
}
```

- [ ] **Step 5: Add `unwatch_workspace` command (stops watcher AND removes root)**

```rust
#[tauri::command]
pub async fn unwatch_workspace(
    state: tauri::State<'_, crate::WorkspaceState>,
    watcher_state: tauri::State<'_, watcher::WatcherState>,
    path: String,
) -> Result<(), String> {
    // Remove from workspace roots
    let canonical = std::fs::canonicalize(&path)
        .unwrap_or_else(|_| std::path::PathBuf::from(&path))
        .to_string_lossy()
        .to_string();
    {
        let mut roots = state.roots.lock().map_err(|e| format!("State error: {e}"))?;
        roots.retain(|r| r != &canonical);
    }
    // Stop watcher
    watcher::stop_watcher(path, &watcher_state)
}
```

- [ ] **Step 6: Verify it compiles**

Run: `cd src-tauri && cargo check 2>&1 | tail -5`

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/commands/workspace.rs
git commit -m "feat(backend): multi-root workspace state, unwatch command, multi-root search"
```

---

### Task 4: TypeScript Types + Command Binding

**Files:**
- Modify: `src/lib/types.ts`
- Modify: `src/lib/commands/workspace.ts`

- [ ] **Step 1: Add `WorkspaceEntry` type and update `AppConfig`**

In `src/lib/types.ts`, add after `SearchMatch`:

```typescript
export interface WorkspaceEntry {
  root: string;
  name: string;
  entries: FileEntry[];
  collapsed: boolean;
}
```

Update `AppConfig` — replace `lastWorkspace: string | null;` with `workspaces?: string[];`:

```typescript
export interface AppConfig {
  completion: CompletionConfig;
  workspaces?: string[];
  splitDirection?: SplitDirection;
  theme?: "dark" | "light";
}
```

- [ ] **Step 2: Add `unwatchWorkspace` command binding**

In `src/lib/commands/workspace.ts`, add:

```typescript
export async function unwatchWorkspace(path: string): Promise<void> {
  return invoke<void>("unwatch_workspace", { path });
}
```

- [ ] **Step 3: Commit (TypeScript won't fully compile yet — store/page still reference old API)**

```bash
git add src/lib/types.ts src/lib/commands/workspace.ts
git commit -m "feat(types): add WorkspaceEntry, update AppConfig, add unwatchWorkspace"
```

---

### Task 5: TDD — Workspace Store Logic

**Files:**
- Create: `test/workspace.test.ts`
- Create: `src/lib/stores/workspace-logic.ts`

- [ ] **Step 1: Write failing tests**

Create `test/workspace.test.ts`:

```typescript
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
```

- [ ] **Step 2: Run tests — verify RED**

Run: `npx vitest run test/workspace.test.ts 2>&1`
Expected: FAIL — module `workspace-logic` not found

- [ ] **Step 3: Create `workspace-logic.ts`**

Create `src/lib/stores/workspace-logic.ts`:

```typescript
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
```

- [ ] **Step 4: Run tests — verify GREEN**

Run: `npx vitest run test/workspace.test.ts 2>&1`
Expected: all 9 tests PASS

- [ ] **Step 5: Commit**

```bash
git add test/workspace.test.ts src/lib/stores/workspace-logic.ts
git commit -m "feat(store): TDD workspace store logic with tests"
```

---

### Task 6: Frontend — Store + Sidebar + Page (single atomic commit)

These three files must change together since they reference each other's API. Committing separately would break compilation.

**Files:**
- Rewrite: `src/lib/stores/workspace.svelte.ts`
- Rewrite: `src/lib/components/Sidebar.svelte`
- Modify: `src/routes/+page.svelte`

- [ ] **Step 1: Rewrite workspace Svelte store**

Replace `src/lib/stores/workspace.svelte.ts`:

```typescript
import type { FileEntry, WorkspaceEntry } from "../types";

function countFiles(entries: FileEntry[]): number {
  return entries.reduce((acc, e) => {
    if (e.fileType === "file") return acc + 1;
    return acc + countFiles(e.children ?? []);
  }, 0);
}

let workspaces = $state<WorkspaceEntry[]>([]);
let sidebarVisible = $state(true);

export const workspace = {
  get workspaces() { return workspaces; },
  set workspaces(v: WorkspaceEntry[]) { workspaces = v; },
  get sidebarVisible() { return sidebarVisible; },
  set sidebarVisible(v: boolean) { sidebarVisible = v; },
  get fileCount(): number {
    return workspaces.reduce((acc, w) => acc + countFiles(w.entries), 0);
  },

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
};
```

- [ ] **Step 2: Rewrite Sidebar**

Replace `src/lib/components/Sidebar.svelte`:

```svelte
<script lang="ts">
  import { workspace } from "$lib/stores/workspace.svelte";
  import FileTree from "./FileTree.svelte";

  let { onFileSelect, onAddWorkspace, onRemoveWorkspace }: {
    onFileSelect: (path: string) => void;
    onAddWorkspace: () => void;
    onRemoveWorkspace: (root: string) => void;
  } = $props();

  let contextMenu = $state<{ x: number; y: number; root: string } | null>(null);

  function handleContextMenu(e: MouseEvent, root: string) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, root };
  }

  function handleRemove() {
    if (contextMenu) {
      onRemoveWorkspace(contextMenu.root);
      contextMenu = null;
    }
  }

  function dismissContextMenu() {
    contextMenu = null;
  }

  function handleContextMenuKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") dismissContextMenu();
  }
</script>

<svelte:window onclick={dismissContextMenu} onkeydown={handleContextMenuKeydown} />

<aside class="sidebar">
  <header class="sidebar-header">
    <span class="label">EXPLORER</span>
    <button class="add-btn" onclick={onAddWorkspace} aria-label="Add workspace" title="Add Workspace (Ctrl+O)">
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="12" y1="5" x2="12" y2="19" />
        <line x1="5" y1="12" x2="19" y2="12" />
      </svg>
    </button>
  </header>
  <div class="sidebar-content">
    {#if workspace.workspaces.length > 0}
      {#each workspace.workspaces as ws}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="workspace-section">
          <button
            class="workspace-root"
            onclick={() => workspace.toggleCollapse(ws.root)}
            oncontextmenu={(e) => handleContextMenu(e, ws.root)}
          >
            <span class="collapse-icon">{ws.collapsed ? "►" : "▼"}</span>
            <span class="workspace-name">{ws.name}</span>
          </button>
          {#if !ws.collapsed}
            <div class="workspace-tree">
              <FileTree entries={ws.entries} {onFileSelect} />
            </div>
          {/if}
        </div>
      {/each}
    {:else}
      <p class="placeholder-text">Open a workspace to begin</p>
    {/if}
  </div>
  <footer class="sidebar-footer">
    <span class="file-count">{workspace.fileCount} files</span>
  </footer>
</aside>

{#if contextMenu}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="context-menu"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    onclick|stopPropagation={() => {}}
  >
    <button class="context-menu-item" onclick={handleRemove}>Remove Workspace</button>
  </div>
{/if}

<style>
  .sidebar {
    width: 100%;
    min-width: 0;
    background: var(--surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    height: 100%;
    user-select: none;
  }

  .sidebar-header {
    padding: 10px 12px 6px;
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    color: var(--text-dimmed);
  }

  .add-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 22px;
    height: 22px;
    background: none;
    border: none;
    color: var(--text-dimmed);
    cursor: pointer;
    border-radius: 4px;
    padding: 0;
  }

  .add-btn:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.06);
  }

  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .workspace-section {
    margin-bottom: 2px;
  }

  .workspace-root {
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    padding: 4px 12px;
    background: none;
    border: none;
    color: var(--text-secondary);
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
    cursor: pointer;
    text-align: left;
    font-family: inherit;
  }

  .workspace-root:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.03);
  }

  .collapse-icon {
    font-size: 8px;
    width: 12px;
    flex-shrink: 0;
    color: var(--text-dimmed);
  }

  .workspace-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .workspace-tree {
    padding: 0 12px;
  }

  .placeholder-text {
    color: var(--text-dimmed);
    font-size: 12px;
    text-align: center;
    margin-top: 24px;
  }

  .sidebar-footer {
    padding: 6px 12px;
    border-top: 1px solid var(--border);
  }

  .file-count {
    font-size: 11px;
    color: var(--text-dimmed);
    font-family: var(--font-mono);
  }

  .context-menu {
    position: fixed;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    padding: 4px;
    z-index: 300;
    min-width: 160px;
  }

  .context-menu-item {
    display: block;
    width: 100%;
    padding: 6px 12px;
    background: none;
    border: none;
    color: var(--text-primary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    border-radius: 4px;
  }

  .context-menu-item:hover {
    background: rgba(37, 99, 235, 0.1);
    color: var(--accent-light);
  }
</style>
```

- [ ] **Step 3: Update `+page.svelte`**

Replace the `<script>` block. Key changes:

1. Import `unwatchWorkspace` from commands
2. Replace `handleOpenWorkspace` with `handleAddWorkspace`:
   ```typescript
   async function handleAddWorkspace() {
     const selected = await open({ directory: true });
     if (typeof selected === "string") {
       try {
         const entries = await scanDirectory(selected);
         const name = selected.split("/").pop() ?? "Workspace";
         workspace.addWorkspace({ root: selected, name, entries, collapsed: false });
         await invoke("watch_workspace", { path: selected });
         await saveCurrentWorkspaces();
       } catch (e) {
         console.error("Failed to add workspace:", e);
         showError("Failed to add workspace");
       }
     }
   }
   ```

3. Add `handleRemoveWorkspace`:
   ```typescript
   async function handleRemoveWorkspace(root: string) {
     try {
       await unwatchWorkspace(root);
     } catch { /* watcher may already be stopped */ }
     workspace.removeWorkspace(root);
     await saveCurrentWorkspaces();
   }
   ```

4. Add `saveCurrentWorkspaces` helper:
   ```typescript
   async function saveCurrentWorkspaces() {
     try {
       const config = await loadConfig();
       config.workspaces = workspace.workspaces.map((w) => w.root);
       await saveConfig(config);
     } catch (e) {
       console.error("Failed to save workspaces:", e);
       showError("Failed to save workspace list");
     }
   }
   ```

5. Update `onMount` to load saved workspaces:
   ```typescript
   onMount(() => {
     loadConfig().then(async (config) => {
       // Theme
       const savedTheme = config.theme === "light" ? "light" : "dark";
       editor.theme = savedTheme;
       document.documentElement.setAttribute("data-theme", savedTheme);
       // Restore workspaces
       const roots = config.workspaces ?? [];
       for (const root of roots) {
         try {
           const entries = await scanDirectory(root);
           const name = root.split("/").pop() ?? "Workspace";
           workspace.addWorkspace({ root, name, entries, collapsed: false });
           await invoke("watch_workspace", { path: root });
         } catch (e) {
           console.error(`Failed to restore workspace ${root}:`, e);
         }
       }
     });
   });
   ```

6. Update file watcher `$effect` — listen for `file-changed`, use `workspaceRoot` field:
   ```typescript
   $effect(() => {
     const roots = workspace.workspaces.map((w) => w.root);
     if (roots.length === 0) return;

     let unlisten: (() => void) | undefined;

     (async () => {
       unlisten = await listen<{ workspaceRoot: string }>("file-changed", async (event) => {
         const changedRoot = event.payload.workspaceRoot;
         try {
           const entries = await scanDirectory(changedRoot);
           workspace.updateEntries(changedRoot, entries);
         } catch {
           // Folder may have been deleted
         }
       });
     })();

     return () => unlisten?.();
   });
   ```

7. Remove old `workspace.workspaceRoot` and `workspace.entries` references.

8. Update placeholder condition: `workspace.workspaces.length === 0` instead of `!editor.tabs.length`.

9. Pass props to Sidebar:
   ```svelte
   <Sidebar onFileSelect={handleFileSelect} onAddWorkspace={handleAddWorkspace} onRemoveWorkspace={handleRemoveWorkspace} />
   ```

10. Update `Ctrl+O` handler to call `handleAddWorkspace` instead of `handleOpenWorkspace`.

- [ ] **Step 4: Verify TypeScript compiles**

Run: `npx svelte-check --tsconfig ./tsconfig.json 2>&1 | tail -5`
Expected: 0 errors

- [ ] **Step 5: Run all tests**

Run: `npx vitest run 2>&1`
Expected: all tests PASS

- [ ] **Step 6: Commit**

```bash
git add src/lib/stores/workspace.svelte.ts src/lib/components/Sidebar.svelte src/routes/+page.svelte
git commit -m "feat: multi-workspace frontend — store, sidebar, page orchestration"
```

---

### Task 7: Full Build Verification

**Files:** None (verification only)

- [ ] **Step 1: Run all unit tests**

Run: `npx vitest run 2>&1`
Expected: all tests PASS

- [ ] **Step 2: Run TypeScript check**

Run: `npx svelte-check --tsconfig ./tsconfig.json 2>&1 | tail -5`
Expected: 0 errors

- [ ] **Step 3: Run Rust check**

Run: `cd src-tauri && cargo check 2>&1 | tail -5`
Expected: no errors

- [ ] **Step 4: Run full Tauri build**

Run: `export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig && npx tauri build --bundles deb 2>&1 | tail -10`
Expected: `.deb` bundle created successfully

- [ ] **Step 5: Install and manual test**

Run: `sudo dpkg -i src-tauri/target/release/bundle/deb/markdowned_*.deb`

Manual verification checklist:
- [ ] Open app → placeholder shown
- [ ] Ctrl+O → add first workspace → file tree appears
- [ ] Ctrl+O → add second workspace → both appear in sidebar
- [ ] Click workspace name → collapse/expand
- [ ] Right-click workspace name → "Remove Workspace" → workspace removed
- [ ] Open files from different workspaces → tabs work
- [ ] Ctrl+Shift+F → search → results from all workspaces
- [ ] Close and reopen app → workspaces restored
- [ ] Resize sidebar divider → still works with multiple workspaces
