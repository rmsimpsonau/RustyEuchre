use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum Rank {
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE
}

impl Rank {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Rank::ACE, Rank::KING, Rank::QUEEN, Rank::JACK, Rank::TEN, Rank::NINE].into_iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::ACE => write!(f, "A"),
            Rank::KING => write!(f, "K"),
            Rank::QUEEN => write!(f, "Q"),
            Rank::JACK => write!(f, "J"),
            Rank::TEN => write!(f, "10"),
            Rank::NINE => write!(f, "9"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    SPADES,
    CLUBS
}

impl Suit {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Suit::HEARTS, Suit::DIAMONDS, Suit::SPADES, Suit::CLUBS].into_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::HEARTS => write!(f, "♥"),
            Suit::DIAMONDS => write!(f, "♦"),
            Suit::SPADES => write!(f, "♠"),
            Suit::CLUBS => write!(f, "♣"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}