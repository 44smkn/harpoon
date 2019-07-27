use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerDetail {
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
