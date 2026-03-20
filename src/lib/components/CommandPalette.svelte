<script lang="ts">
  import { workspace } from "$lib/stores/workspace.svelte";
  import type { FileEntry } from "$lib/types";

  let { visible = $bindable(false), onSelect }: {
    visible: boolean;
    onSelect: (path: string) => void;
  } = $props();

  let query = $state("");
  let selectedIndex = $state(0);
  let inputEl: HTMLInputElement;

  interface PaletteItem {
    label: string;
    detail: string;
    path: string;
    icon: string;
  }

  function flattenFiles(entries: FileEntry[], prefix = ""): PaletteItem[] {
    const items: PaletteItem[] = [];
    for (const entry of entries) {
      if (entry.fileType === "file") {
        items.push({ label: entry.name, detail: prefix, path: entry.path, icon: "📄" });
      }
      if (entry.children) {
        items.push(...flattenFiles(entry.children, prefix ? `${prefix}/${entry.name}` : entry.name));
      }
    }
    return items;
  }

  const allItems = $derived(flattenFiles(workspace.entries));
  const filteredItems = $derived(
    query.trim() === ""
      ? allItems.slice(0, 10)
      : allItems.filter(item => item.label.toLowerCase().includes(query.toLowerCase())).slice(0, 10)
  );

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") { e.preventDefault(); selectedIndex = Math.min(selectedIndex + 1, filteredItems.length - 1); }
    else if (e.key === "ArrowUp") { e.preventDefault(); selectedIndex = Math.max(selectedIndex - 1, 0); }
    else if (e.key === "Enter") { e.preventDefault(); const item = filteredItems[selectedIndex]; if (item) { onSelect(item.path); close(); } }
    else if (e.key === "Escape") { close(); }
  }

  function close() { visible = false; query = ""; selectedIndex = 0; }

  $effect(() => { if (visible) requestAnimationFrame(() => inputEl?.focus()); });
  $effect(() => { query; selectedIndex = 0; });
</script>

{#if visible}
  <div class="overlay" role="presentation" onclick={close}>
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div class="palette" role="dialog" aria-modal="true" aria-label="File search" onclick={(e) => e.stopPropagation()} onkeydown={handleKeydown}>
      <div class="palette-input">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          bind:this={inputEl}
          bind:value={query}
          type="text"
          placeholder="Search files..."
          spellcheck="false"
          autocomplete="off"
        />
      </div>
      <div class="palette-results" role="listbox">
        {#each filteredItems as item, i}
          <button
            class="palette-item"
            class:selected={i === selectedIndex}
            role="option"
            aria-selected={i === selectedIndex}
            onclick={() => { onSelect(item.path); close(); }}
            onmouseenter={() => selectedIndex = i}
          >
            <span class="item-icon">{item.icon}</span>
            <span class="item-label">{item.label}</span>
            {#if item.detail}
              <span class="item-detail">{item.detail}</span>
            {/if}
          </button>
        {/each}
        {#if filteredItems.length === 0}
          <div class="palette-empty">No files found</div>
        {/if}
      </div>
      <div class="palette-hints">
        <span><kbd>↑↓</kbd> navigate</span>
        <span><kbd>↵</kbd> open</span>
        <span><kbd>esc</kbd> close</span>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    padding-top: 80px;
    z-index: 100;
  }

  .palette {
    width: 500px;
    max-height: 420px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    align-self: flex-start;
  }

  .palette-input {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
    color: var(--text-dimmed);
  }

  .palette-input input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 14px;
    font-family: inherit;
    outline: none;
  }

  .palette-input input::placeholder {
    color: var(--text-dimmed);
  }

  .palette-results {
    flex: 1;
    overflow-y: auto;
    padding: 6px;
  }

  .palette-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    width: 100%;
    border-radius: 6px;
    border: none;
    border-left: 2px solid transparent;
    background: transparent;
    color: var(--text-primary);
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
  }

  .palette-item:hover {
    background: rgba(37, 99, 235, 0.06);
  }

  .palette-item.selected {
    background: linear-gradient(90deg, rgba(37, 99, 235, 0.12) 0%, rgba(37, 99, 235, 0.04) 100%);
    border-left-color: var(--accent);
  }

  .item-icon {
    font-size: 14px;
    flex-shrink: 0;
  }

  .item-label {
    font-weight: 500;
    white-space: nowrap;
  }

  .item-detail {
    color: var(--text-dimmed);
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-left: auto;
  }

  .palette-empty {
    padding: 16px;
    text-align: center;
    color: var(--text-dimmed);
    font-size: 13px;
  }

  .palette-hints {
    display: flex;
    gap: 16px;
    padding: 8px 16px;
    border-top: 1px solid var(--border);
    color: var(--text-dimmed);
    font-size: 11px;
  }

  .palette-hints kbd {
    font-family: var(--font-mono);
    font-size: 10px;
    padding: 1px 4px;
    border: 1px solid var(--border);
    border-radius: 3px;
    margin-right: 4px;
  }
</style>
