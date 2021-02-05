use crate::webapi::client::Client;
use async_trait::async_trait;
use domain::container::{ContainerRepository, ContainerSummary};
use std::error::Error;

pub struct RestfulApiContainerRepository<'a, T: Client> {
    client: &'a T,
}

impl<'a, T: Client> RestfulApiContainerRepository<'a, T> {
    pub fn new(client: &'a T) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<'a, T> ContainerRepository for RestfulApiContainerRepository<'a, T>
where
    T: Client + Send + Sync + 'static,
{
    async fn list(&self) -> Result<Vec<ContainerSummary>, Box<dyn Error + Send + Sync>> {
        let bytes = self.client.get("/containers/json").await?;

        let containers: Vec<types::ContainerSummary> = serde_json::from_slice(&bytes)?;
        let items = containers
            .into_iter()
            .map(|v| ContainerSummary::from_repository(v.id, v.names, v.image, v.created.unwrap(), v.status))
            .collect();
        Ok(items)
    }
}

mod types {
    use crate::shared::date_format;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContainerSummary {
        #[serde(rename = "Id")]
        pub id: String,
        #[serde(rename = "Names")]
        pub names: Vec<String>,
        #[serde(rename = "Image")]
        pub image: String,
        #[serde(rename = "ImageID")]
        pub image_id: String,
        #[serde(rename = "Command")]
        pub command: String,
        #[serde(rename = "Created", with = "date_format")]
        pub created: Option<DateTime<Utc>>,
        #[serde(rename = "State")]
        pub state: String,
        #[serde(rename = "Status")]
        pub status: String,
        #[serde(rename = "Ports")]
        pub ports: Vec<Port>,
        #[serde(rename = "Labels")]
        pub labels: Option<HashMap<String, String>>,
        #[serde(rename = "SizeRw")]
        pub size_rw: i64,
        #[serde(rename = "SizeRootFs")]
        pub size_root_fs: i64,
        #[serde(rename = "HostConfig")]
        pub host_config: HostConfig,
        #[serde(rename = "NetworkSettings")]
        pub network_settings: NetworkSettings,
        #[serde(rename = "Mounts")]
        pub mounts: Vec<Mount>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Port {
        #[serde(rename = "PrivatePort")]
        pub private_port: i64,
        #[serde(rename = "PublicPort")]
        pub public_port: i64,
        #[serde(rename = "Type")]
        pub type_field: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HostConfig {
        #[serde(rename = "NetworkMode")]
        pub network_mode: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NetworkSettings {
        #[serde(rename = "Networks")]
        pub networks: Networks,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Networks {
        pub bridge: Bridge,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Bridge {
        #[serde(rename = "NetworkID")]
        pub network_id: String,
        #[serde(rename = "EndpointID")]
        pub endpoint_id: String,
        #[serde(rename = "Gateway")]
        pub gateway: String,
        #[serde(rename = "IPAddress")]
        pub ipaddress: String,
        #[serde(rename = "IPPrefixLen")]
        pub ipprefix_len: i64,
        #[serde(rename = "IPv6Gateway")]
        pub ipv6_gateway: String,
        #[serde(rename = "GlobalIPv6Address")]
        pub global_ipv6_address: String,
        #[serde(rename = "GlobalIPv6PrefixLen")]
        pub global_ipv6_prefix_len: i64,
        #[serde(rename = "MacAddress")]
        pub mac_address: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Mount {
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Source")]
        pub source: String,
        #[serde(rename = "Destination")]
        pub destination: String,
        #[serde(rename = "Driver")]
        pub driver: String,
        #[serde(rename = "Mode")]
        pub mode: String,
        #[serde(rename = "RW")]
        pub rw: bool,
        #[serde(rename = "Propagation")]
        pub propagation: String,
    }
}
