use std::fs;
use std::path::Path;
use tauri::command;
use crate::fs::FileInfo;

#[command]
pub async fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[command]
pub async fn write_file(path: &str, contents: &str) -> Result<(), String> {
    fs::write(path, contents).map_err(|e| e.to_string())
}

#[command]
pub async fn list_files(path: &str) -> Result<Vec<FileInfo>, String> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(format!("Path does not exist: {}", path.display()));
    }
    
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {}", path.display()));
    }
    
    let entries = fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|entry| {
            let path = entry.path();
            FileInfo::from_path(&path).map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(entries)
} 