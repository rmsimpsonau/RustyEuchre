use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::card::{Card, Rank, Suit};

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        deck.populate();
        deck.shuffle();
        deck
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn cards_left(&mut self) -> usize {
        self.cards.len()
    }

    fn populate(&mut self) {
        self.cards.clear();
        for rank in Rank::iter() {
            let rank_clone = rank.clone();
            for suit in Suit::iter() {
                let suit_clone = suit.clone();
                self.cards.push(Card {
                    rank: rank_clone,
                    suit: suit_clone
                });
            }
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn top_card(&mut self) -> &Card {
        &self.cards[self.cards.len() - 1]
    }

    pub fn take_top_card(&mut self) -> Card {
        self.cards[self.cards.len() - 1]
    }
 }


#[cfg(test)]
mod tests {
    use crate::deck::{Card, Deck};

    #[test]
    fn new_deck_populate_and_shuffle_test() {
        let mut deck = Deck::new();
        assert_eq!(deck.cards_left(), 24);
    }

    #[test]
    fn draw_test() {
        let mut deck = Deck::new();
        deck.draw();
        assert_eq!(deck.cards_left(), 23);
    }

    #[test]
    fn top_card_test() {
        let mut deck = Deck::new();
        let card: Card = deck.cards[deck.cards.len() - 2];
        deck.draw();
        let top_card: &Card = deck.top_card();
        assert_eq!(top_card.suit, card.suit);
        assert_eq!(top_card.rank, card.rank);
    }

    #[test]
    fn take_top_card_test() {
        let mut deck = Deck::new();
        let card: Card = deck.cards[deck.cards.len() - 2];
        deck.draw();
        let top_card: Card = deck.take_top_card();
        assert_eq!(top_card.suit, card.suit);
        assert_eq!(top_card.rank, card.rank);
        assert_eq!(deck.cards.len(), 23);
    }

    #[test]
    fn add_card_test() {
        let mut deck = Deck::new();
        let card: Card = deck.draw();
        deck.add_card(card);
        assert_eq!(deck.cards.len(), 24);
    }
}