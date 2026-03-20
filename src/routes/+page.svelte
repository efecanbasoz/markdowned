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
  import { renderPreview } from "$lib/commands/preview";
  import { open } from "@tauri-apps/plugin-dialog";

  let editorComponent: Editor;
  let previewTimeout: ReturnType<typeof setTimeout>;

  async function handleOpenWorkspace() {
    const selected = await open({ directory: true });
    if (selected) {
      workspace.workspaceRoot = selected as string;
      workspace.entries = await scanDirectory(selected as string);
    }
  }

  async function handleFileSelect(path: string) {
    try {
      const content = await openFile(path);
      editor.content = content;
      editor.dirty = false;
      editor.fileSize = formatSize(new Blob([content]).size);
      editorComponent?.setContent(content);
      await updatePreview(content);
    } catch (e) {
      console.error("Failed to open file:", e);
    }
  }

  async function updatePreview(content: string) {
    try {
      const result = await renderPreview(content);
      editor.previewHtml = result.html;
      editor.frontmatter = result.frontmatter;
    } catch (e) {
      console.error("Preview render failed:", e);
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    return `${(bytes / 1024).toFixed(1)} KB`;
  }

  async function handleSave() {
    if (!workspace.activeFilePath) return;
    try {
      await saveFile(workspace.activeFilePath, editor.content);
      editor.dirty = false;
    } catch (e) {
      console.error("Save failed:", e);
    }
  }

  // Watch for active file changes
  $effect(() => {
    const path = workspace.activeFilePath;
    if (path) handleFileSelect(path);
  });

  // Debounced preview updates on content change
  $effect(() => {
    const content = editor.content;
    clearTimeout(previewTimeout);
    previewTimeout = setTimeout(() => updatePreview(content), 300);
  });

  // Global keyboard shortcuts
  function handleKeydown(e: KeyboardEvent) {
    const mod = e.metaKey || e.ctrlKey;
    if (mod && e.key === "s") { e.preventDefault(); handleSave(); }
    if (mod && e.key === "o") { e.preventDefault(); handleOpenWorkspace(); }
    if (mod && e.key === "p") {
      e.preventDefault();
      editor.activeTab = editor.activeTab === "edit" ? "preview" : "edit";
    }
    if (mod && e.shiftKey && (e.key === "E" || e.key === "e")) {
      e.preventDefault();
      workspace.sidebarVisible = !workspace.sidebarVisible;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-layout">
  {#if workspace.sidebarVisible}
    <Sidebar />
  {/if}
  <main class="main-area">
    <TabBar />
    <div class="content-area">
      {#if workspace.activeFilePath}
        {#if editor.activeTab === "edit"}
          <Editor bind:this={editorComponent} />
        {:else}
          <Preview />
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
</div>

<style>
  .app-layout { display: flex; height: 100vh; width: 100%; }
  .main-area { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .content-area { flex: 1; overflow: hidden; }
  .placeholder { display: flex; align-items: center; justify-content: center; height: 100%; }
  .placeholder-content { text-align: center; color: var(--text-dimmed); }
  .open-btn {
    margin-top: 12px; padding: 8px 20px;
    background: var(--accent); color: white; border: none;
    border-radius: 6px; cursor: pointer; font-family: inherit; font-size: 13px;
  }
  .open-btn:hover { background: #3b82f6; }
  .shortcut { display: block; margin-top: 8px; font-size: 11px; font-family: var(--font-mono); color: var(--text-dimmed); }
</style>
