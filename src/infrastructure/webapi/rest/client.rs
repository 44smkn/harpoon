use crate::infrastructure::webapi::client::Client;
use async_trait::async_trait;
use chrono::prelude::*;
use futures_util::stream::TryStreamExt;
use hyper::{self, Body};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub struct RestApi {
    client: hyper::Client<UnixConnector, Body>,
    unix_socket: String,
    // 認証情報とかを後で追加する
}

impl RestApi {
    pub fn new(unix_socket_path: &str) -> RestApi {
        let client = hyper::Client::unix();
        let unix_socket = unix_socket_path.to_string();
        RestApi {
            client,
            unix_socket,
        }
    }
}

#[async_trait]
impl Client for RestApi {
    async fn get(&self, path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
        let url = Uri::new(&self.unix_socket, path).into();
        let response_body = self.client.get(url).await?.into_body();

        let bytes = response_body
            .try_fold(Vec::default(), |mut buf, bytes| async {
                buf.extend(bytes);
                Ok(buf)
            })
            .await?;

        let images: ListImageOutput = serde_json::from_slice(&bytes)?;
        let mut items: Vec<Vec<String>> = Vec::new();

        for image in images.iter() {
            if &image.repo_tags[0] == "<none>:<none>" {
                continue;
            }
            let mut row: Vec<String> = Vec::new();
            row.push(image.repo_tags[0].clone());
            let size = f64::from(image.size) / 1000000.0;
            row.push(format!("{:.2}MB", size));
            let created_date = NaiveDateTime::from_timestamp(image.created, 0);
            row.push(created_date.format("%Y-%m-%d %H:%M:%S").to_string());
            items.push(row);
        }

        //let items = vec![vec!["image".to_string(), "32".to_string()]];
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
