#[allow(unused_imports)]
use crate::domain::image::ImageRepository as _;
use crate::domain::image::{Image, ImageDetail};
use crate::infrastructure::webapi::client::Client;
use crate::infrastructure::webapi::rest::image_repository::ImageRepository;
use crate::presentation::shared::{
    event::{Event, Events},
    layout, span,
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
    layout::Constraint,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
    Terminal,
};

pub async fn draw<T: Client + Send + Sync + 'static>(
    client: &T,
    terminal: &mut Terminal<impl Backend>,
    events: &Events,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let image_repository = ImageRepository::new(client);
    let mut images = ListImageUsecase::new(&image_repository)
        .list_image()
        .await?;
    let items = images_to_table(&mut images);
    let header = vec!["NAME", "SIZE", "CREATED"];
    let widths = vec![
        Constraint::Percentage(10),
        Constraint::Percentage(75),
        Constraint::Percentage(15),
    ];
    let mut table = StatefulTable::new(items, "Images", header, widths);
    let mut tab = tabs::TabsState::new_menu();
    let mut detail_text: Vec<Spans> = vec![Spans::from("It shows container's details here")];
    let mut histories: Vec<Vec<String>> = Vec::new();

    // Input
    loop {
        terminal.draw(|f| {
            // TODO: Change it when split assignments are included in Rust's standard functions.
            let areas = layout::split_into_header_and_main(f);
            let header = areas.0;
            let main = areas.1;
            tab.draw(f, header);

            // TODO: Change it when split assignments are included in Rust's standard functions.
            let areas = layout::split_into_horizontal_pains(main);
            let left_pain = areas.0;
            let right_pain = areas.1;
            table.render_selectable_table(f, left_pain);

            // TODO: Change it when split assignments are included in Rust's standard functions.
            let areas = layout::split_into_vertical_pains(right_pain);
            let detail_up = areas.0;
            let detail_down = areas.1;

            let block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled("Detail", Style::default().fg(Color::DarkGray)));
            let paragraph = Paragraph::new(detail_text.clone())
                .block(block)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, detail_up);

            let normal_style = Style::default().fg(Color::DarkGray);
            let history_table = histories
                .iter()
                .map(|i| Row::StyledData(i.iter(), normal_style));
            let header = vec!["IMAGE ID", "CREATED BY", "SIZE"];
            let image_history = Table::new(header.iter(), history_table)
                .block(Block::default().borders(Borders::ALL).title("History"))
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(75),
                    Constraint::Percentage(15),
                ])
                .column_spacing(1);
            f.render_widget(image_history, detail_down);
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
            Ok(v) => format_detail_text(v),
            Err(e) => span::from_texts(vec![format!("Failed to get container's details: {}", e)]),
        }
    } else {
        span::from_texts(vec!["It shows container's details here"])
    }
}

fn format_detail_text<'a>(detail: ImageDetail) -> Vec<Spans<'a>> {
    let mut texts = Vec::new();
    texts.push(format!("id: {}", detail.image.id));
    texts.push(format!("os/arch: {}/{}", detail.os, detail.architecture));
    texts.push(format!("entrypoint: {:?}", detail.entrypoint));
    texts.push(format!("cmd: {:?}", detail.cmd));

    texts.push("environment variables: ".to_string());
    detail
        .env
        .iter()
        .for_each(|v| texts.push(format!("- {}", v)));

    texts.push("labels: ".to_string());
    for (k, v) in detail.image.labels.iter() {
        texts.push(format!("- {}: {}", k, v));
    }

    span::from_texts(texts)
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
                            .map_or("none", |v| &v[..8])
                            .to_string(),
                        r.created_by,
                        r.size.to_string(),
                    ]
                })
                .collect(),
            Err(e) => vec![vec![e.to_string()]],
        }
    } else {
        vec![]
    }
}
