use clap::Clap;
use hyperlocal::UnixConnector;
use infrastructure::webapi::rest::client::RestApi;
use presentation::image;
use presentation::shared::event::Events;
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

    // Terminal initialization
    let mut terminal = init::terminal()?;
    let events = Events::new();
    let client = RestApi::<UnixConnector>::new("/var/run/docker.sock");

    image::draw(&client, &mut terminal, &events).await?;

    Ok(())
}
