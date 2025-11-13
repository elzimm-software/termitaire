
#[derive(Copy, Clone)]
pub struct Card {
    suit: Suit,
    value: Value,
}

#[derive(Copy, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
}

#[derive(Copy, Clone)]
pub enum Value {
    Ace,
    Number(u8),
    Face(FaceValue)
}

#[derive(Copy, Clone)]
pub enum FaceValue {
    King,
    Queen,
    Jack,
}

impl Card {
    fn new(value: Value, suit: Suit) -> Self {
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
                11 => Value::Face(FaceValue::Jack),
                12 => Value::Face(FaceValue::Queen),
                13 => Value::Face(FaceValue::King),
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

impl Suit {
    fn get_color(&self) -> Color {
        match self {
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }
}
