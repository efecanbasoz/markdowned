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
</script>

<svelte:window onclick={dismissContextMenu} onkeydown={(e) => { if (e.key === "Escape") dismissContextMenu(); }} />

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
        <div class="workspace-section">
          <button
            class="workspace-root"
            onclick={() => workspace.toggleCollapse(ws.root)}
            oncontextmenu={(e) => handleContextMenu(e, ws.root)}
          >
            <span class="collapse-icon">{ws.collapsed ? "\u25B6" : "\u25BC"}</span>
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
