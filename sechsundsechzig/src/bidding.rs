use tbsux::playered::Player;

use crate::{
    contract::{Contract, GameType},
    error::{SechsUndSechzigError, SusResult},
    variant::Variant,
};

#[derive(Debug, Clone, Copy)]
pub enum Bid {
    Pass,
    Raise,
    Game(GameType),
}

pub enum BidResult {
    Finish(Contract),
    Continue(Contract, Player),
}

#[rustfmt::skip]
pub fn bidding(
    current_contract: &Contract,
    bid: &Bid,
    player: Player,
    variant: &Variant,
    initial_dealer: Player
) -> SusResult<BidResult> {
    use Bid::*;
    use BidResult::*;
    use GameType::*;
    let next = |player| current_contract.next_player(player, variant);
    
    match (current_contract, bid, player) {
        // === STAGE 1 - with four cards ===
        // stage 1 is equivalent to current_contract.game_type == NonTriumph

        // last player bids "pass" - proceed to non-triumph game
        (c @ Contract { game_type: NonTriumph, .. }, Pass, player) if next(player) == initial_dealer =>
            Ok(Finish(Contract { ..*c })),

        // last player bids "raise" - raise multiplier, make bidding player dealer and proceed to non-triumph game
        (c @ Contract { game_type: NonTriumph, .. }, Raise, player) if next(player) == initial_dealer =>
            Ok(Finish(Contract { dealer: player, multiplier: c.multiplier * 2, ..*c })),

        // non-last player bids "pass" - do not change contract, move to next player
        (c @ Contract { game_type: NonTriumph, .. }, Pass, player) =>
            Ok(Continue(Contract { ..*c }, next(player))),

        // non-last player bids "raise" - do not change contract, move to next player
        (c @ Contract { game_type: NonTriumph, .. }, Raise, player) =>
            Ok(Continue(Contract { dealer: player, multiplier: c.multiplier * 2, ..*c }, next(player))),

        // player bids "ask-about" - change contract to ask-about, move to stage 2a, same player answers
        (Contract { game_type: NonTriumph, .. }, Game(AskingAbout(t)), player) =>
            Ok(Continue(Contract { game_type: AskingAbout(*t), dealer: player, multiplier: 1 }, player)),

        // == STAGE 2a - players draws rest of cards, player who bid AskingAbout(_) bids
        // stage 2 is equivalent to Contract {AskingAbout(_), dealer, 1}

        // player bids "asking-about" again, with same suit - do not change contract, move to next player and stage 2b
        (c @ Contract { game_type: AskingAbout(_), multiplier: 1, .. }, Game(AskingAbout(s)), player) 
            if c.game_type.triumph() == Some(*s) && c.dealer == player =>
            Ok(Continue(Contract { ..*c }, next(player))),

        // player bids "looking-for", with same suit - set contract to looking-for, move to next player and stage 2b
        (c @ Contract { game_type: AskingAbout(_), multiplier: 1, .. }, Game(LookingFor(s)), player) 
            if c.game_type.triumph() == Some(*s) && c.dealer == player =>
            Ok(Continue(Contract { game_type: LookingFor(*s), ..*c }, next(player))),

        // player bids "shower" - set contract to shower, move to next player and stage 2b
        (c @ Contract { game_type: AskingAbout(_), multiplier: 1, .. }, Game(Shower), player) if c.dealer == player =>
            Ok(Continue(Contract { game_type: Shower, ..*c }, next(player))),

        // == UNIVERSAL MATCHES ==
        // everything else is invaild
        _ => Err(SechsUndSechzigError::InvaildBid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Bid::*;
    use Variant::*;
    use GameType::*;

    #[derive(Debug, PartialEq, Eq)]
    enum TestResult {
        InProgress { contract: Contract, player: Player },
        Finished { contract: Contract },
        Error { error: SechsUndSechzigError },
    }
    use TestResult::*;

    struct BiddingTest {
        initial_dealer: Player,
        variant: Variant,
        moves: Vec<(Player, Bid)>,
        expected: TestResult
    }

    impl BiddingTest {
        fn _test(&self, debug: bool) {
            let initial_state = TestResult::InProgress {
                contract: Contract::initial(self.initial_dealer),
                player: self.initial_dealer,
            };
    
            let folded = self.moves.iter().fold(initial_state, |acc, (expected_player, bid)| match acc {
                TestResult::InProgress { contract, player } => {
                    if debug {
                        print!(
                            "In progress, contract: {:?}, player: {:?}",
                            contract, player
                        )
                    };
                    assert_eq!(*expected_player, player, "Invaild player");
                    match bidding(&contract, bid, player, &self.variant, self.initial_dealer) {
                        Ok(BidResult::Continue(contract, player)) => {
                            if debug {
                                println!(", continuing...")
                            };
                            TestResult::InProgress { contract, player }
                        }
                        Ok(BidResult::Finish(contract)) => {
                            if debug {
                                println!(", finishing...")
                            };
                            TestResult::Finished { contract }
                        }
                        Err(error) => {
                            if debug {
                                println!(", error: {:?}", error)
                            };
                            TestResult::Error { error }
                        }
                    }
                }
                TestResult::Finished { .. } => panic!("Already finished"),
                TestResult::Error { .. } => panic!("Wrong bidding"),
            });
    
            assert_eq!(folded, self.expected);
        }

        fn test(&self) {
            self._test(false);
        }

        #[allow(dead_code)]
        fn debug(&self) {
            self._test(true);
        }
    }

    #[test]
    fn warsaw_three_players() {
        BiddingTest {
            initial_dealer: 0,
            variant: ThreePlayers,
            moves: vec![
                (0, Pass), 
                (1, Pass), 
                (2, Pass)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 0,
                    game_type: NonTriumph,
                    multiplier: 1,
                },
            }
        }.test()
    }

    #[test]
    fn warsaw_four_players() {
        BiddingTest {
            initial_dealer: 0,
            variant: FourPlayers,
            moves: vec![
                (0, Pass), 
                (1, Pass), 
                (2, Pass),
                (3, Pass)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 0,
                    game_type: NonTriumph,
                    multiplier: 1,
                },
            }
        }.test()
    }

    #[test]
    fn counter() {
        BiddingTest {
            initial_dealer: 0,
            variant: ThreePlayers,
            moves: vec![
                (0, Pass), 
                (1, Raise), 
                (2, Pass)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 1,
                    game_type: NonTriumph,
                    multiplier: 2,
                },
            }
        }.test()
    }

    #[test]
    fn recounter() {
        BiddingTest {
            initial_dealer: 0,
            variant: ThreePlayers,
            moves: vec![
                (0, Pass), 
                (1, Raise), 
                (2, Raise)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 2,
                    game_type: NonTriumph,
                    multiplier: 4,
                },
            }
        }.test()
    }

    #[test]
    fn sup() {
        BiddingTest {
            initial_dealer: 0,
            variant: ThreePlayers,
            moves: vec![
                (0, Raise), 
                (1, Raise), 
                (2, Raise)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 2,
                    game_type: NonTriumph,
                    multiplier: 8,
                },
            }
        }.test()
    }

    #[test]
    fn mor() {
        BiddingTest {
            initial_dealer: 0,
            variant: FourPlayers,
            moves: vec![
                (0, Raise), 
                (1, Raise), 
                (2, Raise),
                (3, Raise)
            ],
            expected: Finished {
                contract: Contract {
                    dealer: 3,
                    game_type: NonTriumph,
                    multiplier: 16,
                },
            }
        }.test()
    }
}
