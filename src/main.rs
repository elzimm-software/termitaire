#![allow(dead_code, unused_variables)] // Remove before release

mod card;
mod pile;
mod table;
mod state;

fn main() {
    let table = table::Table::new();
    println!("{:#?}", table);
}
