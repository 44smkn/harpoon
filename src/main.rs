extern crate serde;
extern crate serde_json;
use model::container;
use std::collections::VecDeque;

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
            println!("failed. cause:\n{}", e);
            std::process::exit(1);
        }
    };

    let (stdout, stderr) = match String::from_utf8(stdout) {
        Ok(value) => (value, String::from_utf8(stderr).unwrap()),
        Err(e) => {
            println!("failed to parse from Vec<u8> to utf8. cause:\n{}", e);
            std::process::exit(1);
        }
    };

    let command_result = if String::is_empty(&stderr) {
        String::from(&stdout)
    } else {
        stderr
    };
    dbg!(command_result);

    let details = container::Detail::from_json(&stdout);
    let formatted = container::Detail::details_to_string(container::Format::Yaml, details);
    println!("{}", formatted);
}
