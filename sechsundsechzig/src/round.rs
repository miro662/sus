use rand::prelude::*;
use tbsux::playered::Player;

use crate::{bidding::{bidding, BidResult}, contract::Contract, error::SechsUndSechzigError, hands::Hands, sus_move::SusMove, variant::Variant};

#[derive(Debug, Clone)]
enum Stage {
    Bidding(Player),
    Play
}

#[derive(Debug, Clone)]
pub struct Round {
    variant: Variant,
    initial_dealer: Player,
    hands: Hands,
    contract: Contract,
    stage: Stage
}

impl Round {
    pub fn new(rng: &mut impl Rng, variant: &Variant, dealer: Player) -> Round {
        Round {
            variant: *variant,
            initial_dealer: dealer,
            hands: Hands::deal(rng, variant),
            contract: Contract::initial(dealer),
            stage: Stage::Bidding(dealer)
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

    pub fn current_player(&self) -> Player {
        use Stage::*;

        match self.stage {
            Bidding(p) => p,
            Play => todo!("Play is not yet implemented")
        }
    }

    pub fn handle_move(&mut self, mv: SusMove) -> Result<(), SechsUndSechzigError> {
        use SusMove::*;
        use Stage::*;

        match (&self.stage, mv) {
            (Bidding(player), BiddingMove(bid)) => {
                match bidding(&self.contract, &bid, *player, &self.variant, self.initial_dealer)? {
                    BidResult::Continue(new_contract, new_player) => {
                        self.contract = new_contract;
                        self.stage = Bidding(new_player);
                        Ok(())
                    },
                    BidResult::Finish(final_contract) => {
                        self.contract = final_contract;
                        self.stage = Play;
                        Ok(())
                    }
                }
            },
            (Play, PlayMove(_)) => {
                todo!("Play is not yet implemented")
            }
            _ => Err(SechsUndSechzigError::WrongStage)
        }
    }
}
