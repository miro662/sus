use rand::prelude::*;
use tbsux::playered::Player;

use crate::variant::Variant;

#[derive(Debug, Clone)]
pub struct Round {
    initial_dealer: Player,
    dealer: Player,
}

impl Round {
    pub fn new(_rng: &mut impl Rng, _variant: &Variant, dealer: Player) -> Round {
        Round {
            initial_dealer: dealer,
            dealer: dealer,
        }
    }

    pub fn first(rng: &mut impl Rng, variant: &Variant) -> Round {
        let random_dealer = rng.next_u32() % variant.number_of_players();
        Round::new(rng, variant, random_dealer)
    }
}
