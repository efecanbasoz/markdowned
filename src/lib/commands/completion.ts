import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let requestCounter = 0;

/**
 * Request an AI completion from the Tauri backend.
 * QA-002: Each request has a unique ID so chunks from stale requests are ignored.
 *
 * Returns a cleanup function to cancel/unsubscribe.
 */
export async function requestCompletion(
  context: string,
  onChunk: (text: string) => void,
  onDone: () => void
): Promise<() => void> {
  const requestId = `req-${++requestCounter}-${Date.now()}`;

  const unlistenChunk = await listen<{ request_id: string; text: string }>("completion-chunk", (event) => {
    if (event.payload.request_id === requestId) {
      onChunk(event.payload.text);
    }
  });
  const unlistenDone = await listen<{ request_id: string }>("completion-done", (event) => {
    if (event.payload.request_id === requestId) {
      onDone();
      cleanup();
    }
  });

  function cleanup() {
    unlistenChunk();
    unlistenDone();
  }

  invoke("request_completion", { context, requestId }).catch(() => cleanup());

  return cleanup;
}
