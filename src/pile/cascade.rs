use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

pub struct Cascade;

impl Renderer for Cascade {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}