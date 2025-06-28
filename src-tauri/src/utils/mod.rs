use std::path::PathBuf;
use std::io;
use std::fs;

// Find project root based on common project files
pub fn find_project_root(start_path: &PathBuf) -> Option<PathBuf> {
    let markers = [
        ".git",
        "package.json",
        "Cargo.toml",
        "go.mod",
        "pom.xml",
        ".sln",
        "requirements.txt",
    ];
    
    let mut current = start_path.clone();
    
    loop {
        // Check for project markers
        for marker in &markers {
            let marker_path = current.join(marker);
            if marker_path.exists() {
                return Some(current.clone());
            }
        }
        
        // Go up one directory
        if !current.pop() {
            break;
        }
    }
    
    // If no project root found, just return the starting directory
    Some(start_path.clone())
}

// Get file size in a human-readable format
pub fn human_readable_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

// Check if a path is binary file
pub fn is_binary_file(path: &PathBuf) -> io::Result<bool> {
    // Some file extensions are obviously binary
    let extension = path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .unwrap_or_default();
        
    let binary_extensions = [
        "exe", "dll", "obj", "bin", "png", "jpg", "jpeg", "gif", "zip", 
        "tar", "gz", "7z", "rar", "pdf", "doc", "docx", "xls", "xlsx"
    ];
    
    if binary_extensions.contains(&extension.as_ref()) {
        return Ok(true);
    }
    
    // Otherwise check file contents for null bytes as a heuristic
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 1024];
    let bytes_read = io::Read::read(&mut file, &mut buffer)?;
    
    // If the file contains null bytes, it's probably binary
    for byte in &buffer[0..bytes_read] {
        if *byte == 0 {
            return Ok(true);
        }
    }
    
    Ok(false)
}

// Determine a unique untitled file name
pub fn get_untitled_filename(existing: &[String]) -> String {
    for i in 1..1000 {
        let filename = format!("Untitled-{}.txt", i);
        if !existing.contains(&filename) {
            return filename;
        }
    }
    
    // Fallback
    "Untitled.txt".to_string()
} 