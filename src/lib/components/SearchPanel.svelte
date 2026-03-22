<script lang="ts">
  import { searchWorkspace } from "$lib/commands/workspace";
  import type { SearchMatch } from "$lib/types";

  let { visible = $bindable(false), onSelect }: {
    visible: boolean;
    onSelect: (filePath: string, lineNumber: number) => void;
  } = $props();

  let query = $state("");
  let results = $state<SearchMatch[]>([]);
  let selectedIndex = $state(0);
  let searching = $state(false);
  let inputEl = $state<HTMLInputElement | undefined>(undefined);
  let debounceTimer: ReturnType<typeof setTimeout> | undefined;
  let searchRun = 0;

  function close() {
    visible = false;
    query = "";
    results = [];
    selectedIndex = 0;
    searching = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      scrollSelectedIntoView();
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
      scrollSelectedIntoView();
    } else if (e.key === "Enter") {
      e.preventDefault();
      const match = results[selectedIndex];
      if (match) {
        onSelect(match.filePath, match.lineNumber);
        close();
      }
    } else if (e.key === "Escape") {
      close();
    }
  }

  function scrollSelectedIntoView() {
    requestAnimationFrame(() => {
      const el = document.querySelector(".search-item.selected");
      el?.scrollIntoView({ block: "nearest" });
    });
  }

  $effect(() => {
    if (visible) requestAnimationFrame(() => inputEl?.focus());
  });

  $effect(() => {
    const q = query;
    selectedIndex = 0;

    if (debounceTimer) clearTimeout(debounceTimer);

    if (q.trim().length === 0) {
      results = [];
      searching = false;
      return;
    }

    searching = true;
    // QA-012: Use sequence token to prevent stale search results from overwriting newer ones
    const run = ++searchRun;
    debounceTimer = setTimeout(async () => {
      try {
        const found = await searchWorkspace(q);
        if (run === searchRun) results = found;
      } catch (e) {
        console.error("Search failed:", e);
        if (run === searchRun) results = [];
      } finally {
        if (run === searchRun) searching = false;
      }
    }, 300);
  });
</script>

{#if visible}
  <div class="overlay" role="presentation" onclick={close}>
    <!-- svelte-ignore a11y_interactive_supports_focus -->
    <div class="search-panel" role="dialog" aria-modal="true" aria-label="Workspace search" onclick={(e) => e.stopPropagation()} onkeydown={handleKeydown}>
      <div class="search-input">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input
          bind:this={inputEl}
          bind:value={query}
          type="text"
          placeholder="Search in workspace..."
          spellcheck="false"
          autocomplete="off"
        />
        {#if searching}
          <span class="search-spinner">...</span>
        {/if}
      </div>
      <div class="search-results" role="listbox">
        {#each results as match, i}
          <button
            class="search-item"
            class:selected={i === selectedIndex}
            role="option"
            aria-selected={i === selectedIndex}
            onclick={() => { onSelect(match.filePath, match.lineNumber); close(); }}
            onmouseenter={() => selectedIndex = i}
          >
            <div class="search-item-header">
              <span class="search-file-name">{match.fileName}</span>
              <span class="search-line-number">:{match.lineNumber}</span>
            </div>
            <div class="search-line-content">
              {match.lineContent.slice(0, match.matchStart)}<mark>{match.lineContent.slice(match.matchStart, match.matchEnd)}</mark>{match.lineContent.slice(match.matchEnd)}
            </div>
          </button>
        {/each}
        {#if !searching && query.trim().length > 0 && results.length === 0}
          <div class="search-empty">No matches found</div>
        {/if}
      </div>
      <div class="search-footer">
        <div class="search-hints">
          <span><kbd>&uarr;&darr;</kbd> navigate</span>
          <span><kbd>&crarr;</kbd> open</span>
          <span><kbd>esc</kbd> close</span>
        </div>
        {#if results.length > 0}
          <span class="search-count">{results.length}{results.length >= 200 ? "+" : ""} results</span>
        {/if}
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

  .search-panel {
    width: 600px;
    max-height: 500px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 10px;
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    align-self: flex-start;
  }

  .search-input {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px 16px;
    border-bottom: 1px solid var(--border);
    color: var(--text-dimmed);
  }

  .search-input input {
    flex: 1;
    background: transparent;
    border: none;
    color: var(--text-primary);
    font-size: 14px;
    font-family: inherit;
    outline: none;
  }

  .search-input input::placeholder {
    color: var(--text-dimmed);
  }

  .search-spinner {
    font-size: 12px;
    color: var(--text-dimmed);
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }

  .search-results {
    flex: 1;
    overflow-y: auto;
    padding: 6px;
  }

  .search-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
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

  .search-item:hover {
    background: rgba(37, 99, 235, 0.06);
  }

  .search-item.selected {
    background: linear-gradient(90deg, rgba(37, 99, 235, 0.12) 0%, rgba(37, 99, 235, 0.04) 100%);
    border-left-color: var(--accent);
  }

  .search-item-header {
    display: flex;
    align-items: baseline;
    gap: 2px;
  }

  .search-file-name {
    font-weight: 500;
    white-space: nowrap;
  }

  .search-line-number {
    color: var(--text-dimmed);
    font-size: 12px;
    font-family: var(--font-mono);
  }

  .search-line-content {
    font-family: var(--font-mono);
    font-size: 12px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .search-line-content :global(mark) {
    background: rgba(250, 204, 21, 0.3);
    color: var(--text-primary);
    border-radius: 2px;
    padding: 0 1px;
  }

  .search-empty {
    padding: 16px;
    text-align: center;
    color: var(--text-dimmed);
    font-size: 13px;
  }

  .search-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    border-top: 1px solid var(--border);
    color: var(--text-dimmed);
    font-size: 11px;
  }

  .search-hints {
    display: flex;
    gap: 16px;
  }

  .search-hints kbd {
    font-family: var(--font-mono);
    font-size: 10px;
    padding: 1px 4px;
    border: 1px solid var(--border);
    border-radius: 3px;
    margin-right: 4px;
  }

  .search-count {
    font-family: var(--font-mono);
    color: var(--text-dimmed);
  }
</style>
