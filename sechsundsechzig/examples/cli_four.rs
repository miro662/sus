use sechsundsechzig::{game::SechsUndSechzig, variant::Variant};
use tbsux::cli::run_cli;

fn main() {
    let game = SechsUndSechzig::with_random_seed(Variant::FourPlayers);
    run_cli(game);
}
