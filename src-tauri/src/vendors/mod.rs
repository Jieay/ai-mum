use crate::models::UsageData;

pub trait Vendor: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    async fn fetch_usage(&self, api_key: &str) -> Result<UsageData, String>;
}

pub mod zhipu;
