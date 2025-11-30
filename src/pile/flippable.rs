use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

pub struct Flippable;

impl Renderer for Flippable {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}