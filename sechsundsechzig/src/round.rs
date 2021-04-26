use rand::prelude::*;
use tbsux::playered::Player;

use crate::{contract::Contract, hands::Hands, variant::Variant};

#[derive(Debug, Clone)]
pub struct Round {
    initial_dealer: Player,
    hands: Hands,
    contract: Contract,
}

impl Round {
    pub fn new(rng: &mut impl Rng, variant: &Variant, dealer: Player) -> Round {
        Round {
            initial_dealer: dealer,
            hands: Hands::deal(rng, variant),
            contract: Contract::initial(dealer),
        }
    }

    pub fn first(rng: &mut impl Rng, variant: &Variant) -> Round {
        let random_dealer = rng.gen_range(0..variant.number_of_players());
        Round::new(rng, variant, random_dealer)
    }

    pub fn hands(&self) -> &Hands {
        &self.hands
    }

    pub fn contract(&self) -> &Contract {
        &self.contract
    }
}
