use rand::Rng;
// use rocket_contrib::templates::handlebars::Helper;
use crate::card::{Card, Suit};
use crate::deck::Deck;
use crate::helpers::get_next_player_index;
use crate::player::Player;
use crate::team::Team;
use crate::{TeamOne, TeamTwo};
use crate::stat_tracker::StatTracker;
use crate::team_scores::TeamScores;
use crate::trick::Trick;

pub struct Game {
    deck: Deck,
    players: [Player; 4],
    dealer_index: i8,
    pub current_trump_suit: Option<Suit>,
    maker_going_alone: bool,
    making_team: Option<Team>,
    team_scores: TeamScores
}

impl Game {
    pub fn new(players: [Player; 4]) -> Self {
        // Choose random player to go first
        let mut rng = rand::thread_rng();
        let dealer_index = rng.gen_range(0..4);

        println!("Dealer is Player {}", dealer_index + 1);

        let game = Game { deck: Deck::new(), players, dealer_index, current_trump_suit: None,
            maker_going_alone: false, making_team: None, team_scores: TeamScores::new() };
        game
    }

    pub fn reset_cards(&mut self) {
        // Reset current trump suit
        self.current_trump_suit = None;
        // Clear Player's Hands
        for player in &mut self.players {
            player.clear_hand();
        }
        // Reset the Deck to include all cards again
        self.deck.populate();
        self.deck.shuffle();
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
            println!("Player {} Cards: ", i as usize);
            for card in &player.hand.cards {
                print!(" {}{} ", card.rank, card.suit);
            }
            println!();
            i += 1;
        }
    }

    pub fn decide_trump(&mut self) {
        // Get card from top of the deck
        let top_card = self.deck.take_top_card();
        let player_left_of_dealer = get_next_player_index(self.dealer_index as usize);
        let mut maker_player_index: Option<usize> = None;

        /* TOP CARD IS SHOWING */

        // Starting with the player left of the dealer, ask if players want to order up top card
        println!("Card in front of dealer is {}{}", top_card.rank, top_card.suit);
        for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
            let player_index = i % self.players.len();
            print!("Asking Player {} is they want to order up trump...", player_index + 1);
            let player = &mut self.players[player_index];

            let dealer_is_self = self.dealer_index as usize == player_index;
            // Check if the current player is 2 seats away from the dealer, meaning you are the dealer's teammate
            let dealer_is_teammate = (i + 2) % 4 == (self.dealer_index as usize) || (i + 2) % 4 == ((self.dealer_index + 4) as usize);

            if player.strategy.order_up_card(&top_card, &player.hand, dealer_is_self, dealer_is_teammate) {
                println!("Pick it up!");
                // Player decided to tell Dealer to pick it up. Remember trump that is set and the index of the player
                self.current_trump_suit = Some(top_card.suit);
                self.making_team = Some(player.team);
                maker_player_index = Some(player_index);
                break;
            } else {
                println!("Pass!");
            }
        }

        // Check if somebody made trump and asked dealer to pick up the top card
        if maker_player_index.is_some() {
            let dealer: &mut Player = &mut self.players[self.dealer_index as usize];
            // Take the card from the top of the deck, give it to the dealer and have them discard a card
            let discard: Card = dealer.strategy.swap_trump_card(top_card, &mut dealer.hand);
            println!("Dealer (Player {}) discarded {}{}", dealer.player_number_index, discard.rank, discard.suit);
            self.deck.add_card(discard);
            // Have the maker decide if they are going alone or not
            let maker = &mut self.players[maker_player_index.unwrap()];
            self.maker_going_alone = maker.strategy.go_alone(&maker.hand, top_card.suit);
            println!("Go alone?: {}", self.maker_going_alone.to_string());
            println!("{} are the makers, {} are the defenders", self.making_team.unwrap(), Team::opposite_team(&self.making_team.unwrap()))
        } else {

            /* TOP CARD FLIPPED OVER - ASK IF ANYBODY WANTS TO MAKE TRUMP */

            println!("Nobody asked the dealer to pick it up. Top card is flipped over");
            // Nobody orderd up trump yet so ask all players if they want to call trump
            for i in player_left_of_dealer..player_left_of_dealer + self.players.len() {
                let player_index = i % self.players.len();
                print!("Asking Player {} is they want to choose trump...", player_index + 1);
                let player = &self.players[player_index];
                let dealer_is_self = self.dealer_index as usize == player_index;

                let trump_chosen = player.strategy.choose_trump(&top_card, &player.hand);
                if trump_chosen.is_some() {
                    println!("chose {} as trump!", trump_chosen.unwrap());
                    self.current_trump_suit = Some(top_card.suit);
                    self.making_team = Some(player.team);
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

    pub fn play_trick(&mut self, lead_player_index: usize) -> Trick {
        println!("Player {} will lead", lead_player_index + 1);
        let mut trick: Trick = Trick::new(
            self.current_trump_suit.expect("Trump should be set"));

        // Go through each Player and ask what Card they would each like to play
        for i in lead_player_index..lead_player_index + self.players.len() {
            let player_index = i % self.players.len();
            let player = &mut self.players[player_index];
            let card_to_play: Card = player.strategy.select_card_to_play(&mut player.hand, &mut trick);
            println!("Player {} plays {}{}", player_index + 1, card_to_play.rank, card_to_play.suit);
            trick.card_played(card_to_play, player_index);
        }

        trick
    }

    pub fn get_player_index_left_of_dealer(&self) -> usize {
        get_next_player_index(self.dealer_index as usize)
    }

    pub fn pass_deal_to_left(&mut self) {
        self.dealer_index = (self.dealer_index + 1) % 4;
    }

    pub fn play_hand(&mut self, stat_tracker: &mut StatTracker) {
        let mut starting_player_index: usize = self.get_player_index_left_of_dealer();
        let mut tricks: Vec<Trick> = vec![];
        let mut team_one_tricks_taken = 0;
        let mut team_two_tricks_taken = 0;
        // Play a Trick 5 times so each Player uses all of their Cards
        for _ in 0..5 {
            println!("==================================================");
            let trick: Trick = self.play_trick(starting_player_index);
            let highest_card_player_index = trick.highest_card_player_index.expect("Highest card player should be set");
            starting_player_index = highest_card_player_index;
            // Track and print results
            stat_tracker.trick_played(highest_card_player_index);
            trick.print_results();
            if trick.highest_card_player_index.unwrap() == 0 || trick.highest_card_player_index.unwrap() == 2 {
                team_one_tricks_taken += 1;
            } else {
                team_two_tricks_taken += 1;
            }
            tricks.push(trick);
        }

        // Determine who won the most tricks
        let mut winning_team = TeamOne;
        let mut winning_team_tricks_taken = team_one_tricks_taken;
        if team_two_tricks_taken > team_one_tricks_taken {
            winning_team = TeamTwo;
            winning_team_tricks_taken = team_two_tricks_taken;
        }

        self.add_points_won(winning_team, winning_team_tricks_taken);
        self.team_scores.print_current_score();
        println!("==================================================");
    }

    pub fn add_points_won(&mut self, winning_team: Team, winning_team_tricks_taken: i32) {
        // Determine how many points the winning team gets
        if self.making_team.unwrap() == winning_team {
            if winning_team_tricks_taken == 5 {
                if self.maker_going_alone {
                    println!("4 points - 1");
                    // Winning team won ALL of the tricks while going alone - 4 points
                    self.team_scores.add_points_to_team(4, winning_team);
                } else {
                    println!("2 points - 2");
                    // Winning team won ALL of the tricks while NOT going alone - 2 points
                    self.team_scores.add_points_to_team(2, winning_team);
                }
            } else {
                println!("1 points - 3");
                // Winning team won at least 3, but not all 5 - 1 point
                self.team_scores.add_points_to_team(1, winning_team);
            }
        } else {
            // Making team was Euchred
            if winning_team_tricks_taken == 5 {
                println!("4 points - 4");
                // Winning team Euchred the making team and won all 5 tricks - 4 points
                self.team_scores.add_points_to_team(4, winning_team);
            } else {
                println!("2 points - 5");
                // Winning team Euchred the making team and won at least 3 tricks - 2 points
                self.team_scores.add_points_to_team(2, winning_team);
            }
        }
    }

    pub fn game_over(&self) -> bool {
        // Game is over once one team reaches 10 points
        self.team_scores.team_one >= 10 || self.team_scores.team_two >= 10
    }

    pub fn winning_team_index(&self) -> usize {
        // Winning team's index. TeamOne = 0, TeamTwo = 1
        let mut team_index = 0;
        if self.game_over() {
            if self.team_scores.team_two > self.team_scores.team_one {
                team_index = 1
            }
        } else {
            panic!("The game is not over. Why are you requesting the winning team index?");
        }

        team_index
    }


}


#[cfg(test)]
mod tests {
    use crate::game::{Game};
    use crate::{Player, RandomCardStrategy, TeamOne, TeamTwo};
    use crate::stat_tracker::StatTracker;
    use crate::trick::Trick;

    fn create_players() -> [Player; 4] {
        [
            Player::new(1, TeamOne, RandomCardStrategy),
            Player::new(2, TeamTwo, RandomCardStrategy),
            Player::new(3, TeamOne, RandomCardStrategy),
            Player::new(4, TeamTwo, RandomCardStrategy),
        ]
    }

    #[test]
    fn new_game_test() {
        let mut game: Game = Game::new(create_players());
        assert_eq!(game.cards_left(), 24);
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.team_scores.team_one, 0);
        assert_eq!(game.team_scores.team_two, 0);
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
        assert!(game.dealer_index <= 3);
        assert!(game.dealer_index >= 0);
    }

    #[test]
    fn get_player_index_left_of_dealer_test() {
        let mut game = Game::new(create_players());
        game.dealer_index = 1;
        assert_eq!(game.get_player_index_left_of_dealer(), 2);
        game.dealer_index = 3;
        assert_eq!(game.get_player_index_left_of_dealer(), 0);
    }

    #[test]
    fn decide_trump_test() {
        let mut game = Game::new(create_players());
        game.deal_cards();
        assert!(game.current_trump_suit.is_none());
        game.decide_trump();
        assert!(game.current_trump_suit.is_some());
    }

    #[test]
    fn redeal_test() {
        let mut game = Game::new(create_players());
        game.deal_cards();
        game.decide_trump();
        game.reset_cards();
        assert!(game.current_trump_suit.is_none());
        assert_eq!(game.players[0].hand.cards_left(), 0)
    }

    #[test]
    fn play_trick_test() {
        let mut game: Game = Game::new(create_players());
        game.deal_cards();
        game.decide_trump();
        let trick: Trick = game.play_trick(0);
        assert_eq!(trick.cards_played.len(), 4);
        assert!(trick.highest_card.is_some());
        assert!(trick.lead_card.is_some());
        assert_eq!(game.players[0].hand.cards_left(), 4);
        assert_eq!(game.players[1].hand.cards_left(), 4);
        assert_eq!(game.players[2].hand.cards_left(), 4);
        assert_eq!(game.players[3].hand.cards_left(), 4);
    }

    #[test]
    fn play_hand_test() {
        let mut game = Game::new(create_players());
        game.deal_cards();
        game.decide_trump();
        game.play_hand(&mut StatTracker::new());
        assert_eq!(game.players[0].hand.cards_left(), 0);
        assert_eq!(game.players[1].hand.cards_left(), 0);
        assert_eq!(game.players[2].hand.cards_left(), 0);
        assert_eq!(game.players[3].hand.cards_left(), 0);
    }

    #[test]
    fn add_points_won_test() {
        let mut game = Game::new(create_players());
        game.making_team = Some(TeamOne);
        game.maker_going_alone = true;
        game.add_points_won(TeamOne, 5);
        assert_eq!(game.team_scores.team_one, 4);
        assert_eq!(game.team_scores.team_two, 0);

        game.making_team = Some(TeamOne);
        game.maker_going_alone = true;
        game.add_points_won(TeamOne, 4);
        assert_eq!(game.team_scores.team_one, 5);
        assert_eq!(game.team_scores.team_two, 0);

        game.making_team = Some(TeamOne);
        game.maker_going_alone = true;
        game.add_points_won(TeamTwo, 3);
        assert_eq!(game.team_scores.team_one, 5);
        assert_eq!(game.team_scores.team_two, 2);

        game.making_team = Some(TeamOne);
        game.maker_going_alone = true;
        game.add_points_won(TeamTwo, 5);
        assert_eq!(game.team_scores.team_one, 5);
        assert_eq!(game.team_scores.team_two, 6);
    }

    #[test]
    fn game_over_test() {
        let mut game = Game::new(create_players());
        game.making_team = Some(TeamOne);
        game.maker_going_alone = true;
        assert!(!game.game_over());
        game.add_points_won(TeamOne, 5);
        assert!(!game.game_over());
        game.add_points_won(TeamOne, 5);
        assert!(!game.game_over());
        game.add_points_won(TeamOne, 5);
        assert!(game.game_over());
        assert_eq!(game.winning_team_index(), 0);
    }

    #[test]
    fn pass_deal_to_left_test() {
        let mut game = Game::new(create_players());
        let starting_dealer_index = game.dealer_index;
        game.pass_deal_to_left();
        assert_eq!(game.dealer_index, (starting_dealer_index + 1) % 5);
    }
}