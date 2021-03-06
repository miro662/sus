use std::{cmp::Ordering, error::Error, fmt::Display};

use rand::Rng;

use tbsux::{cli::run_cli, prelude::*};

struct GuessANumber {
    max_number: u32,
}

impl Game for GuessANumber {
    type State = GuessANumberState;
    type Move = u32;
    type Result = u32;
    type View = GuessANumberView;
    type Error = GuessANumberError;

    fn initial_state(&self) -> Self::State {
        let mut rng = rand::thread_rng();
        GuessANumberState {
            max_number: self.max_number,
            number: rng.gen_range(1..=self.max_number),
            guesses: 0,
            last_guess_ordering: None,
        }
    }
}

struct GuessANumberState {
    max_number: u32,
    number: u32,
    guesses: u32,
    last_guess_ordering: Option<Ordering>,
}

impl State<GuessANumber> for GuessANumberState {
    fn progress_report(&self) -> ProgressReport<GuessANumber> {
        use ProgressReport::*;

        match self.last_guess_ordering {
            Some(Ordering::Equal) => Finished(self.guesses),
            other => InProgress(GuessANumberView(other)),
        }
    }

    fn move_reducer(&self, mv: u32) -> Result<GuessANumberState, GuessANumberError> {
        if mv <= 0 || mv > self.max_number {
            Err(GuessANumberError::OutOfBounds)
        } else if self.last_guess_ordering == Some(Ordering::Equal) {
            Err(GuessANumberError::AleradyFinished)
        } else {
            Ok(GuessANumberState {
                guesses: self.guesses + 1,
                last_guess_ordering: Some(self.number.cmp(&mv)),
                ..*self
            })
        }
    }
}

struct GuessANumberView(Option<Ordering>);

impl Display for GuessANumberView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::cmp::Ordering::*;

        let info = match self {
            GuessANumberView(None) => "First guess",
            GuessANumberView(Some(Greater)) => "Greater",
            GuessANumberView(Some(Less)) => "Lesser",
            GuessANumberView(Some(Equal)) => "Equal",
        };

        write!(f, "{}", info)
    }
}

#[derive(Debug)]
enum GuessANumberError {
    AleradyFinished,
    OutOfBounds,
}

impl Display for GuessANumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GuessANumberError {}

fn main() {
    let game = GuessANumber { max_number: 100 };
    run_cli(game);
}
