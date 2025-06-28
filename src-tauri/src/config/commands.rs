use tauri::command;
use tauri::State;
use crate::config::{AppConfig, ConfigManager};

#[command]
pub async fn get_settings(config_manager: State<'_, ConfigManager>) -> Result<AppConfig, String> {
    Ok(config_manager.get_config())
}

#[command]
pub async fn update_settings(config_manager: State<'_, ConfigManager>, config: AppConfig) -> Result<(), String> {
    config_manager.update_config(config)
} 