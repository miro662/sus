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

    pub fn cards_per_player(&self) -> usize {
        24 / self.number_of_players() as usize
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

    #[test]
    fn returns_correct_number_of_cards_per_player_for_variant() {
        assert_eq!(8, Variant::ThreePlayers.cards_per_player());
        assert_eq!(6, Variant::FourPlayers.cards_per_player());
    }
}
