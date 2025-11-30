use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

pub struct Straight;

impl Renderer for Straight {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}