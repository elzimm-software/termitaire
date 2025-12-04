
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self {
            value,
            suit,
        }
    }
}

impl TryFrom<(u8, u8)> for Card {
    type Error = &'static str;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Ok(Self {
            value: match value.0 {
                1 => Value::Ace,
                2_u8..=10_u8 => Value::Number(value.0),
                11 => Value::Jack,
                12 => Value::Queen,
                13 => Value::King,
                _ => return Err("Invalid card value"),
            },
            suit: match value.1 {
                4 => Suit::Spades,
                3 => Suit::Hearts,
                2 => Suit::Diamonds,
                1 => Suit::Clubs,
                _ => return Err("Invalid card suit"),
            }
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn get_color(&self) -> Color {
        match self {
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Value {
    Ace,
    Number(u8),
    Jack,
    King,
    Queen,
}

impl Value {
    fn is_face(&self) -> bool {
        match self {
            Value::Jack | Value::Queen | Value::King => true,
            _ => false,
        }
    }
}
