
/// Representation of a playing card.
///
/// The primary duty of this struct is to handle comparisons between [cards](Card).
/// I haven't yet decided if I want [cards](Card) to know how to render themselves
/// or if I can leave that to the [Pile](crate::pile::Pile) rendering strategies.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    /// Construct a new [Card] with [Value] and [Suit].
    ///
    /// See also [try_from()](Card::try_from()) for constructing from integers.
    pub fn new(value: Value, suit: Suit) -> Self {
        Self {
            value,
            suit,
        }
    }
}

/// Construct a [Card] from a [tuple](tuple) of [u8]s
/// where the first value is the [value](Value) and the second is the [suit](Suit).
///
/// [Card] uses the errable [TryFrom] as [u8] values outside [1, 13] are invalid.
//
// I've considered implementing TryFrom<u8> for both Value and Suit and using that here,
// but trying to pass Err up with `?` gets wierd and I see no reason not to do it this way.
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

/// Enum of possible card suits.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Suit {
    Spades = 4,
    Hearts = 3,
    Diamonds = 2,
    Clubs = 1,
}

impl Suit {
    /// Returns the color of the suit.
    fn get_color(&self) -> Color {
        match self {
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::Diamonds | Suit::Hearts => Color::Red,
        }
    }
}

/// Enum of possible colors for a suit.
///
/// This does not implement [Eq] or [Ord] because all comparisons should be done via [Suit].
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    Red,
}

/// Enum of possible card values.
///
/// [Number](Value::Number) is used for all cards [2, 10].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Value {
    Ace,
    Number(u8),
    Jack,
    King,
    Queen,
}

impl Value {
    /// Returns if the card is a face card.
    /// 
    /// [Ace](Value::Ace) is not considered a face card.
    //
    // Maybe Ace as a face card could be a feature.
    fn is_face(&self) -> bool {
        match self {
            Value::Jack | Value::Queen | Value::King => true,
            _ => false,
        }
    }
}
