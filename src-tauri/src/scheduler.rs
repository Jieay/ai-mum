use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Emitter, Manager};

use crate::cache;
use crate::models::{AppState, UsageData};
use crate::vendors::zhipu::ZhipuVendor;
use crate::vendors::Vendor;

pub async fn do_refresh(app: &AppHandle) -> Result<Vec<UsageData>, String> {
    let config = {
        let state = app.state::<Mutex<AppState>>();
        let s = state.lock().map_err(|e| e.to_string())?;
        s.config.clone()
    };

    let mut results = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    let zhipu = ZhipuVendor::new();
    if !config.zhipu_api_key.is_empty() {
        match zhipu.fetch_usage(&config.zhipu_api_key).await {
            Ok(data) => results.push(data),
            Err(e) => results.push(UsageData {
                vendor_id: zhipu.id().to_string(),
                vendor_name: zhipu.name().to_string(),
                plan_level: String::new(),
                quotas: vec![],
                last_updated: now.clone(),
                is_error: true,
                error_message: Some(e),
            }),
        }
    }

    {
        let state = app.state::<Mutex<AppState>>();
        let mut s = state.lock().map_err(|e| e.to_string())?;
        s.cached_usage = results.clone();
        s.last_update = Some(now);
        let _ = cache::save_cache(&s.cached_usage);
    }

    let _ = app.emit("usage-updated", &results);

    Ok(results)
}

pub fn start_scheduler(app: AppHandle, interval_secs: u64) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            interval.tick().await;
            let _ = do_refresh(&app).await;
        }
    });
}
