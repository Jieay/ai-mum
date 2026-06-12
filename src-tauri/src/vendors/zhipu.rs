use serde_json::Value;

use crate::models::{QuotaInfo, UsageData};
use crate::vendors::Vendor;

pub struct ZhipuVendor;

impl ZhipuVendor {
    pub fn new() -> Self {
        Self
    }
}

impl Vendor for ZhipuVendor {
    fn id(&self) -> &str {
        "zhipu"
    }

    fn name(&self) -> &str {
        "智谱 GLM"
    }

    async fn fetch_usage(&self, api_key: &str) -> Result<UsageData, String> {
        if api_key.is_empty() {
            return Err("API key not configured".to_string());
        }

        let client = reqwest::Client::new();
        let resp = client
            .get("https://open.bigmodel.cn/api/monitor/usage/quota/limit")
            .header("Authorization", api_key)
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

        if !body
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let msg = body
                .get("msg")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return Err(msg.to_string());
        }

        let data = body.get("data").ok_or("No data in response")?;
        let level = data
            .get("level")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let limits = data
            .get("limits")
            .and_then(|v| v.as_array())
            .ok_or("No limits in response")?;

        let now = chrono::Utc::now().to_rfc3339();

        let mut tokens_limits: Vec<&Value> = limits
            .iter()
            .filter(|l| l.get("type").and_then(|v| v.as_str()) == Some("TOKENS_LIMIT"))
            .collect();

        tokens_limits.sort_by(|a, b| {
            let a_time = a.get("nextResetTime").and_then(|v| v.as_i64()).unwrap_or(0);
            let b_time = b.get("nextResetTime").and_then(|v| v.as_i64()).unwrap_or(0);
            a_time.cmp(&b_time)
        });

        let mut quotas = Vec::new();

        if let Some(five_hour) = tokens_limits.first() {
            let pct = five_hour
                .get("percentage")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let reset_ts = five_hour.get("nextResetTime").and_then(|v| v.as_i64());
            let reset_time = reset_ts.map(|ts| {
                chrono::DateTime::from_timestamp(ts / 1000, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            });
            quotas.push(QuotaInfo {
                quota_type: "5hour".to_string(),
                label: "5 小时额度".to_string(),
                used: pct,
                total: 100.0,
                remaining: 100.0 - pct,
                percentage: pct,
                reset_time,
            });
        }

        if let Some(weekly) = tokens_limits.get(1) {
            let pct = weekly
                .get("percentage")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let reset_ts = weekly.get("nextResetTime").and_then(|v| v.as_i64());
            let reset_time = reset_ts.map(|ts| {
                chrono::DateTime::from_timestamp(ts / 1000, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_default()
            });
            quotas.push(QuotaInfo {
                quota_type: "weekly".to_string(),
                label: "每周额度".to_string(),
                used: pct,
                total: 100.0,
                remaining: 100.0 - pct,
                percentage: pct,
                reset_time,
            });
        }

        for limit in limits
            .iter()
            .filter(|l| l.get("type").and_then(|v| v.as_str()) == Some("TIME_LIMIT"))
        {
            let total = limit.get("usage").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let used = limit
                .get("currentValue")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let remaining = limit
                .get("remaining")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            let pct = if total > 0.0 {
                (used / total) * 100.0
            } else {
                0.0
            };
            quotas.push(QuotaInfo {
                quota_type: "mcp_monthly".to_string(),
                label: "MCP 工具调用（月）".to_string(),
                used,
                total,
                remaining,
                percentage: pct,
                reset_time: None,
            });
        }

        Ok(UsageData {
            vendor_id: "zhipu".to_string(),
            vendor_name: "智谱 GLM".to_string(),
            plan_level: level,
            quotas,
            last_updated: now,
            is_error: false,
            error_message: None,
        })
    }
}
