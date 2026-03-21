# Multi-Workspace Support — Design Specification

## Summary

Add Zed-style multi-workspace support: users can open multiple folders simultaneously in the sidebar, each with its own collapsible file tree. Workspaces persist across sessions.

## Data Model

### New Type (`src/lib/types.ts`)

```typescript
interface WorkspaceEntry {
  root: string;        // absolute canonical path
  name: string;        // display name (last segment of path)
  entries: FileEntry[]; // scanned file tree
  collapsed: boolean;  // sidebar collapse state (transient, resets to expanded on startup)
}
```

### Config Changes

**TypeScript (`AppConfig` in `types.ts`):**
- Remove: `lastWorkspace: string | null`
- Add: `workspaces: string[]` (array of root paths)

**Rust (`AppConfig` in `models/config.rs`):**
- Keep: `last_workspace: Option<String>` with `#[serde(default)]` for backward compat deserialization
- Add: `workspaces: Vec<String>` with `#[serde(default)]`
- Migration in `AppConfig::load()`: if `last_workspace` is Some and `workspaces` is empty, set `workspaces = vec![last_workspace]`, then save.

Both sides must change in lockstep since config passes through `invoke()`.

## Store Changes (`workspace.svelte.ts`)

### Remove
- `workspaceRoot: string | null`
- `entries: FileEntry[]`

### Add
- `workspaces: WorkspaceEntry[]`

### Methods
- `addWorkspace(root: string)`: scan directory, append to `workspaces`, save config. No-op if already present (compared by canonical path — the path returned by `scan_directory` is already canonicalized by the Rust backend).
- `removeWorkspace(root: string)`: remove from `workspaces`, save config. Does NOT close tabs from that workspace.
- `toggleCollapse(root: string)`: toggle `collapsed` flag on the matching workspace.
- `fileCount` (computed): sum of files across all workspaces.
- `sidebarVisible`: unchanged.

### Invariants
- Duplicate roots are rejected (compared by canonical path from backend).
- Order is preserved (insertion order).
- Collapse state is transient — resets to expanded on app startup.
- No hard workspace limit. Performance degrades naturally with many watchers.

## Rust Backend Changes

### `WatcherState` refactor (`services/watcher.rs`)

Current: holds `Mutex<Option<WatcherHandle>>` — single watcher, replaced on each call.

New: holds `Mutex<HashMap<String, WatcherHandle>>` keyed by canonical workspace root.

- `start_watcher(app, workspace_path, watcher_state)`: inserts a new watcher for the path. Does not drop other watchers.
- New `stop_watcher(workspace_path, watcher_state)`: removes and drops the watcher for the given path.

### `FileChangeEvent` identity (`services/watcher.rs`)

Current: emits `{ kind, path }` — no workspace identity.

New: add `workspace_root: String` field to `FileChangeEvent`. The watcher thread captures the workspace root and includes it in every emitted event. Frontend uses this to rescan only the affected workspace.

```rust
pub struct FileChangeEvent {
    pub kind: String,
    pub path: String,
    pub workspace_root: String,  // NEW
}
```

### New command: `unwatch_workspace` (`commands/workspace.rs`)

```rust
#[tauri::command]
pub async fn unwatch_workspace(
    watcher_state: tauri::State<'_, watcher::WatcherState>,
    path: String,
) -> Result<(), String> {
    watcher::stop_watcher(path, &watcher_state)
}
```

Register in `lib.rs` invoke_handler.

### `WorkspaceState` refactor (`lib.rs`)

Current: `root: Arc<Mutex<Option<String>>>` — single root.

New: `roots: Arc<Mutex<Vec<String>>>` — multiple roots.

- `scan_directory`: appends the canonical path to `roots` (if not already present) instead of replacing.
- `search_workspace`: iterates all roots and merges results (cap at MAX_SEARCH_RESULTS across all).

### `search_workspace` multi-root (`commands/workspace.rs`)

```rust
#[tauri::command]
pub async fn search_workspace(
    state: tauri::State<'_, crate::WorkspaceState>,
    query: String,
) -> Result<Vec<SearchMatch>, String> {
    let roots = state.roots.lock()
        .map_err(|e| format!("State error: {e}"))?
        .clone();
    if roots.is_empty() {
        return Err("No workspace open".to_string());
    }
    tokio::task::spawn_blocking(move || {
        let mut all_results = Vec::new();
        for root in &roots {
            let mut results = search_workspace_impl(root, &query)?;
            all_results.append(&mut results);
            if all_results.len() >= MAX_SEARCH_RESULTS {
                all_results.truncate(MAX_SEARCH_RESULTS);
                break;
            }
        }
        Ok(all_results)
    })
    .await
    .map_err(|e| format!("Task failed: {e}")?
}
```

