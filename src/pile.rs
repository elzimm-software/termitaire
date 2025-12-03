mod straight;
mod cascade;
mod flippable;

use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use crate::card::Card;

/// A pile of an arbitrary number of cards
///
/// Cards can be placed onto the top or bottom.
/// A pile can also be used as an iterator to non-destructively cycle through the cards from top to bottom.
///
/// The renderer determines how the cards are drawn:
///  - [Flippable](render_type::Flippable) renders two stacks side by side and shows cards being flipped from one onto the other.
///  - [Cascade](render_type::Cascade) renders each card partially overlapping with `n_hidden` appearing only as a card back.
///  - [Straight](render_type::Straight) renders a single stack.
//
// TODO: Ensure proper behavior when inserting while being used as a iterator.
pub struct Pile<'a> {
    cards: Vec<Card>,
    index: usize,
    renderer: Option<Box<dyn Renderer + 'a>>
}

impl<'a> Pile<'a> {
    /// Create an empty Pile.
    pub fn new() -> Self {
        Pile {
            cards: Vec::new(),
            index: 0,
            renderer: None,
        }
    }

    pub fn from<T: TryInto<Card, Error: Debug> + Copy>(cards: Vec<T>) -> Self {
    /// Create a new pile from the given `Into<Vec>`.
        Self {
            cards: cards.iter().map(|&x| x.try_into().unwrap()).collect(),
            index: cards.len(),
            renderer: None,
        }
    }

    /// Set the rendering strategy.
    ///  - [Flippable](render_type::Flippable) renders two stacks side by side and shows cards being flipped from one onto the other.
    ///  - [Cascade](render_type::Cascade) renders each card partially overlapping with `n_hidden` appearing only as a card back.
    ///  - [Straight](render_type::Straight) renders a single stack.
    #[allow(private_bounds)]
    pub fn render_as<R: Renderer + 'a>(self, renderer: R) -> Self {
        Self {
            renderer: Some(Box::new(renderer)),
            ..self
        }
    }

    /// Generates a new 52 card deck.
    pub fn deck52() -> Self {
        let mut vec: Vec<Card> = Vec::new();
        // iterate over suits
        for s in 1..=4 {
            for v in 1..=13 {
            // iterate over values
                vec.push((v, s).try_into().unwrap())
            }
        }

        Self::from(vec)
    }

    // put card on top of pile
    pub fn place_top(&mut self, card: Card) {
    /// Put card on top of pile.
    ///
    /// This places the card on the current top of the pile.
    /// If the pile is being iterated through the top will change.
        self.index += 1;
        self.cards.push(card);
    }

    // put card on bottom of pile
    pub fn place_bottom(&mut self, card: Card) {
    /// Put card on bottom of pile.
        self.index += 1;
        self.cards.insert(0, card);
    }

    /// draw card from top of pile.
    pub fn draw(&mut self) -> Card {
        self.index -= 1;
        self.cards.remove(self.index - 1)
    }

    // peek at top card in pile
    pub fn top(&self) -> &Card {
        self.cards.get(self.index - 1).unwrap()
    }
}

impl Widget for Pile<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        if let Some(renderer) = &self.renderer {
            renderer.render(&self, area, buf);
        }
    }
}

impl Clone for Pile<'_> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Iterator for Pile<'_> {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= 0 {
            return None;
        }
        self.index -= 1;
        self.cards.get(self.index).map(|c| *c)
    }
}

impl Debug for Pile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pile").field("cards", &self.cards).field("index", &self.index).field("renderer", &self.renderer).finish()
    }
}

trait Renderer: Debug {
    fn render(&self, pile: &Pile, area: Rect, buf: &mut Buffer);
}

pub mod render_type {
    pub use super::cascade::Cascade;
    pub use super::flippable::Flippable;
    pub use super::straight::Straight;
}
