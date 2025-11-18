use crate::card::Card;

// A pile of an arbitrary number of cards
pub struct Pile {
    cards: Vec<Card>,
    index: usize,
}

impl Pile {
    pub fn new() -> Self {
        Pile {
            cards: Vec::new(),
            index: 0,
        }
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

impl Iterator for Pile {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= 0 {
            return None;
        }
        self.index -= 1;
        self.cards.get(self.index).map(|c| *c)
    }
}
