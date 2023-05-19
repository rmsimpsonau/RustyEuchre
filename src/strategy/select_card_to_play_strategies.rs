pub enum SelectCardToPlayStrategy {
    Random
}

pub mod select_card_to_play_strategies {
    use rand::Rng;
    use crate::card::{Card, Suit};
    use crate::hand::Hand;
    use crate::strategy::select_card_to_play_strategies::SelectCardToPlayStrategy;
    use crate::trick::Trick;
    use crate::helpers::{filter_cards_by_lead_suit, card_matches_lead_suit};

    pub fn select_card_to_play<'a>(selected_strategy: SelectCardToPlayStrategy,
                                      hand: &'a mut Hand, trick: &'a Trick) -> Card {
        match selected_strategy {
            SelectCardToPlayStrategy::Random => select_card_to_play_random(hand, trick)
        }
    }

    /// ```
    /// First, figure out if the `Player` will have to follow `Suit`. If so, play a random `Card`
    /// that matches the lead `Suit`. Otherwise, pick any random `Card` from the `Player's` `Hand`.
    /// ```
    fn select_card_to_play_random<'a>(hand: &'a mut Hand, trick: &'a Trick) -> Card {
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
}