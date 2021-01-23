use async_trait::async_trait;
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait ImageRepository {
    async fn list(&self) -> Result<Vec<ImageSummary>, Box<dyn Error + Send + Sync>>;
    async fn inspect(&self, id: String) -> Result<Image, Box<dyn Error + Send + Sync>>;
    async fn history(&self, id: String) -> Result<ImageHistory, Box<dyn Error + Send + Sync>>;
}

pub struct ImageSummary {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub created: DateTime<Utc>,
    pub size: i32,
}

impl ImageSummary {
    pub fn from_repository(id: String, repo_tags: Vec<String>, created: i64, size: i32) -> Self {
        Self {
            id,
            repo_tags,
            created: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(created, 0), Utc),
            size,
        }
    }
}

pub struct Image {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub os: String,
    pub architecture: String,
    pub env: Vec<String>,
    pub entrypoint: Vec<String>,
    pub cmd: Vec<String>,
    pub labels: HashMap<String, String>,
}

pub type ImageHistory = Vec<ImageRecord>;

pub struct ImageRecord {
    pub id: String,
    pub created_by: String,
    pub size: i32,
}
