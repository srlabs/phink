use ratatui::{
    layout::Rect,
    Frame,
};

pub trait Paint {
    fn render(&self, f: &mut Frame, area: Rect);
}
