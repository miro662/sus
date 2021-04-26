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
        let shuffled_deck: Vec<_> = Card::shuffled_deck(rng).collect();
        let hands: HashMap<_, _> = (0..variant.number_of_players())
            .map(|player| {
                let f = player as usize * variant.cards_per_player();
                (
                    player,
                    Hand(shuffled_deck[f..f + variant.cards_per_player()].into()),
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

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn deals_correctly_for_three_players() {
        let dealt_hands = Hands::deal(&mut thread_rng(), &Variant::ThreePlayers);
        for player in 0..=2 {
            let player_hand = dealt_hands.hand(&player).unwrap();
            let all_cards: Vec<_> = player_hand.full().collect();
            assert_eq!(8, all_cards.len());
        }
    }

    #[test]
    fn deals_correctly_for_four_players() {
        let dealt_hands = Hands::deal(&mut thread_rng(), &Variant::FourPlayers);
        for player in 0..=3 {
            let player_hand = dealt_hands.hand(&player).unwrap();
            let all_cards: Vec<_> = player_hand.full().collect();
            assert_eq!(6, all_cards.len());
        }
    }

    #[test]
    fn deals_different_cards() {
        let dealt_hands = Hands::deal(&mut thread_rng(), &Variant::ThreePlayers);
        let card_sets: Vec<HashSet<_>> = (0..=2).map(|player| {
            let player_hand = dealt_hands.hand(&player).unwrap();
            player_hand.full().collect()
        }).collect();
        assert!(card_sets[0].is_disjoint(&card_sets[1]));
        assert!(card_sets[0].is_disjoint(&card_sets[2]));
        assert!(card_sets[1].is_disjoint(&card_sets[2]));
    }
}
