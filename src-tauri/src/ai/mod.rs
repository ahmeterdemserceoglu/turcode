pub mod commands;
pub mod client;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

// Response from the AI model for code completion
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeCompletionResponse {
    pub completion: String,
    pub confidence: f32,
    pub language: String,
}

// Response from the AI model for code analysis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeAnalysisResponse {
    pub suggestions: Vec<CodeSuggestion>,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeSuggestion {
    pub line: usize,
    pub suggestion_type: SuggestionType,
    pub content: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SuggestionType {
    Improvement,
    BugFix,
    Refactor,
    Performance,
    Security,
}

// Client for interacting with Claude API
pub struct AIClient {
    api_key: String,
    client: Client,
    model: String,
}

impl AIClient {
    pub fn new(api_key: String, model: String) -> Self {
        AIClient {
            api_key,
            client: Client::new(),
            model,
        }
    }
    
    pub async fn get_code_completion(&self, context: &str, prompt: &str) -> Result<CodeCompletionResponse, String> {
        // In a real implementation, we would call the Claude API
        // For now, just return a mock response
        
        // Mock a simple code completion
        let mock_completion = format!("// Suggested completion\nfunction exampleCompletion() {{\n  console.log(\"This is a mock completion\");\n  // Implement logic here\n}}\n");
        
        Ok(CodeCompletionResponse {
            completion: mock_completion,
            confidence: 0.85,
            language: "typescript".into(),
        })
    }
    
    pub async fn analyze_code(&self, code: &str, language: &str) -> Result<CodeAnalysisResponse, String> {
        // In a real implementation, we would call the Claude API
        // For now, just return a mock response
        
        // Mock a code analysis
        let mock_suggestions = vec![
            CodeSuggestion {
                line: 10,
                suggestion_type: SuggestionType::Performance,
                content: "Use a more efficient data structure".into(),
                description: "Consider using a Map instead of an object for faster lookups".into(),
            },
            CodeSuggestion {
                line: 15,
                suggestion_type: SuggestionType::BugFix,
                content: "Check for null before accessing property".into(),
                description: "Add a null check to prevent potential runtime errors".into(),
            },
        ];
        
        Ok(CodeAnalysisResponse {
            suggestions: mock_suggestions,
            explanation: Some("This is a mock analysis of your code.".into()),
        })
    }
}

// Singleton wrapper for the AI client
pub struct AIService {
    client: Arc<Mutex<Option<AIClient>>>,
}

impl AIService {
    pub fn new() -> Self {
        AIService {
            client: Arc::new(Mutex::new(None)),
        }
    }
    
    pub async fn initialize(&self, api_key: String, model: String) -> Result<(), String> {
        let mut client_guard = self.client.lock().await;
        *client_guard = Some(AIClient::new(api_key, model));
        Ok(())
    }
    
    pub async fn get_client(&self) -> Result<Arc<AIClient>, String> {
        let client_guard = self.client.lock().await;
        match &*client_guard {
            Some(client) => Ok(Arc::new(client.clone())),
            None => Err("AI client not initialized".into()),
        }
    }
} 