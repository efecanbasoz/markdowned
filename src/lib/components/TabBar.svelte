<script lang="ts">
  import { editor } from "$lib/stores/editor.svelte";
  import { workspace } from "$lib/stores/workspace.svelte";
</script>

<div class="tabbar">
  <div class="tabs">
    <button
      class="tab"
      class:active={editor.activeTab === "edit"}
      onclick={() => editor.activeTab = "edit"}
    >
      Edit
    </button>
    <button
      class="tab"
      class:active={editor.activeTab === "preview"}
      onclick={() => editor.activeTab = "preview"}
    >
      Preview
    </button>
  </div>
  <div class="file-info">
    {#if workspace.activeFileName}
      {#if editor.dirty}
        <span class="dirty-dot"></span>
      {/if}
      <span class="filename">{workspace.activeFileName}</span>
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
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    user-select: none;
  }

  .tabs {
    display: flex;
    gap: 2px;
  }

  .tab {
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

  .tab:hover {
    color: var(--text-secondary);
  }

  .tab.active {
    color: var(--text-primary);
    background: var(--surface-elevated);
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .filename {
    font-size: 12px;
    color: var(--text-secondary);
    font-family: var(--font-mono);
  }

  .dirty-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
  }
</style>
