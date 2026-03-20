<script lang="ts">
  import Sidebar from "$lib/components/Sidebar.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import Editor from "$lib/components/Editor.svelte";
  import Preview from "$lib/components/Preview.svelte";
  import { workspace } from "$lib/stores/workspace.svelte";
  import { editor } from "$lib/stores/editor.svelte";
  import { openFile, saveFile } from "$lib/commands/file";
  import { scanDirectory } from "$lib/commands/workspace";
  import CommandPalette from "$lib/components/CommandPalette.svelte";
  import SearchPanel from "$lib/components/SearchPanel.svelte";
  import SettingsDialog from "$lib/components/SettingsDialog.svelte";
  import { renderPreview } from "$lib/commands/preview";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { loadConfig } from "$lib/commands/config";

  let editorComponent = $state<Editor | null>(null);
  let showPalette = $state(false);
  let showSearch = $state(false);
  let showSettings = $state(false);
  let errorMessage = $state<string | null>(null);
  let previousTabId = $state<string | null>(null);

  function showError(msg: string) {
    errorMessage = msg;
    setTimeout(() => { errorMessage = null; }, 5000);
  }

  async function handleOpenWorkspace() {
    const selected = await open({ directory: true });
    if (selected) {
      workspace.workspaceRoot = selected as string;
      workspace.entries = await scanDirectory(selected as string);
    }
  }

  async function handleFileSelect(path: string) {
    try {
      const fileName = path.split(/[\\/]/).pop() ?? "untitled";

      // If tab already exists, just switch to it
      const existingTab = editor.tabs.find((t) => t.id === path);
      if (existingTab) {
        editor.switchTab(path);
        return;
      }

      const content = await openFile(path);
      editor.openTab(path, fileName, content);
      editorComponent?.setContent(content);
      await updatePreview(content);
    } catch (e) {
      console.error("Failed to open file:", e);
      showError("Failed to open file");
    }
  }

  async function handleSearchSelect(filePath: string, lineNumber: number) {
    await handleFileSelect(filePath);
    // Wait for the editor to mount/update, then jump to line
    requestAnimationFrame(() => {
      editorComponent?.goToLine(lineNumber);
    });
  }

  async function updatePreview(content: string) {
    try {
      const result = await renderPreview(content);
      editor.updateActivePreview(result.html, result.frontmatter);
    } catch (e) {
      console.error("Preview render failed:", e);
      showError("Preview render failed");
    }
  }

  async function handleSave() {
    const activeTab = editor.activeTab;
    if (!activeTab) return;
    try {
      await saveFile(activeTab.filePath, activeTab.content);
      editor.dirty = false;
    } catch (e) {
      console.error("Save failed:", e);
      showError("Save failed");
    }
  }

  // Load saved theme on startup
  onMount(() => {
    loadConfig().then((config) => {
      const savedTheme = (config.theme as "dark" | "light") ?? "dark";
      editor.theme = savedTheme;
      document.documentElement.setAttribute("data-theme", savedTheme);
    });
  });

  // Start filesystem watcher when workspace opens
  $effect(() => {
    const root = workspace.workspaceRoot;
    if (!root) return;

    let unlisten: (() => void) | undefined;

    (async () => {
      await invoke("watch_workspace", { path: root });
      unlisten = await listen("file-changed", async () => {
        workspace.entries = await scanDirectory(root);
      });
    })();

    return () => unlisten?.();
  });

  // Watch for active tab changes — save old tab state and restore new tab content
  $effect(() => {
    const currentTabId = editor.activeTabId;

    if (currentTabId !== previousTabId) {
      // Save scroll position from old tab
      if (previousTabId && editorComponent?.getView()) {
        const view = editorComponent.getView();
        if (view) {
          const oldTab = editor.tabs.find((t) => t.id === previousTabId);
          if (oldTab) {
            oldTab.scrollTop = view.scrollDOM.scrollTop;
            oldTab.cursorPos = view.state.selection.main.head;
          }
        }
      }

      // Load content for new tab
      const newTab = editor.activeTab;
      if (newTab && editorComponent) {
        editorComponent.setContent(newTab.content);
        // Restore cursor and scroll after content is set
        requestAnimationFrame(() => {
          editorComponent?.restoreState(newTab.cursorPos, newTab.scrollTop);
        });
      }

      previousTabId = currentTabId;
    }
  });

  // Debounced preview updates on content change
  $effect(() => {
    const content = editor.content;
    if (!editor.activeTabId) return;
    const timeout = setTimeout(() => updatePreview(content), 300);
    return () => clearTimeout(timeout);
  });

  // Global keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === "s") { e.preventDefault(); handleSave(); }
    if (mod && e.key === "o") { e.preventDefault(); handleOpenWorkspace(); }
    if (mod && e.shiftKey && (e.key === "P" || e.key === "p")) {
      e.preventDefault();
      editor.viewMode = editor.viewMode === "edit" ? "preview" : "edit";
    }
    if (mod && e.key === "\\") {
      e.preventDefault();
      const cycle: Record<string, "split" | "preview" | "edit"> = { edit: "split", split: "preview", preview: "edit" };
      editor.viewMode = cycle[editor.viewMode];
    }
    if (mod && e.shiftKey && (e.key === "F" || e.key === "f")) {
      e.preventDefault();
      showSearch = !showSearch;
    }
    if (mod && e.key === "k") { e.preventDefault(); showPalette = !showPalette; }
    if (mod && e.key === ",") { e.preventDefault(); showSettings = !showSettings; }
    if (mod && e.shiftKey && (e.key === "T" || e.key === "t")) {
      e.preventDefault();
      const newTheme = editor.theme === "dark" ? "light" : "dark";
      editor.theme = newTheme;
      document.documentElement.setAttribute("data-theme", newTheme);
    }
    if (mod && e.shiftKey && (e.key === "E" || e.key === "e")) {
      e.preventDefault();
      workspace.sidebarVisible = !workspace.sidebarVisible;
    }
    if (mod && e.key === "w") {
      e.preventDefault();
      if (editor.activeTabId) editor.closeTab(editor.activeTabId);
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-layout">
  {#if workspace.sidebarVisible}
    <Sidebar onFileSelect={handleFileSelect} />
  {/if}
  <main class="main-area">
    <TabBar />
    <div class="content-area">
      {#if editor.tabs.length > 0 && editor.activeTabId}
        {#if editor.viewMode === "edit"}
          <Editor bind:this={editorComponent} />
        {:else if editor.viewMode === "preview"}
          <Preview />
        {:else}
          <div class="split-view" class:vertical={editor.splitDirection === "vertical"}>
            <div class="split-pane"><Editor bind:this={editorComponent} /></div>
            <div class="split-divider"></div>
            <div class="split-pane"><Preview /></div>
          </div>
        {/if}
      {:else}
        <div class="placeholder">
          <div class="placeholder-content">
            <p>Open a workspace to start editing</p>
            <button class="open-btn" onclick={handleOpenWorkspace}>
              Open Workspace
            </button>
            <span class="shortcut">Ctrl+O</span>
          </div>
        </div>
      {/if}
    </div>
    <StatusBar />
  </main>
  {#if errorMessage}
    <div class="toast-error" role="alert">
      <span>{errorMessage}</span>
      <button class="toast-close" onclick={() => errorMessage = null} aria-label="Dismiss">&times;</button>
    </div>
  {/if}
</div>

<CommandPalette bind:visible={showPalette} onSelect={handleFileSelect} />
<SearchPanel bind:visible={showSearch} onSelect={handleSearchSelect} />
<SettingsDialog bind:visible={showSettings} />

<style>
  .app-layout { display: flex; height: 100vh; width: 100%; }
  .main-area { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .content-area { flex: 1; overflow: hidden; display: flex; flex-direction: column; }
  .split-view { display: flex; flex: 1; min-height: 0; }
  .split-view.vertical { flex-direction: column; }
  .split-pane { flex: 1; overflow: hidden; min-width: 0; min-height: 0; }
  .split-divider { flex-shrink: 0; background: var(--border); }
  .split-view:not(.vertical) > .split-divider { width: 1px; }
  .split-view.vertical > .split-divider { height: 1px; }
  .placeholder { display: flex; align-items: center; justify-content: center; height: 100%; }
  .placeholder-content { text-align: center; color: var(--text-dimmed); }
  .open-btn {
    margin-top: 12px; padding: 8px 20px;
    background: var(--accent); color: white; border: none;
    border-radius: 6px; cursor: pointer; font-family: inherit; font-size: 13px;
  }
  .open-btn:hover { background: #3b82f6; }
  .shortcut { display: block; margin-top: 8px; font-size: 11px; font-family: var(--font-mono); color: var(--text-dimmed); }
  .toast-error {
    position: fixed;
    bottom: 40px;
    right: 16px;
    padding: 10px 16px;
    background: var(--surface-elevated);
    border: 1px solid var(--red);
    border-radius: 8px;
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    align-items: center;
    gap: 12px;
    z-index: 200;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  }
  .toast-close {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 18px;
    cursor: pointer;
    padding: 0 4px;
  }
</style>
