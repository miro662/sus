pub trait Game
where
    Self: Sized,
{
    type State: State<Self>;
    type Move;
    type Result;
    type View;

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
{
    fn progress_report(&self) -> ProgressReport<G>;
    fn move_reducer(&self, mv: <G as Game>::Move) -> Self;
}
