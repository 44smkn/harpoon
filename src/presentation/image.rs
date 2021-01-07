use crate::domain::image::Image;
#[allow(unused_imports)]
use crate::domain::image::ImageRepository as _;
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::image_repository::ImageRepository;
use crate::presentation::shared::event::{Event, Events};
use crate::presentation::shared::tabs;

use crate::presentation::shared::table::StatefulTable;
use crate::usecase::inspect_image::InspectImageUsecase;
use crate::usecase::list_image::ListImageUsecase;
use std::error::Error;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
    Terminal,
};

pub async fn table<T: Client + Send + Sync + 'static>(
    client: &T,
    terminal: &mut Terminal<impl Backend>,
    events: &Events,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let image_repository = ImageRepository::new(client);
    let mut images = ListImageUsecase::new(&image_repository)
        .list_image()
        .await?;
    let items = images_to_table(&mut images);
    let mut table = StatefulTable::new(items);
    let mut tab = tabs::TabsState::new_menu();
    let mut detail_text: Vec<Spans> = vec![Spans::from("It shows container's details here")];

    // Input
    loop {
        terminal.draw(|f| {
            // define layout
            let area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(area[1]);

            tab.draw(f, area[0]);
            let selected_style = Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::DarkGray);
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
            f.render_stateful_widget(t, chunks[0], &mut table.state);

            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "Footer",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));
            let paragraph = Paragraph::new(detail_text.clone())
                .block(block)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, chunks[1]);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    break;
                }
                Key::Down => {
                    table.next();
                    detail_text =
                        gen_detail_text(table.state.selected(), &images, &image_repository).await;
                }
                Key::Up => {
                    table.previous();
                    detail_text =
                        gen_detail_text(table.state.selected(), &images, &image_repository).await;
                }
                Key::Right => {
                    tab.next();
                }
                Key::Left => {
                    tab.previous();
                }
                _ => {}
            }
        };
    }

    Ok(())
}

fn images_to_table(images: &mut Vec<Image>) -> Vec<Vec<String>> {
    let mut items: Vec<Vec<String>> = Vec::new();
    for image in images.into_iter() {
        if &image.repo_tags[0] == "<none>:<none>" {
            continue;
        }
        let mut row: Vec<String> = Vec::new();
        row.push(std::mem::take(&mut image.repo_tags[0]));
        let size = f64::from(image.size) / 1000000.0;
        row.push(format!("{:.2}MB", size));
        row.push(image.created.format("%Y-%m-%d %H:%M:%S").to_string());
        items.push(row);
    }
    items
}

async fn gen_detail_text<'a, T>(
    idx: Option<usize>,
    images: &Vec<Image>,
    image_repository: &'a ImageRepository<'a, T>,
) -> Vec<Spans<'a>>
where
    T: Client + Send + Sync + 'static,
{
    if let Some(v) = idx {
        let image_id = &images[v].id;
        let detail = InspectImageUsecase::new(image_repository)
            .inspect_image(image_id)
            .await;
        match detail {
            Ok(v) => vec![
                Spans::from(format!("id: {}", v.image.id)),
                Spans::from(format!(
                    "digest: {}",
                    v.image.repo_digests.get(0).unwrap_or(&"".to_string())
                )),
                Spans::from(format!("os/arch: {}/{}", v.os, v.architecture)),
                Spans::from(format!("entrypoint: {:?}", v.entrypoint)),
                Spans::from(format!("cmd: {:?}", v.cmd)),
                Spans::from(format!("env: {:?}", v.env)),
                Spans::from(format!("labels: {:?}", v.image.labels)),
                Spans::from(""),
                Spans::from("history:"),
                Spans::from(format!(
                    "{}  {}  {}",
                    v.history[0].id, v.history[0].created_by, v.history[0].size
                )),
            ],
            Err(e) => vec![Spans::from(format!(
                "Failed to get container's details: {}",
                e
            ))],
        }
    } else {
        vec![Spans::from("It shows container's details here")]
    }
}
