use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::error::Error;

pub struct Image {
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub created: DateTime<Local>,
    pub size: i32,
    pub labels: HashMap<String, String>,
}

#[async_trait]
pub trait ImageRepository {
    async fn list(&self) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>>;
}
