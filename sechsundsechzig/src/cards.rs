use rand::prelude::*;
use std::{fmt, str::FromStr};

use crate::error::SechsUndSechzigError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Rank {
    Nine,
    Jack,
    Queen,
    King,
    Ten,
    Ace,
}

impl Rank {
    pub const RANKS: [Rank; 6] = [
        Rank::Nine,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ten,
        Rank::Ace,
    ];
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Rank::*;
        write!(
            f,
            "{}",
            match self {
                Nine => "9",
                Jack => "J",
                Queen => "Q",
                King => "K",
                Ten => "10",
                Ace => "A",
            }
        )
    }
}

impl FromStr for Rank {
    type Err = SechsUndSechzigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rank::*;
        match &s.to_lowercase() as &str {
            "9" => Ok(Nine),
            "1" | "10" => Ok(Ten),
            "j" | "jack" => Ok(Jack),
            "q" | "queen" => Ok(Queen),
            "k" | "king" => Ok(King),
            "a" | "ace" => Ok(Ace),
            _ => Err(SechsUndSechzigError::RankParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

impl Suit {
    pub const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Suit::*;
        write!(
            f,
            "{}",
            match self {
                Spade => "♠",
                Club => "♣",
                Heart => "♥",
                Diamond => "♦",
            }
        )
    }
}

impl FromStr for Suit {
    type Err = SechsUndSechzigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Suit::*;
        match &s.to_lowercase() as &str {
            "♠" | "spade" | "spades" | "s" => Ok(Spade),
            "♣" | "club" | "clubs" | "c" => Ok(Club),
            "♥" | "heart" | "hearts" | "h" => Ok(Heart),
            "♦" | "diamond" | "diamonds" | "d" => Ok(Diamond),
            _ => Err(SechsUndSechzigError::SuitParseError),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn deck() -> impl Iterator<Item = Card> {
        Rank::RANKS.iter().flat_map(|rank| {
            Suit::SUITS.iter().map(move |suit| Card {
                rank: *rank,
                suit: *suit,
            })
        })
    }

    pub fn shuffled_deck(rng: &mut impl Rng) -> impl Iterator<Item = Card> {
        let mut deck: Vec<_> = Card::deck().collect();
        deck.shuffle(rng);
        deck.into_iter()
    }

    pub fn points(&self) -> i32 {
        use Rank::*;
        match self.rank {
            Nine => 0,
            Jack => 2,
            Queen => 3,
            King => 4,
            Ten => 10,
            Ace => 11,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.rank, self.suit)
    }
}

impl FromStr for Card {
    type Err = SechsUndSechzigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<_> = s.split(" ").collect();
        if split_str.len() == 2 {
            let rank: Rank = split_str[0].parse()?;
            let suit: Suit = split_str[1].parse()?;
            Ok(Card { rank, suit })
        } else {
            Err(SechsUndSechzigError::InvaildPlayer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACE_OF_SPADES: Card = Card {
        rank: Rank::Ace,
        suit: Suit::Spade,
    };

    #[test]
    fn deck_has_correct_number_of_items() {
        let deck: Vec<_> = Card::deck().collect();
        assert_eq!(24, deck.len())
    }

    #[test]
    fn ace_of_spades_symbol_is_correct() {
        assert_eq!("[A ♠]", ACE_OF_SPADES.to_string())
    }

    #[test]
    fn ace_of_spades_is_worth_11_points() {
        assert_eq!(11, ACE_OF_SPADES.points())
    }

    #[test]
    fn whole_deck_is_worth_120_points() {
        assert_eq!(120, Card::deck().map(|c| c.points()).sum())
    }

    #[test]
    fn ace_of_spades_parsed_correctly() {
        assert_eq!(ACE_OF_SPADES, "a s".parse().unwrap());
        assert_eq!(ACE_OF_SPADES, "A spade".parse().unwrap());
        assert_eq!(ACE_OF_SPADES, "aCe sPaDeS".parse().unwrap());
    }
}
