<script lang="ts">
  import { workspace } from "$lib/stores/workspace.svelte";
</script>

<aside class="sidebar">
  <header class="sidebar-header">
    <span class="label">EXPLORER</span>
  </header>
  <div class="sidebar-content">
    <p class="placeholder-text">Open a workspace to begin</p>
  </div>
  <footer class="sidebar-footer">
    <span class="file-count">{workspace.fileCount} files</span>
  </footer>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    min-width: var(--sidebar-width);
    background: var(--surface);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    height: 100%;
    user-select: none;
  }

  .sidebar-header {
    padding: 10px 12px 6px;
    border-bottom: 1px solid var(--border);
  }

  .label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    color: var(--text-dimmed);
  }

  .sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: 12px;
  }

  .placeholder-text {
    color: var(--text-dimmed);
    font-size: 12px;
    text-align: center;
    margin-top: 24px;
  }

  .sidebar-footer {
    padding: 6px 12px;
    border-top: 1px solid var(--border);
  }

  .file-count {
    font-size: 11px;
    color: var(--text-dimmed);
    font-family: var(--font-mono);
  }
</style>
