use std::{array::from_fn, fmt::Debug};
use crate::pile::Pile;
use crate::pile::render_type::{Cascade, Flippable, Straight};

pub struct Table<'a> {
    stacks: [Pile<'a>; 7], // the general play area
    home: [Pile<'a>; 4],
    draw: Pile<'a>
}

impl Table<'_> {
    pub fn new() -> Self {
        let mut deck = Pile::deck52();
        let mut stacks: [Pile; 7] = from_fn(|i| Pile::new().render_as(Cascade::new(i)));

        for i in 0..7 {
            for j in i..7 {
                stacks[j].place_top(deck.draw());
            }
        }

        Self {
            stacks,
            home: from_fn(|_| Pile::new().render_as(Straight)),
            draw: deck.render_as(Flippable),
        }
    }
}

impl Debug for Table<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Table").field("stacks", &self.stacks).field("home", &self.home).field("draw", &self.draw).finish()
    }
}
