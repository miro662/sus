use crate::cards::{Card, Suit};

pub fn greatest_card_in_suit<'a>(
    cards: impl Iterator<Item = &'a Card>,
    suit: &Suit,
) -> Option<&'a Card> {
    cards
        .filter(|Card { suit: s, .. }| suit == s)
        .max_by_key(|Card { rank, .. }| rank)
}
