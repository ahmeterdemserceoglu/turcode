pub mod commands;
pub mod watcher;

use std::path::Path;

#[derive(Debug, serde::Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<String>,
}

impl FileInfo {
    pub fn from_path(path: &Path) -> Result<Self, std::io::Error> {
        let metadata = path.metadata()?;
        
        let modified = metadata
            .modified()
            .ok()
            .map(|time| {
                let datetime: chrono::DateTime<chrono::Utc> = time.into();
                datetime.to_rfc3339()
            });
        
        Ok(FileInfo {
            name: path.file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| String::from(".")),
            path: path.to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
        })
    }
} 