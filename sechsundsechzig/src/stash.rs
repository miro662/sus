use std::collections::HashMap;

use crate::{cards::{Card, Suit}, contract::Party, error::{SechsUndSechzigError, SusResult}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stash {
    cards: Vec<Card>,
    declarations: Vec<Suit>,
}

impl Stash {
    const DECLARATION_POINTS: i32 = 20;
    const TRIUMPH_DECLARATION_POINTS: i32 = 40;

    pub fn empty() -> Stash {
        Stash {
            cards: vec![],
            declarations: vec![],
        }
    }

    pub fn points(&self, triumph: Option<Suit>) -> i32 {
        let cards_points: i32 = self.cards.iter().map(Card::points).sum();
        let declaration_points: i32 = self
            .declarations
            .iter()
            .map(|suit| {
                if Some(*suit) == triumph {
                    Stash::TRIUMPH_DECLARATION_POINTS
                } else {
                    Stash::DECLARATION_POINTS
                }
            })
            .sum();
        declaration_points + cards_points
    }

    pub fn declare(&mut self, suit: Suit) {
        self.declarations.push(suit);
    }

    pub fn add_cards<'a>(&mut self, cards: impl Iterator<Item=&'a Card>) {
        for card in cards {
            self.cards.push(*card)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stashes(HashMap<Party, Stash>);

impl Stashes {
    pub fn empty<'a>(parties: impl Iterator<Item=&'a Party>) -> Stashes {
        Stashes(parties.map(|party| (*party, Stash::empty())).collect())
    }

    pub fn stash(&self, party: &Party) -> SusResult<&Stash> {
        if let Some(stash) = self.0.get(party) {
            Ok(stash)
        } else {
            Err(SechsUndSechzigError::InvaildParty)
        }
    }

    pub fn stash_mut(&mut self, party: &Party) -> SusResult<&mut Stash> {
        if let Some(stash) = self.0.get_mut(party) {
            Ok(stash)
        } else {
            Err(SechsUndSechzigError::InvaildPlayer)
        }
    }
}
