#![allow(dead_code, unused_variables)] // Remove before release

//! Termitaire is Klondike Solitaire: ratpoisoned and terminal interfaced.
//!
//! The intention here is to make playing Solitaire on a laptop intuitive and enjoyable.
//! Playing on a trackpad is tedious and using a touchscreen becomes tiring, so why not use just the keyboard?
//! Of course TUI Solitaire exists, as does ratpoisoned Solitaire.
//! But they require painstakingly selecting cards with the arrow keys.
//! That simply wont fly here, every interface can, and should, use Vimmic motions.
//!
//! Each pile is assigned a key, pressing the key selects the top card on the pile.
//! Repeated presses cycles through the selectable cards on the pile.
//! Pressing the key of a different pile will move the selected cards onto that pile if possible.
//! Pressing space at anytime will deselect the current pile.

mod card;
mod pile;
mod table;
mod state;

fn main() {
    let table = table::Table::new();
    println!("{:#?}", table);
}
