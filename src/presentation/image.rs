#[allow(unused_imports)]
use crate::domain::image::ImageRepository as _;
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::image_repository::ImageRepository;
use crate::presentation::shared::event::{Event, Events};
use crate::presentation::shared::tabs;

use crate::presentation::shared::table::StatefulTable;
use crate::usecase::list_image::ListImageUsecase;
use std::error::Error;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};

pub async fn table<T: Client + Send + Sync + 'static>(
    client: &T,
    terminal: &mut Terminal<impl Backend>,
    events: &Events,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let image_repository = ImageRepository::new(client);
    let items: Vec<Vec<String>> = ListImageUsecase::new(image_repository).list_image().await?;
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

            tabs::draw(f)
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
