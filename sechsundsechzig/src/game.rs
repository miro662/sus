use rand::prelude::*;
use tbsux::prelude::*;

use crate::{
    cards::Card, error::SechsUndSechzigError, hands::Hands, round::Round, score::Score,
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
    type Move = ();
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
            })
        }
    }

    fn move_reducer(&self, _: ()) -> Result<Self, SechsUndSechzigError> {
        Ok(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct SechsUndSechzigView {
    score: Score,
    hands: Hands,
}

impl playered::View for SechsUndSechzigView {
    type PlayerView = SechsUndSechzigPlayerView;

    fn current_player(&self) -> playered::Player {
        0
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
