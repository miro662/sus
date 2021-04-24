use std::cmp::Ordering;
use std::io;

use io::Write;
use rand::Rng;

use tbsux::prelude::*;

struct GuessANumber {max_number: u32}

impl Game for GuessANumber {
    type State = GuessANumberState;
    type Move = u32;
    type Result = u32;      // number of guesses
    type View = Option<Ordering>;   // result of comparsion between last guess

    fn initial_state(&self) -> Self::State {
        let mut rng = rand::thread_rng();
        GuessANumberState {
            number: rng.gen_range(1..=self.max_number),
            guesses: 0,
            last_guess_ordering: None
        }
    }
}

struct GuessANumberState {
    number: u32,
    guesses: u32,
    last_guess_ordering: Option<Ordering>
}

impl State<GuessANumber> for GuessANumberState {
    fn progress_report(&self) -> ProgressReport<GuessANumber> {
        use ProgressReport::*;

        match self.last_guess_ordering {
            Some(Ordering::Equal) => Finished(self.guesses),
            other => InProgress(other)
        }
    }

    fn move_reducer(&self, mv: u32) -> GuessANumberState {
        GuessANumberState {
            guesses: self.guesses + 1,
            last_guess_ordering: Some(self.number.cmp(&mv)),
            ..*self
        }
    }
}


fn main() {
    let game = GuessANumber{max_number: 100};
    let mut state = game.initial_state();
    let result = loop {
        match state.progress_report() {
            ProgressReport::Finished(result) => break result,
            ProgressReport::InProgress(view) => {
                print!("{esc}c", esc = 27 as char);
                println!("View: {:?}", view);
                print!("Move: ");
                
                let mv: u32 = loop {
                    let mut buf = String::new();
                    io::stdout().flush()
                        .expect("Could not flush stdout");
                    io::stdin().read_line(&mut buf)
                        .expect("Could not read line from stdin");
                    
                    match buf.trim().parse::<u32>() {
                        Ok(mv) => break mv,
                        Err(_) => println!("Could not parse move; enter vaild move")
                    }
                };

                state = state.move_reducer(mv);
            }
        }
    };
    println!("Game finished; result: {}", result);
}
