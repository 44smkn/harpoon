use tui::widgets::TableState;
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub struct StatefulTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
    pub title: String,
    pub header: Vec<String>,
    pub widths: Vec<Constraint>,
}

impl<'a> StatefulTable {
    pub fn new(
        items: Vec<Vec<String>>,
        title: impl Into<String>,
        header: Vec<impl Into<String>>,
        widths: Vec<Constraint>,
    ) -> StatefulTable {
        StatefulTable {
            state: TableState::default(),
            items,
            title: title.into(),
            header: header.into_iter().map(|s| s.into()).collect(),
            widths,
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

    pub fn render_selectable_table(&mut self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let selected_style = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let normal_style = Style::default().fg(Color::DarkGray);
        let rows = self
            .items
            .iter()
            .map(|i| Row::StyledData(i.iter(), normal_style));
        let t = Table::new(self.header.iter(), rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Spans::from(self.title.clone())),
            )
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(65),
                Constraint::Percentage(10),
                Constraint::Percentage(25),
            ]);
        frame.render_stateful_widget(t, rect, &mut self.state);
    }
}
