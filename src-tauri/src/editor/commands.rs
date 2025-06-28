use tauri::command;
use crate::editor::syntax::get_language_id_from_path;

#[derive(Debug, serde::Serialize)]
pub struct SyntaxHighlightingResult {
    pub language_id: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug, serde::Serialize)]
pub struct Token {
    pub line: usize,
    pub start_char: usize,
    pub end_char: usize,
    pub token_type: String,
}

#[command]
pub async fn get_syntax_highlighting(file_path: &str, content: &str) -> Result<SyntaxHighlightingResult, String> {
    // Determine language from file extension
    let language_id = get_language_id_from_path(file_path)
        .unwrap_or_else(|| "plaintext".to_string());
        
    // For now, return a simplified result without actual syntax highlighting
    // In a real implementation, we would use tree-sitter here
    let tokens = vec![];
    
    Ok(SyntaxHighlightingResult {
        language_id,
        tokens,
    })
} 