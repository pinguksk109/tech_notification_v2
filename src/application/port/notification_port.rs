use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationPort {
    async fn send(&self, message: &str) -> Result<()>;
}
