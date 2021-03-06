use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Client {
    async fn get(&self, path: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
}
