use rand::Rng;
use crate::card::{Card, Suit};
use crate::deck::Deck;
use crate::player::Player;

pub struct Game {
    deck: Deck,
    players: [Player; 4],
    dealer_number: i8,
    current_trump_suit: Option<Suit>,
    maker_going_alone: bool
}

impl Game {
    pub fn new(players: [Player; 4]) -> Self {
        // Choose random player to go first
        let mut rng = rand::thread_rng();
        let dealer_number = rng.gen_range(1..5);

        println!("Dealer is Player {}", dealer_number);

        let game = Game { deck: Deck::new(), players, dealer_number, current_trump_suit: None, maker_going_alone: false };
        game
    }

    pub fn cards_left(&mut self) -> usize {
        self.deck.cards_left()
    }

    pub fn deal_cards(&mut self) {
        let mut i = 1;
        for player in self.players.iter_mut() {
            for _ in 0..5 {
                let card = self.deck.draw();
                player.add_card_to_hand(card);
            }
            println!("Player {} Cards: ", i);
            for card in &player.hand.cards {
                print!(" {}{} ", card.rank, card.suit);
            }
            println!();
            i += 1;
        }
    }

    pub fn decide_trump(&mut self) {
        // Get card from top fo the deck
        let top_card = self.deck.take_top_card();
        let player_left_of_dealer = (self.dealer_number) as usize;
        let dealer_index = (self.dealer_number - 1) as usize;
        let mut maker_player_index: Option<usize> = None;

        // Starting with the player left of the dealer, ask if players want to order up top card
        println!("Card in front of dealer is {}{}", top_card.rank, top_card.suit);
        for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
            let player_index = i % self.players.len();
            print!("Asking Player {} is they want to order up trump...", player_index + 1);
            let player = &mut self.players[player_index];

            let dealer_is_self = dealer_index == player_index;
            // Check if the current player is 2 seats away from the dealer, meaning you are the dealer's teammate
            let dealer_is_teammate = (i + 2) % 4 == dealer_index || (i + 2) % 4 == dealer_index + 4;

            if player.strategy.order_up_card(&top_card, &player.hand, dealer_is_self, dealer_is_teammate) {
                println!("Pick it up!");
                // Player decided to tell Dealer to pick it up. Remember trump that is set and the index of the player
                self.current_trump_suit = Option::from(top_card.suit);
                maker_player_index = Some(player_index);
                break;
            } else {
                println!("Pass!");
            }
        }

        // Check if somebody made trump and asked dealer to pick up the top card
        if maker_player_index.is_some() {
            let mut dealer: &mut Player = &mut self.players[dealer_index];
            // Take the card from the top of the deck, give it to the dealer and have them discard a card
            let discard: Card = dealer.strategy.swap_trump_card(top_card, &mut dealer.hand);
            println!("Dealer discarded {}{}", discard.rank, discard.suit);
            self.deck.add_card(discard);
            // Have the maker decide if they are going alone or not
            let maker = &mut self.players[maker_player_index.unwrap()];
            self.maker_going_alone = maker.strategy.go_alone(&maker.hand, top_card.suit);
            println!("Go alone?: {}", self.maker_going_alone.to_string());
        } else {
            println!("Nobody asked the dealer to pick it up. Top card is flipped over");
            // Nobody orderd up trump yet so ask all players if they want to call trump
            for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
                let player_index = i % self.players.len();
                print!("Asking Player {} is they want to choose trump...", player_index + 1);
                let player = &self.players[player_index];
                let dealer_is_self = dealer_index == player_index;

                let trump_chosen = player.strategy.choose_trump(&top_card, &player.hand);
                if trump_chosen.is_some() {
                    println!("chose {} as trump!", trump_chosen.unwrap());
                    self.current_trump_suit = Option::from(top_card.suit);
                    break;
                } else {
                    println!("Pass!");
                }

                /* We have reached the end of the players and nobody has selected trump. We do not
                   enforce "stick the dealer" so we will shuffle and re-deal */
                if dealer_is_self {
                    println!("Nobody called trump. Shuffle and re-deal!");
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::game::{Game};
    use crate::{Player, RandomCardStrategy};

    fn create_players() -> [Player; 4] {
        [
            Player::new(1, 1, RandomCardStrategy),
            Player::new(2, 2, RandomCardStrategy),
            Player::new(3, 1, RandomCardStrategy),
            Player::new(4, 2, RandomCardStrategy),
        ]
    }

    #[test]
    fn new_game_test() {
        let mut game: Game = Game::new(create_players());
        assert_eq!(game.cards_left(), 24);
        assert_eq!(game.players.len(), 4);
    }

    #[test]
    fn deal_cards_test() {
        let mut game = Game::new(create_players());
        game.deal_cards();
        assert_eq!(game.cards_left(), 4);
    }

    #[test]
    fn dealer_randomly_selected_test() {
        let game = Game::new(create_players());
        assert!(game.dealer_number <= 4);
        assert!(game.dealer_number >= 1);
    }

    #[test]
    fn decide_trump_test() {
        let mut game = Game::new(create_players());
        game.deal_cards();
        assert!(game.current_trump_suit.is_none());
        game.decide_trump();
        assert!(game.current_trump_suit.is_some());
    }
}