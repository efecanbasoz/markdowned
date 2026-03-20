<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { createExtensions } from "$lib/editor/extensions";
  import { editor as editorStore } from "$lib/stores/editor.svelte";
  import {
    appendGhostText,
    clearGhostText,
  } from "$lib/editor/ghost-text";
  import { requestCompletion } from "$lib/commands/completion";

  let container: HTMLDivElement;
  let view: EditorView | null = null;

  export function setContent(content: string) {
    if (!view) return;
    view.dispatch({
      changes: { from: 0, to: view.state.doc.length, insert: content },
    });
  }

  export function getView(): EditorView | null {
    return view;
  }

  let cleanupCompletion: (() => void) | null = null;

  function handleUpdate(content: string, line: number, col: number) {
    editorStore.line = line;
    editorStore.column = col;
    editorStore.dirty = true;
    editorStore.content = content;
  }

  function triggerCompletion(editorView: EditorView) {
    // Cancel any in-flight completion
    cleanupCompletion?.();
    cleanupCompletion = null;

    // Get document content up to cursor position
    const cursor = editorView.state.selection.main.head;
    const context = editorView.state.sliceDoc(0, cursor);

    // Clear any existing ghost text
    editorView.dispatch({ effects: clearGhostText.of(undefined) });

    requestCompletion(
      context,
      (chunk) => {
        editorView.dispatch({ effects: appendGhostText.of(chunk) });
      },
      () => {
        cleanupCompletion = null;
      }
    ).then((cleanup) => {
      cleanupCompletion = cleanup;
    });
  }

  onMount(() => {
    const state = EditorState.create({
      doc: editorStore.content,
      extensions: createExtensions(handleUpdate, triggerCompletion),
    });
    view = new EditorView({ state, parent: container });
    return () => {
      cleanupCompletion?.();
      view?.destroy();
      view = null;
    };
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
