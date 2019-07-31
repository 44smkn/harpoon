use clap::{App, Arg};

pub fn new_app<'a, 'b>() -> App<'a, 'b> {
    App::new("harpoon")
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
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
}
