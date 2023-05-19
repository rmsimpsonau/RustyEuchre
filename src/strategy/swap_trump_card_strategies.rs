pub enum SwapTrumpCardStrategy {
    Random
}

pub mod swap_trump_card_strategies {
    use rand::Rng;
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;
    use crate::helpers::get_left_bower_suit;
    use crate::strategy::swap_trump_card_strategies::SwapTrumpCardStrategy;

    pub fn swap_trump_card(selected_strategy: SwapTrumpCardStrategy,
                           incoming_card: Card, hand: &mut Hand) -> Card {
        match selected_strategy {
            SwapTrumpCardStrategy::Random => swap_trump_card_random(
                incoming_card, hand
            )
        }
    }

    /// ```
    /// Randomly choose a `Card` that is NOT a trump `Card` if possible.
    /// ```
    fn swap_trump_card_random(incoming_card: Card, hand: &mut Hand) -> Card {
        let mut discard: Card = Card { rank: Rank::Ace, suit: Suit::Hearts };

        // Find all NON-trump cards first if any exist. Do not include the left bower if found
        let non_matching_cards: Vec<Card> = hand.cards.iter()
            .filter(|&card| {
                card.suit != incoming_card.suit &&
                    !(card.rank == Rank::Jack && card.suit == get_left_bower_suit(incoming_card.suit))
            })
            .cloned()
            .collect();


        let chosen_card = if !non_matching_cards.is_empty() {
            // We have some cards that are NOT trump to choose from. Choose lowest Rank.
            non_matching_cards
                .iter()
                .min_by_key(|&card| card.rank)
                .unwrap()
                .clone()
        } else {
            // All of the cards are trump so choose the lowest Rank (JACK is highest)
            hand.cards
                .iter()
                .min_by_key(|&card| {
                    match card.rank {
                        Rank::Jack => 1, // JACK has the highest value
                        _ => card.rank as i32, // Use the default ranking order for other ranks
                    }
                })
                .unwrap()
                .clone()
        };

        if let Some(index) = hand.cards.iter().position(|&card| card == chosen_card) {
            discard = hand.cards.remove(index);
        }

        hand.cards.push(incoming_card);

        discard
    }
}