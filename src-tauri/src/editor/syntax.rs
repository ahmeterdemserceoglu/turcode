use std::path::Path;
use std::collections::HashMap;

// Map file extensions to language identifiers
pub fn get_language_id_from_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    let extension = path.extension()?.to_string_lossy().to_lowercase();
    
    let mut language_map = HashMap::new();
    
    // JavaScript and TypeScript
    language_map.insert("js", "javascript");
    language_map.insert("jsx", "javascriptreact");
    language_map.insert("ts", "typescript");
    language_map.insert("tsx", "typescriptreact");
    
    // C-family
    language_map.insert("c", "c");
    language_map.insert("cpp", "cpp");
    language_map.insert("h", "c");
    language_map.insert("hpp", "cpp");
    
    // Web
    language_map.insert("html", "html");
    language_map.insert("css", "css");
    language_map.insert("scss", "scss");
    language_map.insert("sass", "sass");
    
    // Python, Rust, Go, Java
    language_map.insert("py", "python");
    language_map.insert("rs", "rust");
    language_map.insert("go", "go");
    language_map.insert("java", "java");
    
    // PHP
    language_map.insert("php", "php");
    
    // Other common formats
    language_map.insert("json", "json");
    language_map.insert("md", "markdown");
    language_map.insert("xml", "xml");
    language_map.insert("yaml", "yaml");
    language_map.insert("yml", "yaml");
    
    // Get language ID from map
    language_map.get(extension.as_ref()).map(|&s| s.to_string())
} 