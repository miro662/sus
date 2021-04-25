/// Describes variant of game (three players/four players)
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Variant {
    ThreePlayers,
    FourPlayers,
}

impl Variant {
    pub fn number_of_players(&self) -> u32 {
        use Variant::*;

        match self {
            ThreePlayers => 3,
            FourPlayers => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_number_of_players_for_variant() {
        assert_eq!(3, Variant::ThreePlayers.number_of_players());
        assert_eq!(4, Variant::FourPlayers.number_of_players());
    }
}
