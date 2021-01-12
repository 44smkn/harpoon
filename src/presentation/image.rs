use crate::domain::image::Image;
#[allow(unused_imports)]
use crate::domain::image::ImageRepository as _;
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::image_repository::ImageRepository;
use crate::presentation::shared::{
    event::{Event, Events},
    span,
    table::StatefulTable,
    tabs,
};

use crate::usecase::{
    get_image_history::GetImageHistoryUsecase, inspect_image::InspectImageUsecase,
    list_image::ListImageUsecase,
};
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
    let mut histories: Vec<Vec<String>> = Vec::new();

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

            let detail_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[1]);
            let block = Block::default().borders(Borders::ALL).title(Span::styled(
                "Detail",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ));
            let paragraph = Paragraph::new(detail_text.clone())
                .block(block)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, detail_area[0]);

            let history_table = histories
                .iter()
                .map(|i| Row::StyledData(i.iter(), normal_style));
            let header = vec!["IMAGE ID", "CREATED BY", "SIZE"];
            let image_history = Table::new(header.iter(), history_table)
                .block(Block::default().borders(Borders::ALL).title("History"))
                .widths(&[
                    Constraint::Percentage(30),
                    Constraint::Percentage(60),
                    Constraint::Percentage(10),
                ])
                .column_spacing(1);
            f.render_widget(image_history, detail_area[1]);
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
                    histories =
                        gen_history_text(table.state.selected(), &images, &image_repository).await;
                }
                Key::Up => {
                    table.previous();
                    detail_text =
                        gen_detail_text(table.state.selected(), &images, &image_repository).await;
                    histories =
                        gen_history_text(table.state.selected(), &images, &image_repository).await;
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
            Ok(v) => span::from_texts(vec![
                format!("id: {}", v.image.id),
                format!(
                    "digest: {}",
                    v.image.repo_digests.get(0).unwrap_or(&"".to_string())
                ),
                format!("os/arch: {}/{}", v.os, v.architecture),
                format!("entrypoint: {:?}", v.entrypoint),
                format!("cmd: {:?}", v.cmd),
                format!("env: {:?}", v.env),
                format!("labels: {:?}", v.image.labels),
            ]),
            Err(e) => span::from_texts(vec![format!("Failed to get container's details: {}", e)]),
        }
    } else {
        span::from_texts(vec!["It shows container's details here"])
    }
}

async fn gen_history_text<'a, T>(
    idx: Option<usize>,
    images: &Vec<Image>,
    image_repository: &'a ImageRepository<'a, T>,
) -> Vec<Vec<String>>
where
    T: Client + Send + Sync + 'static,
{
    if let Some(v) = idx {
        let image_id = &images[v].id;
        let history = GetImageHistoryUsecase::new(image_repository)
            .get_history(image_id)
            .await;
        match history {
            Ok(v) => v
                .into_iter()
                .map(|r| {
                    vec![
                        r.id.split(':')
                            .collect::<Vec<&str>>()
                            .get(1)
                            .map_or("none", |v| *v)
                            .to_string(),
                        r.created_by,
                        r.size.to_string(),
                    ]
                })
                .collect(),
            Err(e) => vec![],
        }
    } else {
        vec![]
    }
}
