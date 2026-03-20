<script lang="ts">
  import { editor } from "$lib/stores/editor.svelte";
</script>

<div class="tabbar">
  <div class="file-tabs">
    {#each editor.tabs as tab}
      <div
        class="file-tab"
        class:active={tab.id === editor.activeTabId}
        role="tab"
        tabindex="0"
        aria-selected={tab.id === editor.activeTabId}
        onclick={() => editor.switchTab(tab.id)}
        onkeydown={(e) => { if (e.key === "Enter") editor.switchTab(tab.id); }}
        onauxclick={(e) => { if (e.button === 1) { e.preventDefault(); editor.closeTab(tab.id); } }}
      >
        {#if tab.dirty}<span class="dirty-dot"></span>{/if}
        <span class="tab-name">{tab.fileName}</span>
        <button
          class="close-btn"
          onclick={(e) => { e.stopPropagation(); editor.closeTab(tab.id); }}
          aria-label="Close tab"
        >&times;</button>
      </div>
    {/each}
  </div>
  <div class="view-toggle">
    <button
      class="view-btn"
      class:active={editor.viewMode === "edit"}
      onclick={() => editor.viewMode = "edit"}
    >Edit</button>
    <button
      class="view-btn"
      class:active={editor.viewMode === "split"}
      onclick={() => editor.viewMode = "split"}
    >Split</button>
    <button
      class="view-btn"
      class:active={editor.viewMode === "preview"}
      onclick={() => editor.viewMode = "preview"}
    >Preview</button>
  </div>
</div>

<style>
  .tabbar {
    height: var(--tabbar-height);
    min-height: var(--tabbar-height);
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: stretch;
    justify-content: space-between;
    user-select: none;
  }

  .file-tabs {
    display: flex;
    overflow-x: auto;
    overflow-y: hidden;
    flex: 1;
    min-width: 0;
    scrollbar-width: none;
  }

  .file-tabs::-webkit-scrollbar {
    display: none;
  }

  .file-tab {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 12px;
    background: transparent;
    border: none;
    border-right: 1px solid var(--border);
    color: var(--text-muted);
    font-family: var(--font-sans);
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
    transition: color 0.15s, background 0.15s;
  }

  .file-tab:hover {
    color: var(--text-secondary);
    background: rgba(56, 139, 253, 0.04);
  }

  .file-tab.active {
    color: var(--text-primary);
    background: var(--surface-elevated);
    border-bottom: 2px solid var(--accent);
  }

  .dirty-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
  }

  .tab-name {
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    background: none;
    border: none;
    color: var(--text-dimmed);
    font-size: 14px;
    cursor: pointer;
    border-radius: 4px;
    padding: 0;
    opacity: 0;
    transition: opacity 0.1s, background 0.1s;
  }

  .file-tab:hover .close-btn,
  .file-tab.active .close-btn {
    opacity: 1;
  }

  .close-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-primary);
  }

  .view-toggle {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 0 12px;
    flex-shrink: 0;
  }

  .view-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-family: var(--font-sans);
    font-size: 12px;
    font-weight: 500;
    padding: 6px 14px;
    cursor: pointer;
    border-radius: 6px 6px 0 0;
    transition: color 0.15s, background 0.15s;
  }

  .view-btn:hover {
    color: var(--text-secondary);
  }

  .view-btn.active {
    color: var(--text-primary);
    background: var(--surface-elevated);
  }
</style>
