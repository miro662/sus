use rand::prelude::*;
use tbsux::playered::Player;

use crate::{
    bidding::{bidding, BidResult},
    contract::{Contract, GameType},
    error::SechsUndSechzigError,
    hands::Hands,
    stash::Stashes,
    sus_move::SusMove,
    table::Table,
    variant::Variant,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stage {
    Bidding(Player),
    Play { table: Table, stashes: Stashes },
}

#[derive(Debug, Clone)]
pub struct Round {
    variant: Variant,
    initial_dealer: Player,
    hands: Hands,
    contract: Contract,
    stage: Stage,
}

impl Round {
    pub fn new(rng: &mut impl Rng, variant: &Variant, dealer: Player) -> Round {
        Round {
            variant: *variant,
            initial_dealer: dealer,
            hands: Hands::deal(rng, variant),
            contract: Contract::initial(dealer),
            stage: Stage::Bidding(dealer),
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

        match &self.stage {
            Bidding(p) => *p,
            Play { table, .. } => table.current_player().expect("Should create new table"),
        }
    }

    pub fn handle_move(&mut self, mv: SusMove) -> Result<(), SechsUndSechzigError> {
        use Stage::*;
        use SusMove::*;

        let current_player = self.current_player();
        match (&mut self.stage, mv) {
            (Bidding(player), BiddingMove(bid)) => {
                match bidding(
                    &self.contract,
                    &bid,
                    *player,
                    &self.variant,
                    self.initial_dealer,
                )? {
                    BidResult::Continue(new_contract, new_player) => {
                        self.contract = new_contract;
                        self.stage = Bidding(new_player);
                        Ok(())
                    }
                    BidResult::Finish(final_contract) => {
                        self.contract = final_contract;
                        self.stage = Play {
                            table: Table::empty(
                                self.variant,
                                self.contract.clone(),
                                self.contract.dealer,
                            ),
                            stashes: Stashes::empty(self.contract.parties(&self.variant)),
                        };
                        Ok(())
                    }
                }
            }
            (Play { table, stashes }, PlayMove(card)) => {
                let hand = self.hands.hand_mut(&current_player).expect("Correct hand");
                let is_declaration = hand.can_declare(card) && self.contract.can_declare();
                table.try_play_card(hand, card)?;

                if let Some(drawer) = table.drawer() {
                    let drawing_party = self.contract.players_party(self.variant, drawer);
                    {
                        let drawing_party_stash = stashes.stash_mut(&drawing_party)?;
                        if is_declaration {
                            drawing_party_stash.declare(card.suit);
                        }
                        drawing_party_stash.add_cards(table.cards());
                    }
                    *table = Table::empty(self.variant, self.contract.clone(), drawer);
                }
                Ok(())
            }
            _ => Err(SechsUndSechzigError::WrongStage),
        }
    }

    pub fn display_full_hand(&self) -> bool {
        if let Stage::Bidding(_) = self.stage {
            self.contract.game_type != GameType::NonTriumph
        } else {
            true
        }
    }

    pub fn get_table(&self) -> Option<Table> {
        match &self.stage {
            Stage::Play { table, .. } => Some(table.clone()),
            _ => None,
        }
    }
}
