use std::error::Error;

pub trait Game
where
    Self: Sized,
{
    type State: State<Self>;
    type Move;
    type Result;
    type View;
    type Error: Error;

    fn initial_state(&self) -> Self::State;
}

pub enum ProgressReport<G>
where
    G: Game,
{
    Finished(<G as Game>::Result),
    InProgress(<G as Game>::View),
}
pub trait State<G>
where
    G: Game,
    Self: Sized,
{
    fn progress_report(&self) -> ProgressReport<G>;
    fn move_reducer(&self, mv: <G as Game>::Move) -> Result<Self, <G as Game>::Error>;
}
