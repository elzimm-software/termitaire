use std::{array::from_fn, fmt::Debug};
use crate::pile::Pile;
use crate::pile::render_type::{Cascade, Flippable, Straight};

/// Composition of all piles used in a game.
///
/// These include:
/// The stack: the set of 7 ascending piles used to move cards around during the game.
/// The home: the set of 4 piles in which cards are placed to complete the game.
/// The draw: the pile of all cards not currently in play that can be flipped through an infinite number of times.
pub struct Table<'a> {
    stacks: [Pile<'a>; 7], // the general play area
    home: [Pile<'a>; 4],
    draw: Pile<'a>
}

impl Table<'_> {
    /// Generate a new table from a shuffled 52 card deck.
    ///
    /// To deal a game cards are placed by row into 7 piles,
    /// For each row `n` skip `n` columns before placing the first card.
    /// Once the 7th row is completed with a single card, the remaining cards form the draw pile.
    pub fn new() -> Self {
        // generate new deck
        let mut deck = Pile::deck52();
        // initialize stacks
        let mut stacks: [Pile; 7] = from_fn(|i| Pile::new().render_as(Cascade::new(i)));
        // place cards onto stacks
        for i in 0..7 {
            // skip the first i stacks
            for j in i..7 {
                // pull off the deck and place onto the pile
                stacks[j].place_top(deck.draw());
            }
        }

        Self {
            stacks,
            // initialize homes
            home: from_fn(|_| Pile::new().render_as(Straight)),
            // set deck renderer to convert to draw pile
            draw: deck.render_as(Flippable),
        }
    }
}

impl Debug for Table<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table").field("stacks", &self.stacks).field("home", &self.home).field("draw", &self.draw).finish()
    }
}
