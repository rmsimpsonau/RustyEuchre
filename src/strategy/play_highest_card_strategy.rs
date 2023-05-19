use rand::Rng;
use crate::card::{Card, Rank, Suit};
use crate::hand::Hand;
use crate::helpers::filter_cards_by_lead_suit;
use crate::strategy::strategy::Strategy;
use crate::trick::Trick;

pub struct PlayHighestCardStrategy;

impl Strategy for PlayHighestCardStrategy {
    /// ```
    /// First, figure out if the `Player` will have to follow `Suit`. If so, play the highest `Card`
    /// that matches the lead `Suit`. Otherwise, pick the highest Card` from the `Player's` `Hand`.
    /// ```
    fn select_card_to_play(&self, hand: &mut Hand, trick: &mut Trick) -> Card {
        let mut rng = rand::thread_rng();
        let mut lead_suit: Option<Suit> = None;
        if trick.lead_card.is_some() {
            lead_suit = Some(trick.lead_card.unwrap().suit);
        }

        // Filter the cards in the hand that have to be played based on the lead suit
        let mut cards_matching_lead_suit: Vec<Card> = vec![];

        // If the lead suit is not None, then filter the cards by the lead suit
        if lead_suit.is_some() {
            cards_matching_lead_suit = filter_cards_by_lead_suit(hand, lead_suit.unwrap());
        }

        if cards_matching_lead_suit.len() > 0 {
            // Generate a random index within the range of playable cards and select card
            let card_index = rng.gen_range(0..cards_matching_lead_suit.len());
            let selected_card = cards_matching_lead_suit[card_index];

            // Find the index of the selected card in the original hand
            let original_index = hand.cards
                .iter()
                .position(|&card| card == selected_card)
                .unwrap();

            // Remove the card from the hand and return it with ownership
            hand.cards.swap_remove(original_index)
        } else {
            // There are no cards that match the lead suit so we can play any card. Pick randomly
            let card_index = rng.gen_range(0..hand.cards_left());
            hand.cards.swap_remove(card_index)
        }
    }

    /// ```
    /// Randomly choose.
    /// ```
    fn order_up_card(&self, _: &Card, _: &Hand, _: bool, _: bool) -> bool {
        let mut rng = rand::thread_rng();
        let random_choice: bool = rng.gen();
        return random_choice;
    }

    /// ```
    /// Randomly choose a `Card` that is NOT a trump `Card` if possible.
    /// ```
    fn swap_trump_card(&self, _: Card, _: &mut Hand) -> Card {
        let mut discard: Card = Card { rank: Rank::Ace, suit: Suit::Hearts };
        discard
    }

    /// ```
    /// Randomly choose to call trump or not. If 'Yes', then randomly choose a `Suit` that is not
    /// the same as the top `Card` that was passed on.
    /// ```
    fn choose_trump(&self, _: &Card, _: &Hand) -> Option<Suit> {
        let mut suit: Option<Suit> = None;
        return suit;
    }

    /// ```
    /// Randomly choose.
    /// ```
    fn go_alone(&self, _: &Hand, _: Suit) -> bool {
        let mut rng = rand::thread_rng();
        let random_choice: bool = rng.gen();
        return random_choice;
    }
}

mod tests {
}