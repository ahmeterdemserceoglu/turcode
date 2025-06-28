use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

pub enum WatcherEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
}

pub struct FileWatcher {
    watchers: Arc<Mutex<HashMap<String, ()>>>,
    app: AppHandle,
}

impl FileWatcher {
    pub fn new(app: AppHandle) -> Self {
        FileWatcher {
            watchers: Arc::new(Mutex::new(HashMap::new())),
            app,
        }
    }
    
    pub fn watch(&self, path: &Path) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();
        
        // Check if we're already watching this path
        {
            let mut watchers = self.watchers.lock().unwrap();
            if watchers.contains_key(&path_str) {
                return Ok(());
            }
            
            // Add placeholder to mark as being watched
            watchers.insert(path_str.clone(), ());
        }
        
        // Create a channel to communicate watcher events
        let (tx, mut rx) = mpsc::channel(100);
        
        // Clone what we need for the watcher task
        let path_to_watch = PathBuf::from(&path_str);
        let app = self.app.clone();
        let watchers = self.watchers.clone();
        
        // Spawn the watcher task
        tauri::async_runtime::spawn(async move {
            // In a real implementation, we would use something like notify-rs
            // For now, just simulate by checking the path periodically
            
            // Store last modified time for all files in the watched path
            let mut last_modified = HashMap::new();
            
            // Initial scan to collect file metadata
            if path_to_watch.is_dir() {
                match std::fs::read_dir(&path_to_watch) {
                    Ok(entries) => {
                        for entry in entries.filter_map(Result::ok) {
                            let path = entry.path();
                            if let Ok(metadata) = path.metadata() {
                                if let Ok(modified) = metadata.modified() {
                                    last_modified.insert(path, modified);
                                }
                            }
                        }
                    },
                    Err(_) => {}
                }
            } else if path_to_watch.is_file() {
                if let Ok(metadata) = path_to_watch.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        last_modified.insert(path_to_watch.clone(), modified);
                    }
                }
            }
            
            // Poll for changes
            loop {
                sleep(Duration::from_secs(1)).await;
                
                // Check if we still need to watch this path
                {
                    let watchers = watchers.lock().unwrap();
                    if !watchers.contains_key(&path_str) {
                        break;
                    }
                }
                
                // Check for new, modified, or deleted files
                if path_to_watch.is_dir() {
                    let mut current_files = HashMap::new();
                    
                    match std::fs::read_dir(&path_to_watch) {
                        Ok(entries) => {
                            for entry in entries.filter_map(Result::ok) {
                                let path = entry.path();
                                if let Ok(metadata) = path.metadata() {
                                    if let Ok(modified) = metadata.modified() {
                                        current_files.insert(path.clone(), modified);
                                        
                                        // Check for new or modified files
                                        match last_modified.get(&path) {
                                            Some(last_time) => {
                                                if modified != *last_time {
                                                    let _ = tx.send(WatcherEvent::Modified(path)).await;
                                                }
                                            },
                                            None => {
                                                let _ = tx.send(WatcherEvent::Created(path)).await;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => {}
                    }
                    
                    // Check for deleted files
                    for path in last_modified.keys() {
                        if !current_files.contains_key(path) {
                            let _ = tx.send(WatcherEvent::Deleted(path.clone())).await;
                        }
                    }
                    
                    // Update last_modified
                    last_modified = current_files;
                } else if path_to_watch.is_file() {
                    match path_to_watch.metadata() {
                        Ok(metadata) => {
                            if let Ok(modified) = metadata.modified() {
                                if let Some(last_time) = last_modified.get(&path_to_watch) {
                                    if &modified != last_time {
                                        let _ = tx.send(WatcherEvent::Modified(path_to_watch.clone())).await;
                                        last_modified.insert(path_to_watch.clone(), modified);
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            // File might have been deleted
                            if last_modified.contains_key(&path_to_watch) {
                                let _ = tx.send(WatcherEvent::Deleted(path_to_watch.clone())).await;
                                last_modified.remove(&path_to_watch);
                            }
                        }
                    }
                }
            }
        });
        
        // Clone what we need for the event handler
        let app = self.app.clone();
        
        // Spawn task to handle events from the watcher
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                // Process the file system event
                match event {
                    WatcherEvent::Created(path) => {
                        let _ = app.emit_all("fs-created", path.to_string_lossy().to_string());
                    },
                    WatcherEvent::Modified(path) => {
                        let _ = app.emit_all("fs-modified", path.to_string_lossy().to_string());
                    },
                    WatcherEvent::Deleted(path) => {
                        let _ = app.emit_all("fs-deleted", path.to_string_lossy().to_string());
                    }
                }
            }
        });
        
        Ok(())
    }
    
    pub fn unwatch(&self, path: &Path) -> Result<(), String> {
        let path_str = path.to_string_lossy().to_string();
        let mut watchers = self.watchers.lock().unwrap();
        watchers.remove(&path_str);
        Ok(())
    }
} 