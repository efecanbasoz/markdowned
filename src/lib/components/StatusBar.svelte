<script lang="ts">
  import { editor } from "$lib/stores/editor.svelte";
</script>

<div class="statusbar">
  <span class="left">Ln {editor.line}, Col {editor.column}</span>
  <span class="right">UTF-8 &middot; Markdown{editor.fileSize ? ` \u00B7 ${editor.fileSize}` : ""}</span>
</div>

<style>
  .statusbar {
    height: var(--statusbar-height);
    min-height: var(--statusbar-height);
    background: var(--surface);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    font-family: var(--font-mono);
    font-size: 11px;
    color: var(--text-muted);
    user-select: none;
  }

  .left, .right {
    white-space: nowrap;
  }
</style>
