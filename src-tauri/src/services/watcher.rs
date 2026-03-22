use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChangeEvent {
    pub kind: String,
    pub path: String,
    pub workspace_root: String,
}

struct WatcherHandle {
    _watcher: RecommendedWatcher,
}

pub struct WatcherState {
    handles: Mutex<HashMap<String, WatcherHandle>>,
}

impl Default for WatcherState {
    fn default() -> Self {
        Self {
            handles: Mutex::new(HashMap::new()),
        }
    }
}

pub fn start_watcher(
    app: AppHandle,
    workspace_path: String,
    watcher_state: &WatcherState,
) -> Result<(), String> {
    let canonical = std::fs::canonicalize(&workspace_path)
        .map_err(|e| format!("Invalid path: {e}"))?
        .to_string_lossy()
        .to_string();

    // Remove existing watcher for this path if any
    {
        let mut guard = watcher_state
            .handles
            .lock()
            .map_err(|e| format!("Watcher state error: {e}"))?;
        guard.remove(&canonical);
    }

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| format!("Failed to create watcher: {e}"))?;

    watcher
        .watch(Path::new(&canonical), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {e}"))?;

    {
        let mut guard = watcher_state
            .handles
            .lock()
            .map_err(|e| format!("Watcher state error: {e}"))?;
        guard.insert(canonical.clone(), WatcherHandle { _watcher: watcher });
    }

    let ws_root = canonical;
    std::thread::spawn(move || {
        use std::time::{Duration, Instant};
        let debounce_ms = Duration::from_millis(200);
        let mut last_emit = Instant::now()
            .checked_sub(debounce_ms)
            .unwrap_or_else(Instant::now);

        while let Ok(event) = rx.recv() {
            if let Ok(Event { kind, paths, .. }) = event {
                let kind_str = match kind {
                    EventKind::Create(_) => "created",
                    EventKind::Modify(_) => "modified",
                    EventKind::Remove(_) => "deleted",
                    _ => continue,
                };

                let now = Instant::now();
                if now.duration_since(last_emit) < debounce_ms {
                    continue;
                }
                last_emit = now;

                for path in paths {
                    let _ = app.emit(
                        "file-changed",
                        FileChangeEvent {
                            kind: kind_str.to_string(),
                            path: path.to_string_lossy().to_string(),
                            workspace_root: ws_root.clone(),
                        },
                    );
                }
            }
        }
    });

    Ok(())
}

pub fn stop_watcher(workspace_path: String, watcher_state: &WatcherState) -> Result<(), String> {
    let canonical = std::fs::canonicalize(&workspace_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(&workspace_path))
        .to_string_lossy()
        .to_string();
    let mut guard = watcher_state
        .handles
        .lock()
        .map_err(|e| format!("Watcher state error: {e}"))?;
    guard.remove(&canonical);
    Ok(())
}
