mod straight;
mod cascade;
mod flippable;

use std::fmt::Debug;
use std::ops::Index;
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

    /// Create a new pile from the given `Into<Vec>`.
    pub fn from<V: Into<Vec<T>>, T: TryInto<Card, Error: Debug> + Copy>(cards: V) -> Self {
        let vec = cards.into();
        Self {
            cards: vec.iter().map(|&x| x.try_into().unwrap()).collect(),
            index: vec.len(),
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
            // iterate over values
            for v in (1..=13).rev() {
                vec.push((v, s).try_into().unwrap())
            }
        }

        Self::from(vec)
    }

    /// Put card on top of pile.
    ///
    /// This places the card on the current top of the pile.
    /// If the pile is being iterated through the top will change.
    pub fn place_top<T: TryInto<Card, Error: Debug> + Copy>(&mut self, card: T) {
        self.cards.insert(self.index, card.try_into().unwrap());
        self.index += 1;
    }

    /// Put card on bottom of pile.
    pub fn place_bottom<T: TryInto<Card, Error: Debug> + Copy>(&mut self, card: T) {
        self.cards.insert(0, card.try_into().unwrap());
        self.index += 1;
    }

    /// draw card from top of pile.
    pub fn draw(&mut self) -> Card {
        self.index -= 1;
        self.cards.remove(self.index)
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

/// Computes returned position as if the current `self.index` is the top of the pile
/// and all [Cards](Card) above it are at the bottom of the pile.
impl Index<usize> for Pile<'_> {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.cards.len() {
            panic!("Index {} out of bounds.", index);
        }
        &self.cards[(index + self.index) % self.cards.len()]
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

    const SPADES: char = '\u{2660}';
    const HEARTS: char = '\u{2665}';
    const DIAMONDS: char = '\u{2666}';
    const CLUBS: char = '\u{2663}';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_top() {
        let first_check = Pile::from(vec![(1,1),(2,1),(4,1),(3,1)]);
        let second_check = Pile::from(vec![(1,1),(2,1),(3,1),(4,1)]);
        let mut pile = Pile::from(vec![(1,1),(2,1),(4,1)]);
        pile.place_top((3,1));
        println!("{:?}", pile);
        for i in 0..4 {
            assert_eq!(pile.cards[i], first_check.cards[i])
        }
        let _ = pile.draw();
        let _ = pile.next();
        pile.place_top((3,1));
        println!("{:?}", pile);
        for i in 0..4 {
            assert_eq!(pile.cards[i], second_check.cards[i])
        }
    }

    #[test]
    fn index_impl() {
        let mut pile  = Pile::from(vec![(1,1), (2,1), (3,1), (4,1)]);
        dbg!(pile.index);
        assert_eq!(pile[3], (4,1).try_into().unwrap());
        let _ = pile.next();
        dbg!(pile.index);
        assert_eq!(pile[3], (3,1).try_into().unwrap());
        dbg!(pile.index);
        assert_eq!(pile[0], (4,1).try_into().unwrap());
    }
}
