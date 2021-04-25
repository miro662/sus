use std::fmt;

use tbsux::playered::Player;

use crate::variant::Variant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Team(u32, Variant);

impl Team {
    pub fn teams(variant: Variant) -> impl Iterator<Item = Team> {
        use Variant::*;
        let teams = match variant {
            ThreePlayers => 3,
            FourPlayers => 2,
        };
        (0..teams).map(move |id| Team(id, variant))
    }

    pub fn players(&self) -> impl Iterator<Item = Player> {
        use Variant::*;
        match self {
            Team(id @ 0..=2, ThreePlayers) => vec![*id].into_iter(),
            Team(0, FourPlayers) => vec![0, 2].into_iter(),
            Team(1, FourPlayers) => vec![1, 3].into_iter(),
            _ => panic!("Invalid team"),
        }
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Team {} [players: {}]",
            self.0,
            self.players()
                .map(|n| n.to_string())
                .reduce(|a, b| format!("{}, {}", a, b))
                .unwrap_or("".to_owned())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_displays_teams_for_three_players_variant() {
        let three_players_teams: Vec<_> = Team::teams(Variant::ThreePlayers).collect();
        assert_eq!("Team 0 [players: 0]", three_players_teams[0].to_string());
        assert_eq!("Team 1 [players: 1]", three_players_teams[1].to_string());
        assert_eq!("Team 2 [players: 2]", three_players_teams[2].to_string());
    }

    #[test]
    fn correctly_displays_teams_for_four_players_variant() {
        let four_players_teams: Vec<_> = Team::teams(Variant::FourPlayers).collect();
        assert_eq!("Team 0 [players: 0, 2]", four_players_teams[0].to_string());
        assert_eq!("Team 1 [players: 1, 3]", four_players_teams[1].to_string());
    }
}