use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub fn split_into_header_and_main(frame: &mut Frame<impl Backend>) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(90)].as_ref())
        .split(frame.size());
    let header = areas[0];
    let main = areas[1];
    (header, main)
}

pub fn split_into_horizontal_pains(rect: Rect) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(rect);
    let left = areas[0];
    let right = areas[1];
    (left, right)
}

pub fn split_into_vertical_pains(rect: Rect) -> (Rect, Rect) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(rect);
    let up = areas[0];
    let down = areas[1];
    (up, down)
}
