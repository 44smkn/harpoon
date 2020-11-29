mod domain;
mod infrastructure;
mod presentation;
mod usecase;

use crate::domain::image::image::ImageRepository as _;
use crate::infrastructure::webapi::rest::client::RestApi;
use crate::infrastructure::webapi::rest::image_repository::ImageRepository;
use crate::presentation::shared::event::{Event, Events};
use crate::usecase::list_image::ListImageUsecase;
use clap::Clap;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table, TableState},
    Terminal,
};

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
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let opts: Opts = Opts::parse();
    println!("Value for config: {}", opts.api_version);

    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let client = RestApi::<UnixConnector>::new("/var/run/docker.sock");
    let image_repository = ImageRepository::new(&client);
    let items: Vec<Vec<String>> = ListImageUsecase::new(&image_repository)
        .list_image()
        .await?;
    let mut table = StatefulTable::new(items);

    // Input
    loop {
        terminal.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());

            let selected_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);
            let header = ["NAME", "SIZE", "CREATED"];
            let rows = table
                .items
                .iter()
                .map(|i| Row::StyledData(i.iter(), normal_style));
            let t = Table::new(header.iter(), rows)
                .block(Block::default().borders(Borders::ALL).title("Images"))
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Max(10),
                ]);
            f.render_stateful_widget(t, rects[0], &mut table.state);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                _ => {}
            }
        };
    }

    Ok(())
}

pub struct StatefulTable {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl<'a> StatefulTable {
    fn new(items: Vec<Vec<String>>) -> StatefulTable {
        StatefulTable {
            state: TableState::default(),
            items: items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
