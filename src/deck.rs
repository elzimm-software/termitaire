use crate::card::Card;

// A pile of an arbitrary number of cards
struct Pile {
    cards: Vec<Card>,
    index: usize,
}

impl Pile {
    fn new() -> Self {
        Pile {
            cards: Vec::new(),
            index: 0,
        }
    }

    // put card on top of pile
    fn place_top(&mut self, card: Card) {
        self.index += 1;
        self.cards.push(card);
    }

    // put card on bottom of pile
    fn place_bottom(&mut self, card: Card) {
        self.index += 1;
        self.cards.insert(0, card);
    }

    // draw card from top of pile
    fn draw(&mut self) -> Card {
        self.index -= 1;
        self.cards.pop().unwrap()
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
