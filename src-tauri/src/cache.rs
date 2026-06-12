use std::fs;
use std::path::PathBuf;

use crate::models::UsageData;

fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".ai-usage-monitor")
}

fn cache_path() -> PathBuf {
    data_dir().join("cache.json")
}

pub fn save_cache(data: &[UsageData]) -> Result<(), String> {
    let dir = data_dir();
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string(data).map_err(|e| e.to_string())?;
    fs::write(cache_path(), json).map_err(|e| e.to_string())
}

pub fn load_cache() -> Option<Vec<UsageData>> {
    let path = cache_path();
    let content = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}
