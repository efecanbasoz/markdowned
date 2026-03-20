<script lang="ts">
  import Sidebar from "$lib/components/Sidebar.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import { workspace } from "$lib/stores/workspace.svelte";
</script>

<div class="app-layout">
  {#if workspace.sidebarVisible}
    <Sidebar />
  {/if}
  <main class="main-area">
    <TabBar />
    <div class="content-area">
      <div class="placeholder">
        <p>Open a workspace to start editing</p>
      </div>
    </div>
    <StatusBar />
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    height: 100vh;
    width: 100%;
  }

  .main-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .content-area {
    flex: 1;
    overflow: hidden;
  }

  .placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-dimmed);
  }
</style>
