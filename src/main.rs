extern crate clap;
extern crate serde;
extern crate serde_json;
use clap::{App, Arg, SubCommand};
use model::container;
use std::collections::VecDeque;

fn main() {
    let matches = App::new("harpoon")
        .version("1.0")
        .author("44smkn from undergroundyin")
        .about("extract information of container")
        .arg(
            Arg::with_name("output_formats")
                .short("o")
                .long("output")
                .value_name("output_format")
                .possible_values(&["toml", "yaml"])
                .help("To output details to your terminal window in a specific format, you can add either the -o or --output flags")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("container_ids")
                .help("Sets the container id")
                .required(true),
        )
        .get_matches();

    let format = matches.value_of("output_formats").unwrap_or("toml");
    let container_ids = matches.value_of("container_ids").unwrap();
    println!("{}", container_ids);

    /*
    let mut args = std::env::args().collect::<VecDeque<String>>();
    let _ = args.pop_front();
    if args.is_empty() {
        println!("failed to parse args");
        std::process::exit(1);
    }
    */

    let (stdout, stderr) = match std::process::Command::new("docker")
        .arg("inspect")
        .arg(&container_ids)
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
    //dbg!(command_result);

    let details = container::Detail::from_json(&stdout);
    let format = match format {
        "yaml" => container::Format::Yaml,
        "toml" => container::Format::Toml,
        _ => {
            println!("Please specify format type");
            std::process::exit(1);
        }
    };
    let formatted = container::Detail::details_to_string(format, details);
    println!("{}", formatted);
}
