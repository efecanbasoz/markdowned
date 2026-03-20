<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { createExtensions, reconfigureTheme } from "$lib/editor/extensions";
  import { editor as editorStore } from "$lib/stores/editor.svelte";
  import {
    appendGhostText,
    clearGhostText,
  } from "$lib/editor/ghost-text";
  import { requestCompletion } from "$lib/commands/completion";
  import { loadConfig } from "$lib/commands/config";

  let container: HTMLDivElement;
  let view: EditorView | null = null;
  let autoCompletionEnabled = false;
  let autoCompletionTimer: ReturnType<typeof setTimeout> | null = null;

  export function setContent(content: string) {
    if (!view) return;
    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: content },
    });
  }

  export function getView(): EditorView | null {
    return view;
  }

  export function restoreState(cursorPos: number, scrollTop: number) {
    if (!view) return;
    try {
      const docLength = view.state.doc.length;
      const safePos = Math.min(cursorPos, docLength);
      view.dispatch({ selection: { anchor: safePos } });
      view.scrollDOM.scrollTop = scrollTop;
    } catch {
      // Ignore if position is invalid
    }
  }

  export function goToLine(lineNumber: number) {
    if (!view) return;
    const line = view.state.doc.line(Math.min(lineNumber, view.state.doc.lines));
    view.dispatch({
      selection: { anchor: line.from },
      effects: EditorView.scrollIntoView(line.from, { y: "center" }),
    });
    view.focus();
  }

  let cleanupCompletion: (() => void) | null = null;

  function handleUpdate(content: string, line: number, col: number, docChanged: boolean) {
    const cursorPos = view?.state.selection.main.head ?? 0;
    editorStore.updateActiveCursor(line, col, cursorPos);
    editorStore.updateActiveContent(content);

    // Auto-completion: debounce on typing pause
    if (autoCompletionEnabled && docChanged && view) {
      if (autoCompletionTimer) clearTimeout(autoCompletionTimer);
      const currentView = view;
      autoCompletionTimer = setTimeout(() => {
        autoCompletionTimer = null;
        triggerCompletion(currentView);
      }, 1500);
    }
  }

  async function triggerCompletion(editorView: EditorView) {
    // Cancel any in-flight completion
    cleanupCompletion?.();
    cleanupCompletion = null;

    // Get document content up to cursor position
    const cursor = editorView.state.selection.main.head;
    const context = editorView.state.sliceDoc(0, cursor);

    // Clear any existing ghost text
    editorView.dispatch({ effects: clearGhostText.of(undefined) });

    const cleanup = await requestCompletion(
      context,
      (chunk) => {
        editorView.dispatch({ effects: appendGhostText.of(chunk) });
      },
      () => {
        cleanupCompletion = null;
      }
    );
    cleanupCompletion = cleanup;
  }

  onMount(() => {
    const state = EditorState.create({
      doc: editorStore.content,
      extensions: createExtensions(handleUpdate, triggerCompletion, editorStore.theme === "dark"),
    });
    view = new EditorView({ state, parent: container });

    // Restore cursor/scroll for the active tab
    const activeTab = editorStore.activeTab;
    if (activeTab && (activeTab.cursorPos > 0 || activeTab.scrollTop > 0)) {
      restoreState(activeTab.cursorPos, activeTab.scrollTop);
    }

    // Load config to check if auto-completion is enabled
    loadConfig().then((config) => {
      autoCompletionEnabled = config.completion.autoCompletion ?? false;
    });

    return () => {
      cleanupCompletion?.();
      if (autoCompletionTimer) clearTimeout(autoCompletionTimer);
      view?.destroy();
      view = null;
    };
  });

  // Watch for theme changes and reconfigure CodeMirror
  $effect(() => {
    const currentTheme = editorStore.theme;
    if (view) {
      reconfigureTheme(view, currentTheme === "dark");
    }
  });
</script>

<div class="editor-container" bind:this={container}></div>

<style>
  .editor-container {
    height: 100%;
    overflow: hidden;
  }
  .editor-container :global(.cm-editor) {
    height: 100%;
  }
  .editor-container :global(.cm-scroller) {
    overflow: auto;
  }
</style>
