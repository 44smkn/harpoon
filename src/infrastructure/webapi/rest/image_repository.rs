use crate::domain::image as domain;
use crate::infrastructure::webapi::client::Client;
use async_trait::async_trait;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::str;

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
                repo_digests: image.repo_digests.unwrap_or_else(Vec::new),
                created: DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(image.created, 0),
                    Utc,
                ),
                size: image.size,
                labels: image.labels.unwrap_or_else(HashMap::new),
            };
            items.push(item);
        }

        Ok(items)
    }

    async fn inspect(
        &self,
        id: String,
    ) -> Result<domain::ImageDetail, Box<dyn Error + Send + Sync>> {
        let bytes = self.client.get(&format!("/images/{}/json", id)).await?;
        let detail: InspectImageOutput = serde_json::from_slice(&bytes)?;
        let bytes = self.client.get(&format!("/images/{}/history", id)).await?;
        let records: ImageHistory = serde_json::from_slice(&bytes)?;
        let mut items: domain::ImageHistory = Vec::new();
        for record in records.into_iter() {
            let item = domain::ImageRecord {
                id: record.id,
                created_by: record.created_by,
                size: record.size,
            };
            items.push(item);
        }

        Ok(domain::ImageDetail {
            image: domain::Image {
                id: detail.id,
                parent_id: detail.parent,
                repo_tags: detail.repo_tags,
                repo_digests: detail.repo_digests,
                created: DateTime::<Utc>::from(
                    DateTime::parse_from_rfc3339(&detail.created).unwrap(),
                ),
                size: detail.size,
                labels: detail.container_config.labels.unwrap_or_else(HashMap::new),
            },
            os: detail.os,
            architecture: detail.architecture,
            env: detail.config.env,
            entrypoint: detail.config.entrypoint,
            cmd: detail.config.cmd.unwrap_or(vec!["".to_string()]),
            history: items,
        })
    }

    async fn history(
        &self,
        id: String,
    ) -> Result<domain::ImageHistory, Box<dyn Error + Send + Sync>> {
        let bytes = self.client.get(&format!("/images/{}/history", id)).await?;
        let records: ImageHistory = serde_json::from_slice(&bytes)?;
        let mut items: domain::ImageHistory = Vec::new();
        for record in records.into_iter() {
            let item = domain::ImageRecord {
                id: record.id,
                created_by: record.created_by,
                size: record.size,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InspectImageOutput {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Container")]
    pub container: String,
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Os")]
    pub os: String,
    #[serde(rename = "Architecture")]
    pub architecture: String,
    #[serde(rename = "Parent")]
    pub parent: String,
    #[serde(rename = "ContainerConfig")]
    pub container_config: ContainerConfig,
    #[serde(rename = "DockerVersion")]
    pub docker_version: String,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i32,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: GraphDriver,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "Config")]
    pub config: Config,
    #[serde(rename = "RootFS")]
    pub root_fs: RootFs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerConfig {
    #[serde(rename = "Tty")]
    pub tty: bool,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Domainname")]
    pub domainname: String,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,
    #[serde(rename = "PublishService", default)]
    pub publish_service: String,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,
    #[serde(rename = "NetworkDisabled", default)]
    pub network_disabled: bool,
    #[serde(rename = "OnBuild")]
    pub on_build: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "WorkingDir")]
    pub working_dir: String,
    #[serde(rename = "MacAddress", default)]
    pub mac_address: String,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDriver {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "NetworkDisabled", default)]
    pub network_disabled: bool,
    #[serde(rename = "OnBuild")]
    pub on_build: Option<Vec<::serde_json::Value>>,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,
    #[serde(rename = "PublishService", default)]
    pub publish_service: String,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,
    #[serde(rename = "Domainname")]
    pub domainname: String,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,
    #[serde(rename = "Tty")]
    pub tty: bool,
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacAddress", default)]
    pub mac_address: String,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,
    #[serde(rename = "WorkingDir")]
    pub working_dir: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFs {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Layers")]
    pub layers: Vec<String>,
}

pub type ImageHistory = Vec<ImageRecord>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageRecord {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "Comment")]
    pub comment: String,
}
