use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::error::Error;

#[async_trait]
pub trait ContainerRepository {
    async fn list(&self) -> Result<Vec<ContainerSummary>, Box<dyn Error + Send + Sync>>;
}

pub struct ContainerSummary {
    pub id: String,
    pub name: Vec<String>,
    pub image: String,
    pub created: DateTime<Utc>,
    pub status: String,
}
