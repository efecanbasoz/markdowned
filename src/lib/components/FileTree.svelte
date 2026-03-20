<script lang="ts">
  import type { FileEntry } from "$lib/types";
  import FileTreeItem from "./FileTreeItem.svelte";

  let { entries }: { entries: FileEntry[] } = $props();
</script>

<div class="file-tree">
  {#each entries as entry}
    <FileTreeItem {entry} depth={0} />
  {/each}
</div>

<style>
  .file-tree {
    display: flex;
    flex-direction: column;
  }
</style>
