pub mod commands;
pub mod syntax;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Represents an open document in the editor
#[derive(Debug)]
pub struct Document {
    pub path: Option<PathBuf>,
    pub language_id: String,
    pub version: i32,
    pub content: String,
    pub dirty: bool,
}

impl Document {
    pub fn new(path: Option<PathBuf>, language_id: &str, content: &str) -> Self {
        Document {
            path,
            language_id: language_id.to_string(),
            version: 1,
            content: content.to_string(),
            dirty: false,
        }
    }
    
    pub fn update_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.version += 1;
        self.dirty = true;
    }
    
    pub fn save(&mut self) {
        self.dirty = false;
    }
}

// Manages open documents in the editor
pub struct DocumentManager {
    documents: Arc<Mutex<HashMap<String, Document>>>,
}

impl DocumentManager {
    pub fn new() -> Self {
        DocumentManager {
            documents: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn open_document(&self, uri: &str, path: Option<PathBuf>, language_id: &str, content: &str) -> Result<(), String> {
        let mut documents = self.documents.lock().unwrap();
        let document = Document::new(path, language_id, content);
        documents.insert(uri.to_string(), document);
        Ok(())
    }
    
    pub fn get_document(&self, uri: &str) -> Option<Document> {
        let documents = self.documents.lock().unwrap();
        documents.get(uri).cloned()
    }
    
    pub fn update_document(&self, uri: &str, content: &str) -> Result<(), String> {
        let mut documents = self.documents.lock().unwrap();
        if let Some(doc) = documents.get_mut(uri) {
            doc.update_content(content);
            Ok(())
        } else {
            Err(format!("Document not found: {}", uri))
        }
    }
    
    pub fn close_document(&self, uri: &str) -> Result<(), String> {
        let mut documents = self.documents.lock().unwrap();
        if documents.remove(uri).is_some() {
            Ok(())
        } else {
            Err(format!("Document not found: {}", uri))
        }
    }
} 