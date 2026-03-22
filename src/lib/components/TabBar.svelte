<script lang="ts">
  import { editor } from "$lib/stores/editor.svelte";

  let { onSettingsClick }: {
    onSettingsClick?: () => void;
  } = $props();
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
    {#if onSettingsClick}
      <button
        class="settings-btn"
        onclick={onSettingsClick}
        aria-label="Settings"
        title="Settings (Ctrl+,)"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>
    {/if}
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

  .settings-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    padding: 0;
    margin-left: 4px;
    transition: color 0.15s, background 0.15s;
  }

  .settings-btn:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.06);
  }
</style>
