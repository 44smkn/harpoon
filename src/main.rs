extern crate clap;
extern crate serde;
extern crate serde_json;
use model::container;

mod init;

fn main() {
    let app = init::new_app();
    let matches = app.get_matches();
    let format = matches.value_of("output_formats").unwrap_or("toml");
    let container_ids = matches
        .values_of("container_ids")
        .unwrap()
        .collect::<Vec<_>>();

    let output = std::process::Command::new("docker")
        .arg("inspect")
        .args(&container_ids)
        .output()
        .unwrap_or_else(|e| panic!("failed. cause:\n{}", e));

    let (stdout, stderr) = (
        String::from_utf8(output.stdout).unwrap_or_else(|e| {
            panic!("failed to parse stdout from Vec<u8> to utf8. cause:\n{}", e)
        }),
        String::from_utf8(output.stderr).unwrap_or_else(|e| {
            panic!("failed to parse stderr from Vec<u8> to utf8. cause:\n{}", e)
        }),
    );

    let command_result = if String::is_empty(&stderr) {
        String::from(&stdout)
    } else {
        stderr
    };
    //dbg!(command_result);

    let details = container::new_from_json(&stdout);
    let format = match format {
        "yaml" => container::Format::Yaml,
        "toml" => container::Format::Toml,
        _ => {
            println!("Please specify format type");
            std::process::exit(1);
        }
    };
    let formatted = format.to_string(&details);
    println!("{}", formatted);
}
