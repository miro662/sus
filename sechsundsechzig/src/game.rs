use tbsux::prelude::*;

use crate::{error::SechsUndSechzigError, score::Score, variant::Variant};

pub struct SechsUndSechzig {
    variant: Variant,
}

impl Game for SechsUndSechzig {
    type State = SechsUndSechzigState;
    type Move = ();
    type Result = Score;
    type View = SechsUndSechzigView;
    type Error = SechsUndSechzigError;

    fn initial_state(&self) -> Self::State {
        SechsUndSechzigState {
            score: Score::empty(self.variant),
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
}

impl State<SechsUndSechzig> for SechsUndSechzigState {
    fn progress_report(&self) -> ProgressReport<SechsUndSechzig> {
        use ProgressReport::*;

        if let Some(_) = self.score.winner() {
            Finished(self.score.clone())
        } else {
            InProgress(SechsUndSechzigView {
                score: self.score.clone(),
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
}

impl playered::View for SechsUndSechzigView {
    type PlayerView = SechsUndSechzigView;

    fn current_player(&self) -> playered::Player {
        0
    }

    fn player_view(&self, _: playered::Player) -> Self::PlayerView {
        self.clone()
    }
}
