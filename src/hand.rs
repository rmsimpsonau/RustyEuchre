use crate::card::{Card};

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Hand {
        let hand = Hand { cards: Vec::new() };
        hand
    }

    pub fn cards_left(&mut self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[cfg(test)]
mod tests {
    use crate::hand::Hand;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn new_hand_test() {
        let mut hand = Hand::new();
        let card: Card = Card { rank: Rank::JACK, suit: Suit::HEARTS };
        hand.add_card(card);
        assert_eq!(hand.cards_left(), 1);
        assert_eq!(hand.cards[0].rank, Rank::JACK);
        assert_eq!(hand.cards[0].suit, Suit::HEARTS);
    }
}