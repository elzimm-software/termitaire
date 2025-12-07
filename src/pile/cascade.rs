use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use crate::pile::{Pile, Renderer};

/// Draws a [Pile] as partially overlapping each other vertically,
/// displaying the true number of cards in the pile.
/// The first `n_hidden` cards from the bottom are displayed as card backs,
/// while the remainder are shown with value and suit.
///
/// ```text
/// ┌─────────┐
/// │░░░░░░░░░│
/// ┌─────────┐
/// │A♠       │
/// ┌─────────┐
/// │K♥       │
/// |         |
/// |         |
/// |         |
/// │       K♥│
/// └─────────┘
/// ```
pub struct Cascade {
    n_hidden: usize,
}

impl Cascade {
    /// Construct a new [Cascade] rendering strategy with a given `n_hidden`.
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
