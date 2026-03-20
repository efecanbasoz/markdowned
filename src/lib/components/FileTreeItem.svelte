<script lang="ts">
  import type { FileEntry } from "$lib/types";
  import { editor } from "$lib/stores/editor.svelte";
  import FileTreeItem from "./FileTreeItem.svelte";

  let { entry, depth = 0, onFileSelect }: {
    entry: FileEntry;
    depth?: number;
    onFileSelect: (path: string) => void;
  } = $props();
  let expanded = $state(false);
  const isActive = $derived(editor.activeTabId === entry.path);
  const isDir = $derived(entry.fileType === "directory");
  const indent = $derived(depth * 16 + 8);

  function handleClick() {
    if (entry.fileType === "directory") {
      expanded = !expanded;
    } else {
      onFileSelect(entry.path);
    }
  }
</script>

<button
  class="tree-item"
  class:active={isActive}
  style="padding-left: {indent}px"
  onclick={handleClick}
>
  {#if isDir}
    <span class="chevron">{expanded ? "\u25BE" : "\u25B8"}</span>
  {/if}
  <span class="icon" role="img" aria-label={isDir ? "folder" : "file"}>{isDir ? "\uD83D\uDCC1" : "\uD83D\uDCC4"}</span>
  <span class="name">{entry.name}</span>
</button>

{#if isDir && expanded && entry.children}
  {#each entry.children as child}
    <FileTreeItem entry={child} depth={depth + 1} {onFileSelect} />
  {/each}
{/if}

<style>
  .tree-item {
    display: flex;
    align-items: center;
    gap: 4px;
    width: 100%;
    font-size: 12px;
    color: var(--text-muted);
    background: transparent;
    border: none;
    border-left: 2px solid transparent;
    cursor: pointer;
    padding-top: 2px;
    padding-bottom: 2px;
    text-align: left;
  }

  .tree-item:hover {
    color: var(--text-secondary);
    background: rgba(56, 139, 253, 0.06);
  }

  .tree-item.active {
    color: var(--accent-glow);
    background: linear-gradient(90deg, rgba(56, 139, 253, 0.12), transparent);
    border-left-color: var(--accent);
  }

  .chevron {
    font-size: 10px;
    width: 12px;
    flex-shrink: 0;
  }

  .icon {
    flex-shrink: 0;
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
