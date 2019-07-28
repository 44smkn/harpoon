use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    Path: String,
    Args: Vec<String>,
    State: ContainerState,
    Mounts: Vec<Mount>,
    Config: ContainerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerState {
    Status: String,
    ExitCode: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    Type: String,
    Name: String,
    Source: String,
    Destination: String,
    Driver: String,
    RW: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerConfig {
    Env: Vec<String>,
    Cmd: Vec<String>,
    Image: String,
    WorkingDir: String,
    Entrypoint: Option<Vec<String>>,
    Labels: HashMap<String, String>,
}

pub enum Format {
    Yaml,
    Toml,
}

impl Detail {
    pub fn from_json(json: &String) -> Vec<Detail> {
        match serde_json::from_str(json) {
            Ok(value) => value,
            Err(e) => {
                println!("failed to parse from json to object. cause:\n{}", e);
                std::process::exit(1);
            }
        }
    }

    pub fn details_to_string(fmt: Format, containers: Vec<Detail>) -> String {
        match fmt {
            Format::Yaml => {
                match serde_yaml::to_string(&containers) {
                    Ok(value) => value,
                    Err(e) => {
                        println!("failed to serialize from object to yaml. cause:\n{}", e);
                        std::process::exit(1);
                    },
                }
            },
            Format::Toml => {
                match toml::to_string(&containers) {
                    Ok(value) => value,
                    Err(e) => {
                        println!("failed to serialize from object to toml. cause:\n{}", e);
                        std::process::exit(1);
                    },
                }
            },
        }
    }
}
