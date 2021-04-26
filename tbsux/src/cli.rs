use std::{fmt::Display, io, io::Write, str::FromStr};

use crate::prelude::*;

pub fn run_cli<G>(game: G) -> G::Result
where
    G: Game,
    G::View: Display,
    G::Move: FromStr,
    G::Result: Display,
{
    let mut state = game.initial_state();
    let result = loop {
        match state.progress_report() {
            ProgressReport::Finished(result) => break result,
            ProgressReport::InProgress(view) => {
                clear_screen();
                println!("{}", view);
                state = loop {
                    let mv = retrieve_move();
                    match state.move_reducer(mv) {
                        Ok(s) => break s,
                        Err(err) => println!("Invaild move: {}", err),
                    }
                }
            }
        }
    };
    clear_screen();
    println!("Game finished, result: {}", result);
    result
}

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn retrieve_move<M: FromStr>() -> M {
    print!("MOVE> ");
    loop {
        let mut buf = String::new();
        io::stdout().flush().expect("Could not flush stdout");
        io::stdin()
            .read_line(&mut buf)
            .expect("Could not read line from stdin");

        match buf.trim().parse() {
            Ok(mv) => break mv,
            Err(_) => println!("Could not parse move"),
        }
    }
}
