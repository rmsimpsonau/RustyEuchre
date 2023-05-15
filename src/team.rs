use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum Team {
    TeamOne,
    TeamTwo
}

impl Team {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Team::TeamOne, Team::TeamTwo].into_iter()
    }
    pub fn opposite_team(&self) -> Team {
        match self {
            Team::TeamOne => Team::TeamTwo,
            Team::TeamTwo => Team::TeamOne
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Team::TeamOne => write!(f, "Team 1"),
            Team::TeamTwo => write!(f, "Team 2")
        }
    }
}

mod tests {
    use crate::team::Team;

    #[test]
    fn opposite_team_test() {
        assert_eq!(Team::opposite_team(&Team::TeamOne), Team::TeamTwo)
    }
}
