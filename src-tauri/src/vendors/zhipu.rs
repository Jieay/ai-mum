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

        let mut five_hour_limit: Option<&Value> = None;
        let mut weekly_limit: Option<&Value> = None;
        let mut mcp_limit: Option<&Value> = None;

        for limit in limits.iter() {
            match detect_quota_type(limit) {
                Some("5hour") if five_hour_limit.is_none() => five_hour_limit = Some(limit),
                Some("weekly") if weekly_limit.is_none() => weekly_limit = Some(limit),
                Some("mcp_monthly") if mcp_limit.is_none() => mcp_limit = Some(limit),
                _ => {}
            }
        }

        let mut quotas = Vec::new();

        if let Some(five_hour) = five_hour_limit {
            quotas.push(build_token_quota_info(five_hour, "5hour", "5 小时额度"));
        }

        if let Some(weekly) = weekly_limit {
            quotas.push(build_token_quota_info(weekly, "weekly", "每周额度"));
        }

        if let Some(mcp) = mcp_limit {
            quotas.push(build_mcp_quota_info(mcp));
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

fn detect_quota_type(limit: &Value) -> Option<&'static str> {
    let unit = limit.get("unit").and_then(|v| v.as_i64())?;
    let number = limit.get("number").and_then(|v| v.as_i64())?;
    match (unit, number) {
        (6, 1) => Some("weekly"),
        (3, 5) => Some("5hour"),
        (5, 1) => Some("mcp_monthly"),
        _ => None,
    }
}

fn build_token_quota_info(limit: &Value, quota_type: &str, label: &str) -> QuotaInfo {
    let pct = limit
        .get("percentage")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let reset_ts = limit.get("nextResetTime").and_then(|v| v.as_i64());
    let reset_time = reset_ts.map(|ts| {
        chrono::DateTime::from_timestamp(ts / 1000, 0)
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default()
    });
    QuotaInfo {
        quota_type: quota_type.to_string(),
        label: label.to_string(),
        used: pct,
        total: 100.0,
        remaining: 100.0 - pct,
        percentage: pct,
        reset_time,
    }
}

fn build_mcp_quota_info(limit: &Value) -> QuotaInfo {
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
    QuotaInfo {
        quota_type: "mcp_monthly".to_string(),
        label: "MCP 工具调用（月）".to_string(),
        used,
        total,
        remaining,
        percentage: pct,
        reset_time: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn detect_quota_type_by_unit_and_number() {
        assert_eq!(detect_quota_type(&json!({"unit": 6, "number": 1})), Some("weekly"));
        assert_eq!(detect_quota_type(&json!({"unit": 3, "number": 5})), Some("5hour"));
        assert_eq!(detect_quota_type(&json!({"unit": 5, "number": 1})), Some("mcp_monthly"));
        assert_eq!(detect_quota_type(&json!({"unit": 6, "number": 5})), None);
        assert_eq!(detect_quota_type(&json!({})), None);
    }
}
