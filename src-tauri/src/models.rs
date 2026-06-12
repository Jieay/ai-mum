use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaInfo {
    pub quota_type: String,
    pub label: String,
    pub used: f64,
    pub total: f64,
    pub remaining: f64,
    pub percentage: f64,
    pub reset_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub vendor_id: String,
    pub vendor_name: String,
    pub plan_level: String,
    pub quotas: Vec<QuotaInfo>,
    pub last_updated: String,
    pub is_error: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub zhipu_api_key: String,
    pub kimi_api_key: String,
    pub refresh_interval_secs: u64,
    pub notification_threshold: f64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            zhipu_api_key: String::new(),
            kimi_api_key: String::new(),
            refresh_interval_secs: 300,
            notification_threshold: 20.0,
        }
    }
}

pub struct AppState {
    pub cached_usage: Vec<UsageData>,
    pub config: AppConfig,
    pub last_update: Option<String>,
}