## Sidebar UI Changes

### Layout
```
┌─ EXPLORER ──────── [+] ─┐
│ ▼ project-a              │  ← click name to collapse, right-click for context menu
│   ├── src/                │
│   └── README.md           │
│ ► project-b               │  ← collapsed
│                           │
│ 12 files                  │
└───────────────────────────┘
```

### Components

**Sidebar.svelte**:
- Header gains a [+] button that opens the directory picker.
- Iterates `workspace.workspaces` and renders a section per workspace.
- Each section: clickable workspace name (collapse toggle), file tree (when expanded).

**Context menu**:
- Right-click on workspace root name shows a custom `<div>` menu with "Remove Workspace".
- Positioned at cursor (`clientX/clientY`). Dismissed on click-outside or Escape.

### Keyboard Shortcut
- `Ctrl+O` opens directory picker to add a new workspace (replaces current "open workspace" behavior).
- When no workspaces are open, the placeholder with "Open Workspace" button remains.

## +page.svelte Changes

- `handleOpenWorkspace` → `handleAddWorkspace`: adds to workspace list instead of replacing.
- File watcher `$effect`: one watcher per workspace. Listen for `file-changed` events, check `workspace_root` field to rescan only that workspace.
- On `removeWorkspace`: call `unwatch_workspace` to clean up the watcher.
- Placeholder shown when `workspace.workspaces.length === 0`.

## Tab Behavior

No changes. Tabs are identified by file path (unique across all workspaces). Files from different workspaces appear in the same tab bar.

## Config Persistence

On every `addWorkspace`/`removeWorkspace`, save the current workspace root list to config.toml via `saveConfig`. On app startup, `loadConfig` reads `workspaces`, scans each directory, populates the store.

## Edge Cases

- **Same folder added twice**: rejected silently (already present, compared by canonical path).
- **Folder deleted externally**: file tree shows empty; watcher may emit errors — handle gracefully (show empty tree, don't crash).
- **All workspaces removed**: show placeholder, same as fresh start.
- **Tab from removed workspace**: tab stays open (file path still valid). Re-adding the same workspace later doesn't affect existing tabs.

## Testing

### Unit Tests — TypeScript (TDD)
- `addWorkspace`: adds entry, rejects duplicate, preserves order
- `removeWorkspace`: removes correct entry, no-op for unknown root
- `toggleCollapse`: toggles collapsed state
- `fileCount`: sums across multiple workspaces
- Config migration: `lastWorkspace` → `workspaces` array

### Rust Tests
- Config migration: `last_workspace` present + empty `workspaces` → migrated
- `search_workspace_impl`: merges results from multiple roots
- Watcher: multiple watchers coexist in HashMap

### Manual Testing
- Add 2+ workspaces, collapse/expand each
- Right-click → Remove Workspace
- Close and reopen app → workspaces restored
- Open files from different workspaces → tabs work correctly
- Search across multiple workspaces → results from both
- Resize sidebar divider → still works with multiple workspaces

## Files Changed

| File | Change |
|------|--------|
| `src/lib/types.ts` | Add `WorkspaceEntry`, update `AppConfig` |
| `src/lib/stores/workspace.svelte.ts` | Multi-workspace state + methods |
| `src/lib/components/Sidebar.svelte` | Multi-workspace sections, [+] button, context menu |
| `src/routes/+page.svelte` | Multi-watcher, updated add/remove logic |
| `src/lib/commands/config.ts` | Migration logic, workspaces array support |
| `src-tauri/src/lib.rs` | `WorkspaceState` refactor (multi-root), register `unwatch_workspace` |
| `src-tauri/src/services/watcher.rs` | Multi-watcher HashMap, `stop_watcher`, `workspace_root` in events |
| `src-tauri/src/models/config.rs` | Add `workspaces: Vec<String>`, migration in `load()` |
| `src-tauri/src/commands/workspace.rs` | Multi-root search, `unwatch_workspace` command, remove single-root mutation |
| `test/workspace.test.ts` | New: TDD tests for workspace store |
