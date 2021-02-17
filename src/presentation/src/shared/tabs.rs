use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn new_menu() -> TabsState<'a> {
        let titles = vec!["Container", "Image", "Volume", "Network"];
        TabsState::new(titles)
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }

    pub fn draw(&mut self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
        frame.render_widget(block, frame.size());

        let titles = self
            .titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::Green)),
                ])
            })
            .collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Tabs"))
            .select(self.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));
        frame.render_widget(tabs, rect);
    }
}
