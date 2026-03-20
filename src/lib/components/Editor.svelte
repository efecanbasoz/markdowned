<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { createExtensions } from "$lib/editor/extensions";
  import { editor as editorStore } from "$lib/stores/editor.svelte";

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

  function handleUpdate(content: string, line: number, col: number) {
    editorStore.line = line;
    editorStore.column = col;
    editorStore.dirty = true;
    editorStore.content = content;
  }

  onMount(() => {
    const state = EditorState.create({
      doc: editorStore.content,
      extensions: createExtensions(handleUpdate),
    });
    view = new EditorView({ state, parent: container });
    return () => {
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
