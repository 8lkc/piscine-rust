extern crate rand;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    // Generate a random suit
    pub fn random() -> Suit {
        match rand::thread_rng().gen_range(1, 5) {  // Corrected range
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }

    // Translate a u8 value into a suit
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for Suit"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    // Generate a random rank
    pub fn random() -> Rank {
        match rand::thread_rng().gen_range(1, 14) {  // Corrected range for Rank
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n as u8),
        }
    }

    // Translate a u8 value into a rank
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid value for Rank"),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Check if the card is an Ace of Spades
pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let your_card = Card {rank: Rank::random(), suit: Suit::random()};
    }
}
