use std::collections::HashMap;

use rand::prelude::*;
use tbsux::playered::Player;

use crate::{
    cards::Card,
    error::{SechsUndSechzigError, SusResult},
    variant::Variant,
};

#[derive(Debug, Copy, Clone)]
pub enum HandType {
    First,
    Full,
}

#[derive(Debug, Clone)]
pub struct Hand(Vec<Card>);

impl Hand {
    const FIRST_HAND_LEN: usize = 4;

    pub fn full(&self) -> impl Iterator<Item = &Card> {
        self.0.iter()
    }

    pub fn first(&self) -> impl Iterator<Item = &Card> {
        self.0.iter().take(Self::FIRST_HAND_LEN)
    }
}

#[derive(Debug, Clone)]
pub struct Hands(HashMap<Player, Hand>);

impl Hands {
    pub fn deal(rng: &mut impl Rng, variant: &Variant) -> Hands {
        let mut shuffled_deck: Vec<_> = Card::shuffled_deck(rng).collect();
        let hands: HashMap<_, _> = (0..variant.number_of_players())
            .map(|player| {
                (
                    player,
                    Hand(shuffled_deck.split_off(variant.cards_per_player())),
                )
            })
            .collect();
        Hands(hands)
    }

    pub fn hand(&self, player: &Player) -> SusResult<&Hand> {
        if let Some(hand) = self.0.get(player) {
            Ok(hand)
        } else {
            Err(SechsUndSechzigError::InvaildPlayer)
        }
    }
}
