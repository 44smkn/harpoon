use crate::presentation::shared::span;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct SimpleParagraph {
    pub title: String,
    pub texts: Vec<String>,
}

impl SimpleParagraph {
    pub fn new(title: impl Into<String>, texts: Vec<impl Into<String>>) -> SimpleParagraph {
        SimpleParagraph {
            title: title.into(),
            texts: texts.into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            &self.title,
            Style::default().fg(Color::DarkGray),
        ));
        let paragraph = Paragraph::new(span::from_texts(self.texts.clone()))
            .block(block)
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, rect);
    }
}
