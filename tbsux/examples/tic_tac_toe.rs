use std::{
    error::Error,
    fmt::{self, Display},
};

use playered::View;
use tbsux::{cli, playered, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Cross,
    Circle,
}

impl Symbol {
    fn other(&self) -> Symbol {
        use Symbol::*;
        match self {
            Cross => Circle,
            Circle => Cross,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Symbol::*;
        write!(
            f,
            "{}",
            match self {
                Cross => "X",
                Circle => "O",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Field(Option<Symbol>);

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Field(Some(s)) => s.to_string(),
                Field(None) => " ".to_string(),
            }
        )
    }
}

#[derive(Clone)]
struct Board([Field; 9]);

impl Board {
    fn empty() -> Board {
        Board([Field(None); 9])
    }

    fn place(&mut self, position: u8, symbol: Symbol) -> Result<(), TicTacToeError> {
        if position >= 9 {
            Err(TicTacToeError::OutOfBoard)
        } else if self.0[position as usize] != Field(None) {
            Err(TicTacToeError::AlreadyOccupied)
        } else {
            self.0[position as usize] = Field(Some(symbol));
            Ok(())
        }
    }

    fn winner(&self) -> Option<Symbol> {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        LINES
            .iter()
            .map(|line| {
                (
                    line[0],
                    self.0[line[0]] == self.0[line[1]] && self.0[line[1]] == self.0[line[2]],
                )
            })
            .filter(|(_, winning)| *winning)
            .map(|(sample, _)| {
                if let Field(Some(symbol)) = self.0[sample] {
                    Some(symbol)
                } else {
                    None
                }
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .next()
    }

    fn filled(&self) -> bool {
        self.0.iter().all(|f| f.0.is_some())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "+---+---+---+")?;
        for row in 0..3 {
            write!(f, "| ")?;
            write!(
                f,
                "{}",
                (0..3)
                    .map(|col| self.0[row * 3 + col])
                    .map(|f| f.to_string())
                    .reduce(|a, b| format!("{} | {}", a, b))
                    .unwrap_or("#ERR".to_owned())
            )?;
            writeln!(f, " |\n+---+---+---+")?;
        }
        Ok(())
    }
}

struct TicTacToeGame;

impl Game for TicTacToeGame {
    type State = TicTacToeState;
    type Move = u8;
    type Result = TicTacToeResult;
    type View = TicTacToeView;
    type Error = TicTacToeError;

    fn initial_state(&self) -> Self::State {
        TicTacToeState {
            board: Board::empty(),
            current_player: Symbol::Cross,
        }
    }
}

impl playered::Game for TicTacToeGame {
    fn no_of_players(&self) -> u32 {
        2
    }
}

struct TicTacToeState {
    board: Board,
    current_player: Symbol,
}

impl State<TicTacToeGame> for TicTacToeState {
    fn progress_report(&self) -> ProgressReport<TicTacToeGame> {
        use ProgressReport::*;
        use TicTacToeResult::*;

        match self.board.winner() {
            Some(winner) => Finished(Victory(winner)),
            None => {
                if self.board.filled() {
                    Finished(Tie)
                } else {
                    InProgress(TicTacToeView {
                        board: self.board.clone(),
                        current_player: self.current_player,
                    })
                }
            }
        }
    }

    fn move_reducer(&self, mv: u8) -> Result<TicTacToeState, TicTacToeError> {
        let mut new_board = self.board.clone();
        new_board.place(mv, self.current_player)?;
        Ok(TicTacToeState {
            board: new_board,
            current_player: self.current_player.other(),
        })
    }
}

#[derive(Clone)]
struct TicTacToeView {
    board: Board,
    current_player: Symbol,
}

impl Display for TicTacToeView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nPlayer {} move", self.board, self.current_player)
    }
}

impl View for TicTacToeView {
    type PlayerView = Self;

    fn current_player(&self) -> playered::Player {
        match self.current_player {
            Symbol::Cross => 1,
            Symbol::Circle => 2,
        }
    }

    fn player_view(&self, _: playered::Player) -> Self::PlayerView {
        self.clone()
    }
}

#[derive(Debug)]
enum TicTacToeError {
    OutOfBoard,
    AlreadyOccupied,
}

impl Display for TicTacToeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TicTacToeError {}

enum TicTacToeResult {
    Victory(Symbol),
    Tie,
}

impl Display for TicTacToeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TicTacToeResult::*;
        match self {
            Victory(symbol) => write!(f, "Winner: {}", symbol),
            Tie => write!(f, "Tie"),
        }
    }
}

fn main() {
    let game = TicTacToeGame;
    cli::run_cli(game);
}
