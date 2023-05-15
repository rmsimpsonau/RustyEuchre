use crate::card::Suit;

pub fn get_left_bower_suit(trump_suit: Suit) -> Suit {
    match trump_suit {
        Suit::HEARTS => Suit::DIAMONDS,
        Suit::DIAMONDS => Suit::HEARTS,
        Suit::SPADES => Suit::CLUBS,
        Suit::CLUBS => Suit::SPADES,
    }
}

mod tests {
    use crate::card::{Suit};
    use crate::helpers::get_left_bower_suit;

    #[test]
    fn get_left_bower_test() {
        assert_eq!(get_left_bower_suit(Suit::HEARTS), Suit::DIAMONDS);
        assert_eq!(get_left_bower_suit(Suit::DIAMONDS), Suit::HEARTS);
        assert_eq!(get_left_bower_suit(Suit::CLUBS), Suit::SPADES);
        assert_eq!(get_left_bower_suit(Suit::SPADES), Suit::CLUBS);
    }
}