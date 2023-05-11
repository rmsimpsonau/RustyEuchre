use rand::Rng;
use crate::card::Suit;
use crate::deck::Deck;
use crate::player::Player;

pub struct Game {
    deck: Deck,
    players: [Player; 4],
    dealer_number: i8,
    current_trump_suit: Option<Suit>
}

impl Game {
    pub fn new(players: [Player; 4]) -> Self {
        // Choose random player to go first
        let mut rng = rand::thread_rng();
        let dealer_number = rng.gen_range(1..5);

        println!("Dealer is Player {}", dealer_number);

        let game = Game { deck: Deck::new(), players, dealer_number, current_trump_suit: None };
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
        let card = self.deck.top_card();
        let player_left_of_dealer = (self.dealer_number) as usize;
        let dealer_index = (self.dealer_number - 1) as usize;

        println!("Card in front of dealer is {}{}", card.rank, card.suit);
        for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
            let player_index = i % self.players.len();
            print!("Asking Player {} is they want to order up trump...", player_index + 1);
            let player = &self.players[player_index];

            let dealer_is_self = dealer_index == player_index;
            // Check if the current player is 2 seats away from the dealer, meaning you are the dealer's teammate
            let dealer_is_teammate = (i + 2) % 4 == dealer_index || (i + 2) % 4 == dealer_index + 4;

            // println!("Dealer {}, Player {}", dealer_index, player_index);
            // println!("Dealer is self {}, Dealer is teammate {}", dealer_is_self, dealer_is_teammate);

            if player.strategy.order_up_card(&card, &player.hand, dealer_is_self, dealer_is_teammate) {
                println!("PICK IT UP!");
                self.current_trump_suit = Option::from(card.suit);
                break;
            } else {
                println!("Pass!");
            }
        }

        if self.current_trump_suit.is_none() {
            println!("Nobody asked the dealer to pick it up. Top card is flipped over");
            // Nobody orderd up trump yet so ask all players if they want to call trump
            for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
                let player_index = i % self.players.len();
                print!("Asking Player {} is they want to choose trump...", player_index + 1);
                let player = &self.players[player_index];
                let dealer_is_self = dealer_index == player_index;

                let trump_chosen = player.strategy.choose_trump(&card, &player.hand);
                if trump_chosen.is_some() {
                    println!("chose {} as trump!", trump_chosen.unwrap());
                    self.current_trump_suit = Option::from(card.suit);
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