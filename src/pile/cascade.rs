use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

pub struct Cascade {
    n_hidden: usize,
}

impl Cascade {
    pub fn new(n_hidden: usize) -> Self {
        Self {
            n_hidden,
        }
    }
}

impl Debug for Cascade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cascade").field("n_hidden", &self.n_hidden).finish()
    }
}

impl Renderer for Cascade {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
