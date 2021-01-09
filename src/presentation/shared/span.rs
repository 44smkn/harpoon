use tui::text::Spans;

pub fn from_texts<'a>(texts: Vec<impl Into<String>>) -> Vec<Spans<'a>> {
    texts.into_iter().map(|t| Spans::from(t.into())).collect()
}
