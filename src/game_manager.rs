use crate::{Game, Player, RandomCardStrategy, TeamOne, TeamTwo};
use crate::stat_tracker::StatTracker;

pub struct GameManager {}

impl GameManager {
    pub fn run_game(number_of_plays: i32) {
        // Track stats
        let mut stat_tracker = StatTracker::new();

        for _ in 0..number_of_plays {

            // Create players
            let players: [Player; 4] = [
                Player::new(1, TeamOne, RandomCardStrategy),
                Player::new(2, TeamTwo, RandomCardStrategy),
                Player::new(3, TeamOne, RandomCardStrategy),
                Player::new(4, TeamTwo, RandomCardStrategy)
            ];

            // Create a new game with the players
            let mut game = Game::new(players);

            loop {
                game.reset_cards();
                game.deal_cards();
                game.decide_trump();
                if game.current_trump_suit.is_some() {
                    // Trump is set so start the first Trick, otherwise we redeal
                    println!("Playing the hand...");
                    game.play_hand(&mut stat_tracker);
                }
                if game.game_over() {
                    stat_tracker.game_played(game.winning_team_index());
                    break;
                } else {
                    game.pass_deal_to_left();
                }
            }
        }

        stat_tracker.print_game_stats();
    }
}