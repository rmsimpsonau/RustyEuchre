pub enum OrderUpCardStrategy {
    Random
}

pub mod order_up_card_strategies {
    use rand::Rng;
    use crate::card::Card;
    use crate::hand::Hand;
    use crate::strategy::order_up_card_strategies::OrderUpCardStrategy;
    use crate::trick::Trick;

    pub fn order_up_card<'a>(selected_strategy: OrderUpCardStrategy,
                             card: &'a Card, hand: &'a Hand,
                             dealer_is_self: bool, dealer_is_teammate: bool) -> bool {
        match selected_strategy {
            OrderUpCardStrategy::Random => order_up_card_random(
                card, hand, dealer_is_self, dealer_is_teammate
            )
        }
    }

    /// ```
    /// Randomly choose.
    /// ```
    fn order_up_card_random(_: &Card, _: &Hand, _: bool, _: bool) -> bool {
        let mut rng = rand::thread_rng();
        let random_choice: bool = rng.gen();
        return random_choice;
    }
}