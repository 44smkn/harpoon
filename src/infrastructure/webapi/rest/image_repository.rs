use crate::domain::image::image as domain;
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::client::RestApi;
use async_trait::async_trait;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use futures_util::stream::TryStreamExt;
use hyper::{self, Body};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub struct ImageRepository<'a, T: Client> {
    client: &'a T,
}

impl<'a, T: Client> ImageRepository<'a, T> {
    pub fn new(client: &'a T) -> Self {
        ImageRepository { client }
    }
}

#[async_trait]
impl<'a, T> domain::ImageRepository for ImageRepository<'a, T>
where
    T: Client + Send + Sync + 'static,
{
    async fn list(&self) -> Result<Vec<domain::Image>, Box<dyn Error + Send + Sync>> {
        let bytes = self.client.get("/images/json").await?;

        let images: ListImageOutput = serde_json::from_slice(&bytes)?;
        let mut items: Vec<domain::Image> = Vec::new();

        for image in images.into_iter() {
            let item = domain::Image {
                id: image.id,
                parent_id: image.parent_id,
                repo_tags: image.repo_tags,
                repo_digests: image.repo_digests.unwrap_or_else(|| Vec::new()),
                created: DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(image.created, 0),
                    Utc,
                ),
                size: image.size,
                labels: image.labels.unwrap_or_else(|| HashMap::new()),
            };
            items.push(item);
        }

        Ok(items)
    }
}

pub type ListImageOutput = Vec<Image>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i32,
    #[serde(rename = "SharedSize")]
    pub shared_size: i32,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Containers")]
    pub containers: i32,
}
