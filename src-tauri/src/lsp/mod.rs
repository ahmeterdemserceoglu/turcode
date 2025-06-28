pub mod commands;
pub mod server;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

// Represents a diagnostic message from an LSP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub line: u32,
    pub column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

// Represents a language server process
struct LanguageServerProcess {
    language_id: String,
    process: Child,
    root_uri: String,
}

// Manages language servers for different languages
pub struct LanguageServerManager {
    servers: Arc<Mutex<HashMap<String, LanguageServerProcess>>>,
}

impl LanguageServerManager {
    pub fn new() -> Self {
        LanguageServerManager {
            servers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    // Start a language server for a specific language and project root
    pub fn start_server(&self, language_id: &str, root_path: &PathBuf) -> Result<(), String> {
        let mut servers = self.servers.lock().unwrap();
        
        // Skip if server already running for this language
        if servers.contains_key(language_id) {
            return Ok(());
        }
        
        // Get command to start language server based on language ID
        let (command, args) = match language_id {
            "typescript" | "javascript" => (
                "typescript-language-server",
                vec!["--stdio"]
            ),
            "python" => (
                "pylsp",
                vec![]
            ),
            "rust" => (
                "rust-analyzer",
                vec![]
            ),
            "go" => (
                "gopls",
                vec!["serve", "-rpc.trace"]
            ),
            "java" => (
                "jdtls",
                vec!["-data", root_path.to_string_lossy().as_ref()]
            ),
            "html" | "css" => (
                "vscode-html-language-server",
                vec!["--stdio"]
            ),
            // Add more language servers as needed
            _ => return Err(format!("No language server configured for {}", language_id)),
        };
        
        // Try to spawn the language server process
        let process = Command::new(command)
            .args(args)
            .spawn()
            .map_err(|e| format!("Failed to start language server: {}", e))?;
        
        // Store the language server process
        let root_uri = format!("file://{}", root_path.to_string_lossy());
        servers.insert(language_id.to_string(), LanguageServerProcess {
            language_id: language_id.to_string(),
            process,
            root_uri,
        });
        
        Ok(())
    }
    
    // Stop a language server
    pub fn stop_server(&self, language_id: &str) -> Result<(), String> {
        let mut servers = self.servers.lock().unwrap();
        
        if let Some(mut server) = servers.remove(language_id) {
            // Try to kill the process
            match server.process.kill() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to stop language server: {}", e)),
            }
        } else {
            Err(format!("No language server running for {}", language_id))
        }
    }
    
    // Get the root URI for a language server
    pub fn get_root_uri(&self, language_id: &str) -> Option<String> {
        let servers = self.servers.lock().unwrap();
        servers.get(language_id).map(|server| server.root_uri.clone())
    }
} 