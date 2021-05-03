use core::fmt;

use rand::prelude::*;
use tbsux::{playered::Player, prelude::*};

use crate::{
    cards::Card,
    contract::Contract,
    error::SechsUndSechzigError,
    hands::Hands,
    round::{Round, RoundResult},
    score::Score,
    sus_move::SusMove,
    table::Table,
    team::Team,
    variant::Variant,
};

pub struct SechsUndSechzig {
    variant: Variant,
    seed: u64,
}

impl SechsUndSechzig {
    pub fn with_random_seed(variant: Variant) -> SechsUndSechzig {
        SechsUndSechzig {
            variant,
            seed: thread_rng().next_u64(),
        }
    }
}

impl Game for SechsUndSechzig {
    type State = SechsUndSechzigState;
    type Move = SusMove;
    type Result = Score;
    type View = SechsUndSechzigView;
    type Error = SechsUndSechzigError;

    fn initial_state(&self) -> Self::State {
        let mut rng = SeedableRng::seed_from_u64(self.seed);
        SechsUndSechzigState {
            score: Score::empty(self.variant),
            round: Round::first(&mut rng, &self.variant),
            variant: self.variant,
            rng,
        }
    }
}

impl playered::Game for SechsUndSechzig {
    fn no_of_players(&self) -> u32 {
        self.variant.number_of_players()
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigState {
    score: Score,
    rng: StdRng,
    round: Round,
    variant: Variant,
}

impl State<SechsUndSechzig> for SechsUndSechzigState {
    fn progress_report(&self) -> ProgressReport<SechsUndSechzig> {
        use ProgressReport::*;

        if let Some(_) = self.score.winner() {
            Finished(self.score.clone())
        } else {
            InProgress(SechsUndSechzigView {
                score: self.score.clone(),
                hands: self.round.hands().clone(),
                current_player: self.round.current_player(),
                contract: self.round.contract().clone(),
                display_full_hand: self.round.display_full_hand(),
                table: self.round.get_table(),
            })
        }
    }

    fn move_reducer(&self, mv: SusMove) -> Result<Self, SechsUndSechzigError> {
        use RoundResult::*;

        let mut cloned_rng = self.rng.clone();
        let mut cloned_round = self.round.clone();
        let move_result = cloned_round.handle_move(mv)?;

        Ok(SechsUndSechzigState {
            round: match move_result {
                Contiune => cloned_round,
                Finished(_, _, last_game_dealer) => {
                    Round::new(&mut cloned_rng, &self.variant, last_game_dealer)
                }
            },
            rng: cloned_rng,
            variant: self.variant,
            score: {
                let mut cloned_score = self.score.clone();
                if let Finished(players, points, _) = move_result {
                    let teams = Team::for_players(players, self.variant);
                    for team in teams {
                        cloned_score.add_points(&team, points)?;
                    }
                }
                cloned_score
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigView {
    score: Score,
    current_player: Player,
    hands: Hands,
    contract: Contract,
    table: Option<Table>,
    display_full_hand: bool,
}

impl playered::View for SechsUndSechzigView {
    type PlayerView = SechsUndSechzigPlayerView;

    fn current_player(&self) -> playered::Player {
        self.current_player
    }

    fn player_view(&self, player: playered::Player) -> SechsUndSechzigPlayerView {
        let hand: Vec<_> = if let Ok(hand) = self.hands.hand(&player) {
            let mut h: Vec<Card> = if !self.display_full_hand {
                hand.first().map(|c| c.clone()).collect()
            } else {
                hand.full().map(|c| c.clone()).collect()
            };
            use std::cmp::Ordering::*;
            h.sort_by(|l, r| match l.suit.cmp(&r.suit) {
                Equal => l.rank.cmp(&r.rank),
                other => other,
            });
            h
        } else {
            vec![]
        };

        SechsUndSechzigPlayerView {
            score: self.score.clone(),
            contract: self.contract.clone(),
            table: self.table.clone(),
            hand,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigPlayerView {
    score: Score,
    hand: Vec<Card>,
    contract: Contract,
    table: Option<Table>,
}

impl fmt::Display for SechsUndSechzigView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use tbsux::playered::View;

        writeln!(f, "PLAYER {} MOVE\n", self.current_player())?;
        writeln!(f, "{}", self.player_view(self.current_player()))
    }
}

impl fmt::Display for SechsUndSechzigPlayerView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SCORE:\n{}", self.score)?;
        writeln!(f, "CONTRACT:\n{}\n", self.contract)?;

        if let Some(table) = &self.table {
            writeln!(f, "TABLE:\n{}", table)?;
        }

        let hand_view = self
            .hand
            .iter()
            .map(|it| it.to_string())
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or("".to_owned());
        write!(f, "HAND:\n{}", hand_view)
    }
}
