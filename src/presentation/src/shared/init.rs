use crate::image;
use crate::shared::event::Events;
use hyperlocal::UnixConnector;
use infrastructure::webapi::rest::client::RestApi;
use std::error::Error;
use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};

pub async fn draw_by_default() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Terminal initialization
    let mut terminal = terminal()?;
    let events = Events::new();
    let client = RestApi::<UnixConnector>::new("/var/run/docker.sock");
    image::draw(&client, &mut terminal, &events).await?;
    Ok(())
}

fn terminal() -> Result<Terminal<impl Backend>, Box<dyn Error + Send + Sync>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}
