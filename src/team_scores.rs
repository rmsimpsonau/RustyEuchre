use crate::team::Team;
use crate::TeamOne;

pub struct TeamScores {
    pub team_one: i32,
    pub team_two: i32
}

impl TeamScores {
    pub fn new() -> Self {
        TeamScores { team_one: 0, team_two: 0 }
    }

    pub fn add_points_to_team(&mut self, num_pts: i32, team: Team) {
        if team == TeamOne {
            self.team_one += num_pts;
        } else {
            self.team_two += num_pts;
        }
    }

    pub(crate) fn print_current_score(&self) {
        println!("+---------------+");
        println!("| Current Score |");
        println!("|   Team 1 - {}  |", self.team_one as usize);
        println!("|   Team 2 - {}  |", self.team_two as usize);
        println!("+---------------+");
    }
}


mod tests {
    #[allow(unused_imports)]
    use crate::team_scores::TeamScores;
    #[allow(unused_imports)]
    use crate::{TeamOne, TeamTwo};

    #[test]
    fn add_points_to_team_test() {
        let mut team_scores = TeamScores::new();
        team_scores.add_points_to_team(2, TeamOne);
        team_scores.add_points_to_team(3, TeamTwo);
        assert_eq!(team_scores.team_one, 2);
        assert_eq!(team_scores.team_two, 3);
    }
}