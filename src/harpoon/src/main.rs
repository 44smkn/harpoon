use clap::Clap;
use presentation::shared::init;
use std::error::Error;

#[allow(dead_code)]
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Kenji S. <xxxxxxxxxx@gmail.com>")]
struct Opts {
    #[clap(short, long, default_value = "1.39")]
    api_version: String,

    #[clap(short, long, default_value = "unix:///var/run/docker.sock")]
    endpoint: String,

    #[clap(short, long, default_value = "0")]
    verbose: i32,
}

#[tokio::main]
#[allow(unused_variables)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let opts: Opts = Opts::parse();

    init::draw_by_default().await?;
    Ok(())
}
