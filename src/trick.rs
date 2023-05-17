use crate::card::{Card, Suit};
use crate::helpers::get_card_value;

pub struct Trick {
    pub trump_suit: Suit,
    pub lead_card: Option<Card>,
    pub highest_card: Option<Card>,
    pub highest_card_player_index: Option<usize>,
    pub cards_played: Vec<Card>
}

impl Trick {
    pub fn new(trump_suit: Suit) -> Self {
        Trick { trump_suit, lead_card: None, highest_card: None, highest_card_player_index: None, cards_played: Vec::new() }
    }

    pub fn card_played(&mut self, card: Card, player_index: usize) {
        if self.lead_card.is_none() {
            self.lead_card = Some(card);
            self.highest_card = Some(card);
            self.highest_card_player_index = Some(player_index);
        } else {
            let lead_card_suit: Suit = self.lead_card.expect("Lead card should be set").suit;
            // If card played is a higher value than the current highest card,
            let card_value = get_card_value(card, self.trump_suit, lead_card_suit);
            let highest_card = get_card_value(
                self.highest_card.expect("Highest card should be set"), self.trump_suit, lead_card_suit
            );

            // If the new card value is bigger than the highest_card value, then set the new card as the highest card
            if card_value > highest_card {
                self.highest_card = Some(card);
                self.highest_card_player_index = Some(player_index);
            }
        }

        self.cards_played.push(card);
    }

    pub fn print_results(&self) {
        let highest_card: Card = self.highest_card.expect("Highest card should not be None");
        println!("Player {} wins the Trick", self.highest_card_player_index.expect("Player index should not be None") + 1);
        print!("[ ");
        for card in self.cards_played.iter() {
            // If this card is the highest card, then print parentheses around it to designate
            if card.suit == highest_card.suit && card.rank == highest_card.rank {
                print!("({}{}) ", card.rank, card.suit);
            } else {
                print!("{}{} ", card.rank, card.suit);
            }
        }
        println!("]");
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::card::{Card, Rank, Suit};
    #[allow(unused_imports)]
    use crate::trick::Trick;

    #[test]
    fn new_trick_test() {
        let trick: Trick = Trick::new(Suit::Clubs);
        assert_eq!(trick.trump_suit, Suit::Clubs);
    }

    #[test]
    fn card_played_lead_test() {
        let mut trick: Trick = Trick::new(Suit::Clubs);
        let lead_card: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        trick.card_played(lead_card, 2);
        assert_eq!(trick.lead_card.expect("Card should have been lead"), lead_card);
        assert_eq!(trick.highest_card.expect("There should be a high card"), lead_card);
        assert_eq!(trick.highest_card_player_index.expect("There should be a high card player index"), 2);
    }

    #[test]
    fn card_played_two_cards_test() {
        let mut trick: Trick = Trick::new(Suit::Clubs);
        let card1: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        let card2: Card = Card { rank: Rank::Nine, suit: Suit::Clubs };

        trick.card_played(card1, 1);
        trick.card_played(card2, 2);
        assert_eq!(trick.lead_card.expect("Card should have been lead"), card1);
        assert!(trick.cards_played.contains(&card1));
    }
}