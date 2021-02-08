use crate::shared::{
    event::{Event, Events},
    layout,
    paragraph::SimpleParagraph,
    table::{StatefulTable, StatelessTable},
    tabs::TabsState,
};
use crate::TuiResult;
use domain::image::ImageSummary;

use termion::event::Key;
use tui::{backend::Backend, layout::Constraint, Terminal};
use usecase::{
    inspect_image::{HistoryRecord, InspectImageDto, InspectImageUsecase},
    list_image::ListImageUsecase,
};

pub struct ImageTuiController<'a> {
    list_usecase: &'a ListImageUsecase<'a>,
    inspect_usecase: &'a InspectImageUsecase<'a>,
}

impl<'a> ImageTuiController<'a> {
    pub fn new(list_usecase: &'a ListImageUsecase<'a>, inspect_usecase: &'a InspectImageUsecase) -> Self {
        Self {
            list_usecase,
            inspect_usecase,
        }
    }

    pub async fn draw(&self, terminal: &mut Terminal<impl Backend>, tab: &mut TabsState<'a>) -> TuiResult {
        let mut images = self.list_usecase.list_image().await?;
        // image list table
        let items = images_to_table(&mut images);
        let header = vec!["NAME", "SIZE", "CREATED"];
        let widths = vec![
            Constraint::Percentage(65),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
        ];
        let mut image_table = StatefulTable::new(items, "Images", header, widths);
        // image detail paragraph
        let mut paragraph = SimpleParagraph::new("Detail", vec!["It shows container's details here"]);
        // image history table
        let header = vec!["IMAGE ID", "CREATED BY", "SIZE"];
        let widths = vec![
            Constraint::Percentage(10),
            Constraint::Percentage(75),
            Constraint::Percentage(15),
        ];
        let mut history_table = StatelessTable::new(vec![], "History", header, widths);
        let events = Events::new();

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
                image_table.render(f, left_pain);
                // TODO: Change it when split assignments are included in Rust's standard functions.
                let areas = layout::split_into_vertical_pains(right_pain);
                let detail_up = areas.0;
                let detail_down = areas.1;
                paragraph.render(f, detail_up);
                history_table.render(f, detail_down);
            })?;

            if let Event::Input(key) = events.next()? {
                match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Down => {
                        image_table.next();
                        // TODO: Change it when split assignments are included in Rust's standard functions.
                        let selected = image_table.state.selected();
                        let detail = gen_detail_text(selected, &images, self.inspect_usecase).await;
                        paragraph.texts = detail.0;
                        history_table.items = detail.1;
                    }
                    Key::Up => {
                        image_table.previous();
                        let selected = image_table.state.selected();
                        let detail = gen_detail_text(selected, &images, self.inspect_usecase).await;
                        paragraph.texts = detail.0;
                        history_table.items = detail.1;
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
}

fn images_to_table(images: &mut Vec<ImageSummary>) -> Vec<Vec<String>> {
    let mut items: Vec<Vec<String>> = Vec::new();
    for image in images.iter_mut() {
        if &image.repo_tags[0] == "<none>:<none>" {
            continue;
        }
        let mut row: Vec<String> = Vec::new();
        row.push(std::mem::take(&mut image.repo_tags[0]));
        let size = f64::from(image.size) / 1000000.0;
        row.push(format!("{:.2}MB", size));
        row.push(image.created.format("%Y-%m-%d").to_string());
        items.push(row);
    }
    items
}

async fn gen_detail_text<'a>(
    idx: Option<usize>,
    images: &[ImageSummary],
    inspect_image_usecase: &'a InspectImageUsecase<'a>,
) -> (Vec<String>, Vec<Vec<String>>) {
    if let Some(v) = idx {
        let image_id = &images[v].id;
        let detail = inspect_image_usecase.inspect_image(image_id).await;
        match detail {
            Ok(v) => (format_detail_text(&v), format_history_text(v.history)),
            Err(e) => (vec![format!("Failed to get container's details: {}", e)], vec![]),
        }
    } else {
        (vec!["It shows container's details here".to_string()], vec![])
    }
}

fn format_detail_text(detail: &InspectImageDto) -> Vec<String> {
    let mut texts = Vec::new();
    texts.push(format!("id: {}", detail.id));
    texts.push(format!("os/arch: {}/{}", detail.os, detail.architecture));
    texts.push(format!("entrypoint: {:?}", detail.entrypoint));
    texts.push(format!("cmd: {:?}", detail.cmd));

    texts.push("environment variables: ".to_string());
    detail
        .environment_variables
        .iter()
        .for_each(|v| texts.push(format!("- {}", v)));

    texts.push("labels: ".to_string());
    for (k, v) in detail.labels.iter() {
        texts.push(format!("- {}: {}", k, v));
    }

    texts
}

fn format_history_text(records: Vec<HistoryRecord>) -> Vec<Vec<String>> {
    records
        .into_iter()
        .map(|r| {
            vec![
                r.image_id
                    .split(':')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .map_or("none", |v| &v[..8])
                    .to_string(),
                r.created_by,
                r.size.to_string(),
            ]
        })
        .collect()
}
