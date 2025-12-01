mod straight;
mod cascade;
mod flippable;

use std::fmt::Debug;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use crate::card::Card;

// A pile of an arbitrary number of cards
pub struct Pile<'a> {
    cards: Vec<Card>,
    index: usize,
    renderer: Option<Box<dyn Renderer + 'a>>
}

impl<'a> Pile<'a> {
    pub fn new() -> Self {
        Pile {
            cards: Vec::new(),
            index: 0,
            renderer: None,
        }
    }

    pub fn from<T: TryInto<Card, Error: Debug> + Copy>(cards: Vec<T>) -> Self {
        Self {
            cards: cards.iter().map(|&x| x.try_into().unwrap()).collect(),
            index: cards.len(),
            renderer: None,
        }
    }

    #[allow(private_bounds)]
    pub fn render_as<R: Renderer + 'a>(self, renderer: R) -> Self {
        Self {
            renderer: Some(Box::new(renderer)),
            ..self
        }
    }

    pub fn deck52() -> Self {
        let mut vec: Vec<Card> = Vec::new();
        for s in 1..=4 {
            for v in 1..=13 {
                vec.push((v, s).try_into().unwrap())
            }
        }

        Self::from(vec)
    }

    // put card on top of pile
    pub fn place_top(&mut self, card: Card) {
        self.index += 1;
        self.cards.push(card);
    }

    // put card on bottom of pile
    pub fn place_bottom(&mut self, card: Card) {
        self.index += 1;
        self.cards.insert(0, card);
    }

    // draw card from top of pile
    pub fn draw(&mut self) -> Card {
        self.index -= 1;
        self.cards.remove(self.index + 1)
    }

    // peek at top card in pile
    pub fn top(&self) -> &Card {
        self.cards.get(self.index).unwrap()
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
