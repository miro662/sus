use std::{error::Error, fmt};

use tbsux::prelude::*;

use crate::{score::Score, variant::Variant};

pub struct SechsUndSechzig {
    variant: Variant,
}

impl Game for SechsUndSechzig {
    type State = SechsUndSechzigState;
    type Move = ();
    type Result = ();
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

pub struct SechsUndSechzigState {
    score: Score,
}

impl State<SechsUndSechzig> for SechsUndSechzigState {
    fn progress_report(&self) -> ProgressReport<SechsUndSechzig> {
        todo!()
    }

    fn move_reducer(&self, _: ()) -> Result<Self, SechsUndSechzigError> {
        todo!()
    }
}

pub struct SechsUndSechzigView;

impl playered::View for SechsUndSechzigView {
    type PlayerView = ();

    fn current_player(&self) -> playered::Player {
        todo!()
    }

    fn player_view(&self, _: playered::Player) -> Self::PlayerView {
        todo!()
    }
}

#[derive(Debug)]
pub enum SechsUndSechzigError {}

impl fmt::Display for SechsUndSechzigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for SechsUndSechzigError {}
