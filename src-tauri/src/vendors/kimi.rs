use serde_json::Value;

use crate::models::{QuotaInfo, UsageData};
use crate::vendors::Vendor;

pub struct KimiVendor;

impl KimiVendor {
    pub fn new() -> Self {
        Self
    }
}

impl Vendor for KimiVendor {
    fn id(&self) -> &str {
        "kimi"
    }

    fn name(&self) -> &str {
        "Kimi Code"
    }

    async fn fetch_usage(&self, api_key: &str) -> Result<UsageData, String> {
        if api_key.is_empty() {
            return Err("API key not configured".to_string());
        }

        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.kimi.com/coding/v1/usages")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("HTTP error: {}", resp.status()));
        }

        let raw = resp
            .text()
            .await
            .map_err(|e| format!("Read body error: {}", e))?;

        let body: Value = serde_json::from_str(&raw)
            .map_err(|e| format!("Parse error: {} | raw: {}", e, &raw[..raw.len().min(500)]))?;

        let now = chrono::Utc::now().to_rfc3339();
        let plan_level = parse_plan_level(&body);
        let mut quotas = Vec::new();

        if let Some(weekly) = body.get("usage") {
            let limit = parse_number(weekly.get("limit"));
            let used = parse_number(weekly.get("used"));
            let remaining = parse_number(weekly.get("remaining"));
            let reset_time = parse_string(weekly.get("resetTime"));
            quotas.push(QuotaInfo {
                quota_type: "weekly".to_string(),
                label: "每周额度".to_string(),
                used,
                total: limit,
                remaining,
                percentage: percentage(used, limit),
                reset_time,
            });
        }

        if let Some(limits) = body.get("limits").and_then(|v| v.as_array()) {
            for limit in limits {
                let window = limit.get("window").and_then(|v| v.as_object());
                let detail = limit.get("detail").unwrap_or(limit);
                if !is_five_hour_window(window) {
                    continue;
                }
                let used = parse_number(detail.get("used"));
                let total = parse_number(detail.get("limit"));
                let remaining = parse_number(detail.get("remaining"));
                let reset_time = detail
                    .get("resetTime")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                quotas.push(QuotaInfo {
                    quota_type: "5hour".to_string(),
                    label: "5 小时额度".to_string(),
                    used,
                    total,
                    remaining,
                    percentage: percentage(used, total),
                    reset_time,
                });
            }
        }

        Ok(UsageData {
            vendor_id: "kimi".to_string(),
            vendor_name: "Kimi Code".to_string(),
            plan_level,
            quotas,
            last_updated: now,
            is_error: false,
            error_message: None,
        })
    }
}

fn parse_plan_level(body: &Value) -> String {
    let raw = body
        .get("user")
        .and_then(|u| u.get("membership"))
        .and_then(|m| m.get("level"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let level = raw.strip_prefix("LEVEL_").unwrap_or(raw).to_lowercase();
    match level.as_str() {
        "intermediate" => "allegretto".to_string(),
        _ => level,
    }
}

fn is_five_hour_window(window: Option<&serde_json::Map<String, Value>>) -> bool {
    let Some(window) = window else { return false };
    let duration = parse_number(window.get("duration"));
    let time_unit = window
        .get("timeUnit")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    duration == 300.0 && time_unit.contains("MINUTE")
}

fn parse_number(value: Option<&Value>) -> f64 {
    value
        .and_then(|v| v.as_f64())
        .or_else(|| value.and_then(|v| v.as_str()).and_then(|s| s.parse().ok()))
        .unwrap_or(0.0)
}

fn parse_string(value: Option<&Value>) -> Option<String> {
    value.and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn percentage(used: f64, total: f64) -> f64 {
    if total > 0.0 {
        (used / total) * 100.0
    } else {
        0.0
    }
}
