use crate::card::Suit;

pub fn get_left_bower_suit(trump_suit: Suit) -> Suit {
    match trump_suit {
        Suit::Hearts => Suit::Diamonds,
        Suit::Diamonds => Suit::Hearts,
        Suit::Spades => Suit::Clubs,
        Suit::Clubs => Suit::Spades,
    }
}

mod tests {
    use crate::card::{Suit};
    use crate::helpers::get_left_bower_suit;

    #[test]
    fn get_left_bower_test() {
        assert_eq!(get_left_bower_suit(Suit::Hearts), Suit::Diamonds);
        assert_eq!(get_left_bower_suit(Suit::Diamonds), Suit::Hearts);
        assert_eq!(get_left_bower_suit(Suit::Clubs), Suit::Spades);
        assert_eq!(get_left_bower_suit(Suit::Spades), Suit::Clubs);
    }
}