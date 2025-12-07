use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

/// Draws a [Pile] as a single stack.
///
/// ```text
/// ┌─────────┐
/// │A♠       │
/// │         │
/// |         |
/// │         │
/// │       A♠│
/// └─────────┘
/// ```
pub struct Straight;

impl Debug for Straight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Straight").finish()
    }
}

impl Renderer for Straight {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
