use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use tauri::State;

use crate::models::{AppConfig, AppState};

fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".ai-usage-monitor")
}

fn config_path() -> PathBuf {
    data_dir().join("config.json")
}

pub fn load_config_file() -> AppConfig {
    let path = config_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
            return config;
        }
    }
    AppConfig::default()
}

pub fn save_config_file(config: &AppConfig) -> Result<(), String> {
    let dir = data_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path(), json).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_config(state: State<'_, Mutex<AppState>>) -> Result<AppConfig, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    Ok(s.config.clone())
}

#[tauri::command]
pub async fn save_config(
    state: State<'_, Mutex<AppState>>,
    config: AppConfig,
) -> Result<(), String> {
    let config_to_save = config.clone();
    {
        let mut s = state.lock().map_err(|e| e.to_string())?;
        s.config = config;
    }
    save_config_file(&config_to_save)
}
