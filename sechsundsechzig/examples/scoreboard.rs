use sechsundsechzig::{score::Score, team::Team, variant::Variant};

fn three_players() {
    let variant = Variant::ThreePlayers;
    let teams: Vec<_> = Team::teams(variant).collect();
    let mut scoreboard = Score::empty(variant);
    scoreboard.add_points(&teams[0], 21).unwrap();
    scoreboard.add_points(&teams[1], 66).unwrap();
    scoreboard.add_points(&teams[2], 37).unwrap();
    println!("{}", scoreboard);
}

fn four_players() {
    let variant = Variant::FourPlayers;
    let teams: Vec<_> = Team::teams(variant).collect();
    let mut scoreboard = Score::empty(variant);
    scoreboard.add_points(&teams[0], 4).unwrap();
    scoreboard.add_points(&teams[1], 20).unwrap();
    println!("{}", scoreboard);
}

fn main() {
    println!("Three players variant:");
    three_players();
    println!();
    println!("Four players variant:");
    four_players();
}
