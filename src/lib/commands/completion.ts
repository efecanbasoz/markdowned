import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/**
 * Request an AI completion from the Tauri backend.
 * Streams chunks via `completion-chunk` events and signals
 * completion via `completion-done`.
 *
 * Returns a cleanup function to cancel/unsubscribe.
 */
export async function requestCompletion(
  context: string,
  onChunk: (text: string) => void,
  onDone: () => void
): Promise<() => void> {
  const unlistenChunk = await listen<string>("completion-chunk", (event) => {
    onChunk(event.payload);
  });
  const unlistenDone = await listen("completion-done", () => {
    onDone();
    cleanup();
  });

  function cleanup() {
    unlistenChunk();
    unlistenDone();
  }

  invoke("request_completion", { context }).catch(() => cleanup());

  return cleanup;
}
