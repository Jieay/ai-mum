use std::sync::Mutex;

use tauri::{AppHandle, State};

use crate::models::{AppState, UsageData};
use crate::scheduler;

#[tauri::command]
pub async fn get_usage(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<UsageData>, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    Ok(s.cached_usage.clone())
}

#[tauri::command]
pub async fn refresh_usage(app: AppHandle) -> Result<Vec<UsageData>, String> {
    scheduler::do_refresh(&app).await
}

#[tauri::command]
pub async fn get_last_update_time(
    state: State<'_, Mutex<AppState>>,
) -> Result<Option<String>, String> {
    let s = state.lock().map_err(|e| e.to_string())?;
    Ok(s.last_update.clone())
}
