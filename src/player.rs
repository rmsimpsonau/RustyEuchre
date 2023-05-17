use crate::card::Card;
use crate::hand::Hand;
use crate::strategy::strategy::Strategy;
use crate::team::Team;

pub struct Player {
    pub hand: Hand,
    pub player_number_index: usize,
    pub team: Team,
    pub strategy: Box<dyn Strategy>,
}

impl Player {
    pub fn new(player_number_index: usize, team: Team, strategy: impl Strategy + 'static) -> Self {
        Self {
            hand: Hand::new(),
            player_number_index,
            team,
            strategy: Box::new(strategy),
        }
    }

    pub fn cards_left(&mut self) -> usize {
        self.hand.cards_left()
    }

    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.add_card(card);
    }

    pub fn clear_hand(&mut self) {
        self.hand.cards.clear();
    }
}


#[cfg(test)]
mod tests {
    use crate::card::{Card, Rank, Suit};
    use crate::player::Player;
    use crate::RandomCardStrategy;
    use crate::team::Team::{TeamOne, TeamTwo};

    #[test]
    fn new_player_test() {
        let mut player = Player::new(1, TeamOne, RandomCardStrategy);
        assert_eq!(player.cards_left(), 0);
    }

    #[test]
    fn add_card_to_hand_test() {
        let mut player = Player::new(1, TeamOne, RandomCardStrategy);
        let card: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        player.add_card_to_hand(card);
        assert_eq!(player.cards_left(), 1);
    }

    #[test]
    fn player_team_number_test() {
        let player1 = Player::new(1, TeamOne, RandomCardStrategy);
        let player2 = Player::new(2, TeamTwo, RandomCardStrategy);
        assert_eq!(player1.team, TeamOne);
        assert_eq!(player2.team, TeamTwo);
    }

    #[test]
    fn player_clear_hand_test() {
        let mut player1 = Player::new(1, TeamOne, RandomCardStrategy);
        let card: Card = Card { rank: Rank::Queen, suit: Suit::Spades };
        player1.add_card_to_hand(card);
        player1.clear_hand();
        assert_eq!(player1.cards_left(), 0);
    }
}