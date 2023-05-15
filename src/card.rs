use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum Rank {
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, Rank::Nine].into_iter()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::King => write!(f, "K"),
            Rank::Queen => write!(f, "Q"),
            Rank::Jack => write!(f, "J"),
            Rank::Ten => write!(f, "10"),
            Rank::Nine => write!(f, "9"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

impl Suit {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs].into_iter()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Spades => write!(f, "♠"),
            Suit::Clubs => write!(f, "♣"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}