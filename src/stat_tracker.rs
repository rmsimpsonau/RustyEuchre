pub struct StatTracker {
    games_played: i32,
    pub team_games_won: [i32; 2],
    player_tricks_won: [i32; 4],
    total_tricks_played: i32,
}

impl StatTracker {
    pub fn new() -> Self {
        StatTracker {
            games_played: 0,
            team_games_won: [0, 0],
            player_tricks_won: [0, 0, 0, 0],
            total_tricks_played: 0
        }
    }

    pub fn trick_played(&mut self, winning_player_index: usize) {
        self.player_tricks_won[winning_player_index] += 1;
        self.total_tricks_played += 1;
    }

    pub fn game_played(&mut self, winning_team_index: usize) {
        self.team_games_won[winning_team_index] += 1;
        self.games_played += 1;
    }

    pub(crate) fn print_game_stats(&self) {
        println!("Total tricks played {}", self.total_tricks_played as usize);
        println!("||||||||||||| Game Stats ||||||||||||||||");
        println!("| Player   | Tricks Won   |  Games Won  |");
        println!("| Player 1 | {:>4} ({:>3.0}%) | {:>4} ({:>3.0}%) |", self.player_tricks_won[0],
                 self.player_tricks_won[0] as f32 / self.total_tricks_played as f32 * 100.0, self.team_games_won[0],
                 self.team_games_won[0] as f32 / self.games_played as f32 * 100.0);
        println!("| Player 2 | {:>4} ({:>3.0}%) | {:>4} ({:>3.0}%) |", self.player_tricks_won[1],
                 self.player_tricks_won[1] as f32 / self.total_tricks_played as f32 * 100.0, self.team_games_won[1],
                 self.team_games_won[1] as f32 / self.games_played as f32 * 100.0);
        println!("| Player 3 | {:>4} ({:>3.0}%) | {:>4} ({:>3.0}%) |", self.player_tricks_won[2],
                 self.player_tricks_won[2] as f32 / self.total_tricks_played as f32 * 100.0, self.team_games_won[0],
                 self.team_games_won[0] as f32 / self.games_played as f32 * 100.0);
        println!("| Player 4 | {:>4} ({:>3.0}%) | {:>4} ({:>3.0}%) |", self.player_tricks_won[3],
                 self.player_tricks_won[3] as f32 / self.total_tricks_played as f32 * 100.0, self.team_games_won[1],
                 self.team_games_won[1] as f32 / self.games_played as f32 * 100.0);



        println!("|||||||||||||||||||||||||||||||||||||||||");
    }
}

mod tests {
    use crate::stat_tracker::StatTracker;

    #[test]
    fn trick_played_test() {
        let mut stat_tracker = StatTracker::new();
        stat_tracker.trick_played(0);
        assert_eq!(stat_tracker.player_tricks_won, [1, 0, 0, 0]);
        assert_eq!(stat_tracker.total_tricks_played, 1);
    }

    #[test]
    fn game_played_test() {
        let mut stat_tracker = StatTracker::new();
        stat_tracker.game_played(1);
        assert_eq!(stat_tracker.team_games_won, [0, 1]);
    }
}