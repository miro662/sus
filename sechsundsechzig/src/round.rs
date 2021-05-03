use rand::prelude::*;
use tbsux::playered::Player;

use crate::{
    bidding::{bidding, BidResult},
    contract::{Contract, GameType, Party},
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

pub enum RoundResult {
    Contiune,
    Finished(Vec<Player>, i32, Player),
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

    pub fn handle_move(&mut self, mv: SusMove) -> Result<RoundResult, SechsUndSechzigError> {
        use RoundResult::*;
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
                        Ok(Contiune)
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
                        Ok(Contiune)
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

                    if let Some((winning_party, points)) =
                        Round::immediate_winner(&stashes, drawer, &self.contract)
                    {
                        let winners: Vec<_> = self
                            .contract
                            .players_in_party(&self.variant, &winning_party)
                            .collect();
                        return Ok(Finished(winners, points.clone(), self.contract.dealer));
                    }

                    if self.hands.are_empty() {
                        let (winning_party, points) =
                            Round::winner(&stashes, drawer, &self.contract, self.variant);
                        let winners: Vec<_> = self
                            .contract
                            .players_in_party(&self.variant, &winning_party)
                            .collect();
                        return Ok(Finished(winners, points.clone(), self.contract.dealer));
                    }
                }
                Ok(Contiune)
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

    fn immediate_winner(
        stashes: &Stashes,
        last_drawer: Player,
        contract: &Contract,
    ) -> Option<(Party, i32)> {
        use GameType::*;
        use Party::*;

        let points = stashes.points(contract.game_type.triumph());

        match contract.game_type {
            NonTriumph => None,
            AskingAbout(_) => points
                .iter()
                .filter(|(_, points)| **points >= 66)
                .map(|(party, _)| *party)
                .next()
                .map(|party| {
                    (
                        party,
                        match points[&party.other()] {
                            0 => 3,
                            1..=32 => 2,
                            _ => 1,
                        },
                    )
                }),
            LookingFor(_) => {
                if last_drawer != contract.dealer {
                    Some((NonDealers, 5))
                } else if points[&Dealers] > 66 {
                    Some((Dealers, 5))
                } else {
                    None
                }
            }
            Misery => {
                if points[&Dealers] > 0 {
                    Some((NonDealers, 7))
                } else {
                    None
                }
            }
            Shower => {
                if points[&NonDealers] > 0 {
                    Some((NonDealers, 10))
                } else {
                    None
                }
            }
        }
    }

    fn winner(
        stashes: &Stashes,
        last_drawer: Player,
        contract: &Contract,
        variant: Variant,
    ) -> (Party, i32) {
        use GameType::*;
        use Party::*;

        match contract.game_type {
            NonTriumph => (
                stashes
                    .points(contract.game_type.triumph())
                    .iter()
                    .min_by_key(|(_, points)| *points)
                    .map(|(party, _)| *party)
                    .expect("This is not empty"),
                1,
            ),
            AskingAbout(_) => (contract.players_party(variant, last_drawer), 1),
            LookingFor(_) => (Dealers, 5),
            Misery => (Dealers, 7),
            Shower => (Dealers, 10),
        }
    }
}
