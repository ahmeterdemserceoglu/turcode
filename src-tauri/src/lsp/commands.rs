use tauri::command;
use tauri::State;
use std::path::PathBuf;
use crate::lsp::{LanguageServerManager, Diagnostic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: u32,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[command]
pub async fn initialize_language_server(
    lsp_manager: State<'_, LanguageServerManager>,
    language_id: String,
    root_path: String,
) -> Result<(), String> {
    let path = PathBuf::from(root_path);
    lsp_manager.start_server(&language_id, &path)
}

#[command]
pub async fn get_completions(
    lsp_manager: State<'_, LanguageServerManager>,
    language_id: String,
    file_path: String,
    position: Position,
) -> Result<Vec<CompletionItem>, String> {
    // In a real implementation, we would send an LSP completion request
    // For now, just return mock completions
    
    // Check if we have a language server running
    match lsp_manager.get_root_uri(&language_id) {
        Some(_) => {
            // Mock completions based on language
            let completions = match language_id.as_str() {
                "typescript" | "javascript" => vec![
                    CompletionItem {
                        label: "console.log".to_string(),
                        kind: 1, // Method
                        detail: Some("console.log(message: any): void".to_string()),
                        documentation: Some("Logs a message to the console".to_string()),
                        insert_text: Some("console.log($1)$0".to_string()),
                    },
                    CompletionItem {
                        label: "function".to_string(),
                        kind: 14, // Snippet
                        detail: Some("Function Declaration".to_string()),
                        documentation: Some("Creates a new function".to_string()),
                        insert_text: Some("function ${1:name}(${2:params}) {\n\t${0}\n}".to_string()),
                    },
                ],
                "python" => vec![
                    CompletionItem {
                        label: "def".to_string(),
                        kind: 14, // Snippet
                        detail: Some("Function Definition".to_string()),
                        documentation: Some("Define a new function".to_string()),
                        insert_text: Some("def ${1:name}(${2:params}):\n\t${0}".to_string()),
                    },
                    CompletionItem {
                        label: "print".to_string(),
                        kind: 1, // Method
                        detail: Some("print(*args, sep=' ', end='\n')".to_string()),
                        documentation: Some("Print objects to the text stream file".to_string()),
                        insert_text: Some("print($1)$0".to_string()),
                    },
                ],
                "rust" => vec![
                    CompletionItem {
                        label: "fn".to_string(),
                        kind: 14, // Snippet
                        detail: Some("Function Definition".to_string()),
                        documentation: Some("Define a new function".to_string()),
                        insert_text: Some("fn ${1:name}(${2:params}) -> ${3:ReturnType} {\n\t${0}\n}".to_string()),
                    },
                    CompletionItem {
                        label: "println!".to_string(),
                        kind: 1, // Method
                        detail: Some("println!(args)".to_string()),
                        documentation: Some("Print to the standard output".to_string()),
                        insert_text: Some("println!(\"$1\")$0".to_string()),
                    },
                ],
                _ => vec![],
            };
            
            Ok(completions)
        },
        None => Err(format!("No language server running for {}", language_id)),
    }
}

#[command]
pub async fn get_diagnostics(
    lsp_manager: State<'_, LanguageServerManager>,
    language_id: String,
    file_path: String,
) -> Result<Vec<Diagnostic>, String> {
    // In a real implementation, we would retrieve diagnostics from the LSP
    // For now, just return mock diagnostics
    
    // Check if we have a language server running
    match lsp_manager.get_root_uri(&language_id) {
        Some(_) => {
            // Mock diagnostics
            let diagnostics = vec![
                Diagnostic {
                    line: 10,
                    column: 5,
                    end_line: 10,
                    end_column: 15,
                    severity: crate::lsp::DiagnosticSeverity::Error,
                    code: Some("E0001".to_string()),
                    source: Some(language_id.clone()),
                    message: "Undefined variable 'someVar'".to_string(),
                },
                Diagnostic {
                    line: 15,
                    column: 10,
                    end_line: 15,
                    end_column: 20,
                    severity: crate::lsp::DiagnosticSeverity::Warning,
                    code: Some("W0001".to_string()),
                    source: Some(language_id),
                    message: "Unused variable".to_string(),
                },
            ];
            
            Ok(diagnostics)
        },
        None => Err(format!("No language server running for {}", language_id)),
    }
} 