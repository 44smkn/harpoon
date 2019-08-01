#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

pub mod container;

#[cfg(test)]
mod tests {
    use super::container;

    #[test]
    fn deserilaze_json() {
        let json = r#"[
{
    "Path": "docker-entrypoint.sh",
    "Args": [
      "mysqld"
    ],
    "State": {
      "Status": "running",
      "ExitCode": 0
    },
    "Mounts": [
      {
        "Name": "7740838474d47599f7c3465c21f54703c75a6466b71d464109805cfcb9ca209e",
        "Source": "/var/lib/docker/volumes/7740838474d47599f7c3465c21f54703c75a6466b71d464109805cfcb9ca209e/_data",
        "Destination": "/var/lib/mysql",
        "Driver": "local",
        "Type": "volume",
        "RW": true
      }
    ],
    "Config": {
      "Env": [
        "MYSQL_ROOT_PASSWORD=mochoten",
        "MYSQL_DATABASE=sealion",
        "MYSQL_USER=kenji",
        "MYSQL_PASSWORD=kenji",
        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        "GOSU_VERSION=1.7",
        "MYSQL_MAJOR=8.0",
        "MYSQL_VERSION=8.0.15-1debian9"
      ],
      "Cmd": [
        "mysqld"
      ],
      "Image": "mysql:8.0.15",
      "WorkingDir": "",
      "Entrypoint": [
        "docker-entrypoint.sh"
      ],
      "Labels": {
        "com.docker.compose.container-number": "1",
        "com.docker.compose.service": "mysql",
        "com.docker.compose.config-hash": "406474a2af57d865d8618b75760a9b4cb7f4739e21004e4c53fd667cb2261b79",
        "com.docker.compose.version": "1.23.2",
        "com.docker.compose.oneoff": "False",
        "com.docker.compose.project": "sealion"
      }
    }
  }
]"#;
        let contaieners = container::new_from_json(&String::from(json));
        assert_eq!(contaieners.get(0).unwrap().path, "docker-entrypoint.sh");
    }
}
