use crate::pile::Pile;

pub struct Table {
    stacks: [Pile; 7], // the general play area
    home: [Pile; 4],
    draw: Pile
}

impl Table {
    pub fn new() -> Self {
        todo!()
    }
}
