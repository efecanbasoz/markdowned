<script lang="ts">
  import type { FileEntry } from "$lib/types";
  import FileTreeItem from "./FileTreeItem.svelte";

  let { entries, onFileSelect }: {
    entries: FileEntry[];
    onFileSelect: (path: string) => void;
  } = $props();
</script>

<div class="file-tree">
  {#each entries as entry}
    <FileTreeItem {entry} depth={0} {onFileSelect} />
  {/each}
</div>

<style>
  .file-tree {
    display: flex;
    flex-direction: column;
  }
</style>
