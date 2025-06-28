pub mod commands;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorConfig {
    pub font_family: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub auto_save: bool,
    pub theme: String,
    pub rulers: Vec<u32>,
    pub minimap_enabled: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        EditorConfig {
            font_family: "JetBrains Mono, monospace".into(),
            font_size: 14,
            tab_size: 4,
            insert_spaces: true,
            word_wrap: true,
            auto_save: false,
            theme: "vs-dark".into(),
            rulers: vec![80, 100],
            minimap_enabled: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub show_welcome_page: bool,
    pub recent_projects: Vec<String>,
    pub editor: EditorConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            language: "tr".into(),
            show_welcome_page: true,
            recent_projects: Vec::new(),
            editor: EditorConfig::default(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: Arc<Mutex<AppConfig>>,
}

impl ConfigManager {
    pub fn new(app_handle: &AppHandle) -> Result<Self, String> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create app dir: {}", e))?;
        }

        let config_path = app_dir.join("config.json");
        let config = if config_path.exists() {
            // Load existing config
            let config_str = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            serde_json::from_str::<AppConfig>(&config_str)
                .map_err(|e| format!("Failed to parse config file: {}", e))?
        } else {
            // Create default config
            let default_config = AppConfig::default();
            let config_str = serde_json::to_string_pretty(&default_config)
                .map_err(|e| format!("Failed to serialize config: {}", e))?;
            fs::write(&config_path, config_str)
                .map_err(|e| format!("Failed to write config file: {}", e))?;
            default_config
        };

        Ok(ConfigManager {
            config_path,
            config: Arc::new(Mutex::new(config)),
        })
    }

    pub fn get_config(&self) -> AppConfig {
        self.config.lock().unwrap().clone()
    }

    pub fn update_config(&self, new_config: AppConfig) -> Result<(), String> {
        // Update in memory
        {
            let mut config = self.config.lock().unwrap();
            *config = new_config.clone();
        }

        // Save to disk
        let config_str = serde_json::to_string_pretty(&new_config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(&self.config_path, config_str)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    pub fn add_recent_project(&self, path: &Path) -> Result<(), String> {
        let mut config = self.config.lock().unwrap();
        let path_str = path.to_string_lossy().to_string();

        // Remove existing entry if present
        config.recent_projects.retain(|p| p != &path_str);

        // Add to front of list
        config.recent_projects.insert(0, path_str);

        // Limit to 10 recent projects
        if config.recent_projects.len() > 10 {
            config.recent_projects.truncate(10);
        }

        // Save to disk
        let config_str = serde_json::to_string_pretty(&*config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(&self.config_path, config_str)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
} 