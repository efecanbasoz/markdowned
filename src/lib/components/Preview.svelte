<script lang="ts">
  import { onMount } from "svelte";
  import { editor } from "$lib/stores/editor.svelte";
  import type { ScrollMetrics } from "$lib/utils/scroll-sync";
  import { mapScrollToTarget } from "$lib/utils/scroll-sync";

  let { onScrollChange = undefined }: {
    onScrollChange?: (metrics: ScrollMetrics) => void;
  } = $props();

  let previewContainer = $state<HTMLDivElement | null>(null);
  let pendingSyncedScrollTop: number | null = null;

  export function getScrollMetrics(): ScrollMetrics | null {
    if (!previewContainer) return null;
    const { scrollTop, scrollHeight, clientHeight } = previewContainer;
    return { scrollTop, scrollHeight, clientHeight };
  }

  function emitScrollChange() {
    const metrics = getScrollMetrics();
    if (metrics) {
      onScrollChange?.(metrics);
    }
  }

  function clearPendingSyncIfMatched(currentScrollTop: number) {
    if (
      pendingSyncedScrollTop !== null &&
      Math.abs(currentScrollTop - pendingSyncedScrollTop) < 1
    ) {
      pendingSyncedScrollTop = null;
      return true;
    }
    return false;
  }

  export function syncScroll(metrics: ScrollMetrics) {
    if (!previewContainer) return;
    const targetScrollTop = mapScrollToTarget(
      metrics,
      previewContainer.scrollHeight,
      previewContainer.clientHeight,
    );

    pendingSyncedScrollTop = targetScrollTop;
    previewContainer.scrollTop = targetScrollTop;

    requestAnimationFrame(() => {
      if (previewContainer) {
        clearPendingSyncIfMatched(previewContainer.scrollTop);
      }
    });
  }

  onMount(() => {
    const handleScroll = () => {
      if (!previewContainer) return;
      if (clearPendingSyncIfMatched(previewContainer.scrollTop)) return;
      emitScrollChange();
    };

    previewContainer?.addEventListener("scroll", handleScroll, { passive: true });

    return () => {
      previewContainer?.removeEventListener("scroll", handleScroll);
    };
  });
</script>

<div class="preview-container" bind:this={previewContainer}>
  {#if editor.frontmatter}
    <div class="frontmatter-badge">
      {#each editor.frontmatter.split("\n").filter(Boolean) as line}
        {@const parts = line.split(":")}
        {@const key = parts[0]?.trim()}
        {@const value = parts.slice(1).join(":").trim()}
        <span class="fm-entry">
          <span class="fm-key">{key}:</span> {value}
        </span>
      {/each}
    </div>
  {/if}
  <div class="preview-content">
    {@html editor.previewHtml}
  </div>
</div>

<style>
  .preview-container {
    height: 100%;
    width: 100%;
    overflow-y: auto;
    padding: 32px 48px;
  }

  .frontmatter-badge {
    display: inline-flex;
    gap: 16px;
    padding: 8px 14px;
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 11px;
    font-family: var(--font-mono);
    margin-bottom: 20px;
  }

  .fm-key {
    color: var(--accent-light);
  }

  /* Content styles for rendered HTML */
  .preview-content :global(h1) {
    color: var(--text-primary);
    font-size: 28px;
    font-weight: 700;
    letter-spacing: -0.5px;
  }

  .preview-content :global(h2) {
    color: var(--text-primary);
    font-size: 20px;
    font-weight: 600;
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }

  .preview-content :global(h3) {
    color: var(--text-primary);
    font-size: 16px;
    font-weight: 600;
  }

  .preview-content :global(p) {
    color: var(--text-secondary);
    font-size: 15px;
    line-height: 1.75;
  }

  .preview-content :global(a) {
    color: var(--accent-light);
    text-decoration: underline;
  }

  .preview-content :global(strong) {
    color: var(--text-primary);
    font-weight: 600;
  }

  .preview-content :global(code) {
    font-family: var(--font-mono);
    font-size: 0.9em;
    background: var(--surface-elevated);
    padding: 2px 6px;
    border-radius: 4px;
    color: var(--green);
  }

  .preview-content :global(pre.code-block) {
    background: var(--surface-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px 18px;
    font-family: var(--font-mono);
    font-size: 13px;
    line-height: 1.7;
  }

  .preview-content :global(pre.code-block code) {
    background: transparent;
    padding: 0;
    color: inherit;
  }

  .preview-content :global(table) {
    border-collapse: collapse;
    width: 100%;
  }

  .preview-content :global(th) {
    background: var(--surface-elevated);
    color: var(--text-primary);
    font-weight: 600;
    border: 1px solid var(--border);
    padding: 8px 12px;
  }

  .preview-content :global(td) {
    color: var(--text-secondary);
    border: 1px solid var(--border);
    padding: 8px 12px;
  }

  .preview-content :global(ul),
  .preview-content :global(ol) {
    color: var(--text-secondary);
    padding-left: 24px;
    line-height: 1.75;
  }

  .preview-content :global(blockquote) {
    border-left: 3px solid var(--accent);
    padding: 8px 16px;
    color: var(--text-muted);
    background: rgba(37, 99, 235, 0.05);
    border-radius: 0 6px 6px 0;
  }

  .preview-content :global(hr) {
    border: none;
    border-top: 1px solid var(--border);
    margin: 24px 0;
  }

  .preview-content :global(img) {
    max-width: 100%;
    border-radius: 6px;
  }
</style>
