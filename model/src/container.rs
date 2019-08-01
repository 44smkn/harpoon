use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Detail {
    pub path: String,
    args: Vec<String>,
    state: ContainerState,
    mounts: Vec<Mount>,
    config: ContainerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerState {
    status: String,
    exit_code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    name: String,
    source: String,
    destination: String,
    driver: String,

    #[serde(rename = "Type")]
    mount_type: String,

    #[serde(rename = "RW")]
    rw: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    env: Vec<String>,
    cmd: Vec<String>,
    image: String,
    working_dir: String,
    entrypoint: Option<Vec<String>>,
    labels: HashMap<String, String>,
}

pub enum Format {
    Json,
    Yaml,
    Toml,
}

pub fn new_from_json(json: &String) -> Vec<Detail> {
    serde_json::from_str(json).expect("failed to parse from json to object")
}

impl Format {
    pub fn to_string(&self, containers: &Vec<Detail>) -> String {
        match self {
            Format::Json => serde_json::to_string_pretty(&containers)
                .expect("failed to serialize from object to json"),
            Format::Yaml => {
                serde_yaml::to_string(&containers).expect("failed to serialize from object to yaml")
            }
            Format::Toml => {
                toml::to_string(&containers).expect("failed to serialize from object to toml")
            }
        }
    }
}
