use crate::image::tui_controller::ImageTuiController;
use crate::shared::event::Events;
use infrastructure::webapi::rest::client;
use infrastructure::webapi::rest::image_repository::RestfulApiImageRepository;
use std::error::Error;
use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{Backend, TermionBackend},
    Terminal,
};
use usecase::{inspect_image::InspectImageUsecase, list_image::ListImageUsecase};

pub async fn draw_by_default() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Terminal initialization
    let mut terminal = terminal()?;
    let events = Events::new();

    // image
    let client = client::new_restapi_client("/var/run/docker.sock");
    let image_repository = RestfulApiImageRepository::new(&client);
    let list_image_usecase = ListImageUsecase::new(&image_repository);
    let inspect_image_usecase = InspectImageUsecase::new(&image_repository);
    let image_controller = ImageTuiController::new(&list_image_usecase, &inspect_image_usecase);
    image_controller.draw(&mut terminal, &events).await?;
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
