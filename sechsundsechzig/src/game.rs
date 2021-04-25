use core::fmt;

use rand::prelude::*;
use tbsux::{playered::Player, prelude::*};

use crate::{cards::Card, error::SechsUndSechzigError, hands::Hands, round::Round, score::Score, sus_move::SusMove, variant::Variant};

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
                current_player: 0
            })
        }
    }

    fn move_reducer(&self, _: SusMove) -> Result<Self, SechsUndSechzigError> {
        Ok(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigView {
    score: Score,
    current_player: Player,
    hands: Hands,
}

impl playered::View for SechsUndSechzigView {
    type PlayerView = SechsUndSechzigPlayerView;

    fn current_player(&self) -> playered::Player {
        self.current_player
    }

    fn player_view(&self, player: playered::Player) -> SechsUndSechzigPlayerView {
        let hand: Vec<_> = if let Ok(hand) = self.hands.hand(&player) {
            hand.first().map(|c| c.clone()).collect()
        } else {
            vec![]
        };

        SechsUndSechzigPlayerView {
            score: self.score.clone(),
            hand,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigPlayerView {
    score: Score,
    hand: Vec<Card>,
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
        let hand_view = self
            .hand
            .iter()
            .map(|it| it.to_string())
            .reduce(|a, b| format!("{} {}", a, b))
            .unwrap_or("".to_owned());
        writeln!(f, "HAND:\n{}", hand_view)
    }
}
