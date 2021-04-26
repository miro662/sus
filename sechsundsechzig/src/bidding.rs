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

    #[derive(Debug, PartialEq, Eq)]
    enum TestResult {
        InProgress { contract: Contract, player: Player },
        Finished { contract: Contract },
        Error { error: SechsUndSechzigError },
    }

    fn test_bidding<'a>(
        variant: Variant,
        moves: impl Iterator<Item = &'a Bid>,
        expected: TestResult,
        debug: bool,
    ) {
        let initial_dealer = 0;
        let initial_state = TestResult::InProgress {
            contract: Contract::initial(initial_dealer),
            player: initial_dealer,
        };

        let folded = moves.fold(initial_state, |acc, x| match acc {
            TestResult::InProgress { contract, player } => {
                if debug {
                    print!(
                        "In progress, contract: {:?}, player: {:?}",
                        contract, player
                    )
                };
                match bidding(&contract, x, player, &variant, initial_dealer) {
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

        assert_eq!(folded, expected);
    }

    #[test]
    fn warsaw_three_players() {
        test_bidding(
            Variant::ThreePlayers,
            vec![Bid::Pass, Bid::Pass, Bid::Pass].iter(),
            TestResult::Finished {
                contract: Contract {
                    dealer: 0,
                    game_type: GameType::NonTriumph,
                    multiplier: 1,
                },
            },
            false,
        );
    }

    #[test]
    fn warsaw_four_players() {
        test_bidding(
            Variant::FourPlayers,
            vec![Bid::Pass, Bid::Pass, Bid::Pass, Bid::Pass].iter(),
            TestResult::Finished {
                contract: Contract {
                    dealer: 0,
                    game_type: GameType::NonTriumph,
                    multiplier: 1,
                },
            },
            false,
        );
    }
}
