use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

/// Draws a [Pile] as two stacks side by side and shows cards being flipped from one onto the other.
///
/// ```text
/// ┌─────────┐ ┌──┌──┌─────────┐
/// │░░░░░░░░░│ |A♠|2♠|3♠       │
/// │▒▒▒▒▒▒▒▒▒│ |  |  |         │
/// │░░░░░░░░░│ |  |  |         │
/// │▒▒▒▒▒▒▒▒▒│ |  |  |         │
/// │░░░░░░░░░│ |  |  |       3♠│
/// └─────────┘ └──└──└─────────┘
/// ```
pub struct Flippable;

impl Debug for Flippable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Flippable").finish()
    }
}

impl Renderer for Flippable {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}
