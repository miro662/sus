use std::fmt;
use rand::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}
