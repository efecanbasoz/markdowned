use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChangeEvent {
    pub kind: String,
    pub path: String,
}

struct WatcherHandle {
    _watcher: RecommendedWatcher,
}

pub struct WatcherState {
    handle: Mutex<Option<WatcherHandle>>,
}

impl Default for WatcherState {
    fn default() -> Self {
        Self { handle: Mutex::new(None) }
    }
}

pub fn start_watcher(app: AppHandle, workspace_path: String, watcher_state: &WatcherState) -> Result<(), String> {
    // Drop the previous watcher before creating a new one
    {
        let mut guard = watcher_state.handle.lock().unwrap();
        *guard = None;
    }

    let (tx, rx) = mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .map_err(|e| format!("Failed to create watcher: {e}"))?;

    watcher
        .watch(Path::new(&workspace_path), RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {e}"))?;

    // Store the watcher handle to keep it alive and allow cleanup
    {
        let mut guard = watcher_state.handle.lock().unwrap();
        *guard = Some(WatcherHandle { _watcher: watcher });
    }

    std::thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            if let Ok(Event { kind, paths, .. }) = event {
                let kind_str = match kind {
                    EventKind::Create(_) => "created",
                    EventKind::Modify(_) => "modified",
                    EventKind::Remove(_) => "deleted",
                    _ => continue,
                };
                for path in paths {
                    let _ = app.emit(
                        "file-changed",
                        FileChangeEvent {
                            kind: kind_str.to_string(),
                            path: path.to_string_lossy().to_string(),
                        },
                    );
                }
            }
        }
    });

    Ok(())
}
