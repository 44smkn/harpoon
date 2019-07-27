#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug)]
struct ContainerDetail {
    Path: String,
}

fn main() {
    println!("Hello, harpoon!");

    let mut args = std::env::args().collect::<VecDeque<String>>();
    let _ = args.pop_front();
    if args.is_empty() {
        println!("failed to parse args");
        std::process::exit(1);
    }

    let (stdout, stderr) = match std::process::Command::new("docker")
        .arg("inspect")
        .args(&args)
        .output()
    {
        Ok(output) => (output.stdout, output.stderr),
        Err(e) => {
            println!("failed. cause is {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    );

    let details: Vec<ContainerDetail> =
        serde_json::from_str(&String::from_utf8(stdout).unwrap()).unwrap();

    println!("{:?}", details);
}
