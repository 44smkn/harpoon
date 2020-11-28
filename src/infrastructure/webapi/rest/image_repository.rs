use crate::domain::image::image as domain;
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::client::RestApi;
use async_trait::async_trait;
use chrono::prelude::*;
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
    async fn list(&self) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
        let bytes = self.client.get("/images/json").await?;

        let images: ListImageOutput = serde_json::from_slice(&bytes)?;
        let mut items: Vec<Vec<String>> = Vec::new();

        for mut image in images.into_iter() {
            if &image.repo_tags[0] == "<none>:<none>" {
                continue;
            }
            let mut row: Vec<String> = Vec::new();
            row.push(std::mem::replace(
                &mut image.repo_tags[0],
                Default::default(),
            ));
            let size = f64::from(image.size) / 1000000.0;
            row.push(format!("{:.2}MB", size));
            let created_date = NaiveDateTime::from_timestamp(image.created, 0);
            row.push(created_date.format("%Y-%m-%d %H:%M:%S").to_string());
            items.push(row);
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
