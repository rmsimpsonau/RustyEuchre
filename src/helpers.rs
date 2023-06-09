use crate::card::{Card, Rank, Suit};
use crate::hand::Hand;

pub fn get_left_bower_suit(trump_suit: Suit) -> Suit {
    match trump_suit {
        Suit::Hearts => Suit::Diamonds,
        Suit::Diamonds => Suit::Hearts,
        Suit::Spades => Suit::Clubs,
        Suit::Clubs => Suit::Spades,
    }
}

pub fn get_next_player_index(index: usize) -> usize {
    (index + 1) % 4
}

pub fn get_card_value(card: Card, trump_suit: Suit, lead_card_suit: Suit) -> i32 {
    let mut value = 0;
    let left_bower_suit: Suit = get_left_bower_suit(trump_suit);

    if card.suit == trump_suit {
        // All trump cards
        match card.rank {
            Rank::Jack => value = 13,
            Rank::Ace => value = 11,
            Rank::King => value = 10,
            Rank::Queen => value = 9,
            Rank::Ten => value = 8,
            Rank::Nine => value = 7,
        }
    } else if card.suit == left_bower_suit && card.rank == Rank::Jack {
        // Left bower
        value = 12
    } else if card.suit == lead_card_suit {
        // Same suit as the lead card
        match card.rank {
            Rank::Ace => value = 6,
            Rank::King => value = 5,
            Rank::Queen => value = 4,
            Rank::Jack => value = 3,
            Rank::Ten => value = 2,
            Rank::Nine => value = 1,
        }
    }
    // All other cards are worth 0 as they cannot win a trick

    value
}

pub fn filter_cards_by_lead_suit(hand: &Hand, lead_suit: Suit) -> Vec<Card> {
    // Filter the cards in the hand that have to be played based on the lead suit
    hand.cards
        .iter()
        .filter(|&card| card_matches_lead_suit(card, lead_suit))
        .cloned()
        .collect()
}

pub fn card_matches_lead_suit(card: &Card, lead_suit: Suit) -> bool {
    lead_suit == card.suit
}

mod tests {
    #[allow(unused_imports)]
    use crate::card::{Card, Rank, Suit};
    use crate::hand::Hand;
    #[allow(unused_imports)]
    use crate::helpers::{get_card_value, get_left_bower_suit, get_next_player_index};
    use crate::helpers::{card_matches_lead_suit, filter_cards_by_lead_suit};
    use crate::RandomCardStrategy;

    #[test]
    fn get_left_bower_test() {
        assert_eq!(get_left_bower_suit(Suit::Hearts), Suit::Diamonds);
        assert_eq!(get_left_bower_suit(Suit::Diamonds), Suit::Hearts);
        assert_eq!(get_left_bower_suit(Suit::Clubs), Suit::Spades);
        assert_eq!(get_left_bower_suit(Suit::Spades), Suit::Clubs);
    }

    #[test]
    fn get_next_player_index_test() {
        assert_eq!(get_next_player_index(0), 1);
        assert_eq!(get_next_player_index(1), 2);
        assert_eq!(get_next_player_index(2), 3);
        assert_eq!(get_next_player_index(3), 0);
    }
    #[test]
    fn get_card_value_test() {
        assert_eq!(get_card_value(Card { rank: Rank::Jack, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 13);
        assert_eq!(get_card_value(Card { rank: Rank::Jack, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 12);
        assert_eq!(get_card_value(Card { rank: Rank::Ace, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 11);
        assert_eq!(get_card_value(Card { rank: Rank::King, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 10);
        assert_eq!(get_card_value(Card { rank: Rank::Queen, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 9);
        assert_eq!(get_card_value(Card { rank: Rank::Ten, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 8);
        assert_eq!(get_card_value(Card { rank: Rank::Nine, suit: Suit::Hearts }, Suit::Hearts, Suit::Diamonds), 7);
        assert_eq!(get_card_value(Card { rank: Rank::Ace, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 6);
        assert_eq!(get_card_value(Card { rank: Rank::King, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 5);
        assert_eq!(get_card_value(Card { rank: Rank::Queen, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 4);
        assert_eq!(get_card_value(Card { rank: Rank::Ten, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 2);
        assert_eq!(get_card_value(Card { rank: Rank::Nine, suit: Suit::Diamonds }, Suit::Hearts, Suit::Diamonds), 1);
        assert_eq!(get_card_value(Card { rank: Rank::Ace, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::King, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Queen, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Jack, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Ten, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Nine, suit: Suit::Spades }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Ace, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::King, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Queen, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Jack, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Ten, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);
        assert_eq!(get_card_value(Card { rank: Rank::Nine, suit: Suit::Clubs }, Suit::Hearts, Suit::Diamonds), 0);

        assert_eq!(get_card_value(Card { rank: Rank::Jack, suit: Suit::Spades }, Suit::Diamonds, Suit::Spades), 3);
    }

    #[test]
    fn filter_cards_by_lead_suit_one_match_test() {
        let mut hand = Hand::new(); // Create a test Hand instance
        let card1: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        let card2: Card = Card { rank: Rank::Nine, suit: Suit::Diamonds };
        hand.add_card(card1);
        hand.add_card(card2);

        let matching_cards: Vec<Card> = filter_cards_by_lead_suit(&hand, Suit::Hearts);

        assert_eq!(matching_cards.len(), 1);
        assert_eq!(matching_cards[0].suit, card1.suit);
        assert_eq!(matching_cards[0].rank, card1.rank);
    }

    #[test]
    fn filter_cards_by_lead_suit_two_match_test() {
        let mut hand = Hand::new(); // Create a test Hand instance
        let card1: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        let card2: Card = Card { rank: Rank::Nine, suit: Suit::Hearts };
        hand.add_card(card1);
        hand.add_card(card2);

        let matching_cards: Vec<Card> = filter_cards_by_lead_suit(&hand, Suit::Hearts);

        assert_eq!(matching_cards.len(), 2);
        assert_eq!(matching_cards[0].suit, card1.suit);
        assert_eq!(matching_cards[0].rank, card1.rank);
        assert_eq!(matching_cards[1].suit, card2.suit);
        assert_eq!(matching_cards[1].rank, card2.rank);
    }

    #[test]
    fn card_matches_lead_suit_test() {
        let mut hand = Hand::new(); // Create a test Hand instance
        let card1: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        let card2: Card = Card { rank: Rank::Nine, suit: Suit::Diamonds };
        hand.add_card(card1);
        hand.add_card(card2);

        assert!(card_matches_lead_suit(&card1, Suit::Hearts));
        assert!(!card_matches_lead_suit(&card2, Suit::Hearts));
    }

    #[test]
    fn filter_cards_by_lead_suit_no_matches_test() {
        let mut hand = Hand::new(); // Create a test Hand instance
        let card1: Card = Card { rank: Rank::Jack, suit: Suit::Hearts };
        let card2: Card = Card { rank: Rank::Nine, suit: Suit::Diamonds };
        let card3: Card = Card { rank: Rank::Nine, suit: Suit::Spades };
        hand.add_card(card1);
        hand.add_card(card2);
        hand.add_card(card3);

        let matching_cards: Vec<Card> = filter_cards_by_lead_suit(&hand, Suit::Clubs);

        assert_eq!(matching_cards.len(), 0);
    }
}