use std::fmt;

use tbsux::playered::Player;

use crate::{cards::Suit, variant::Variant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    NonTriumph,
    AskingAbout(Suit),
    LookingFor(Suit),
    Misery,
    Shower,
}

impl GameType {
    pub fn triumph(&self) -> Option<Suit> {
        use GameType::*;
        match self {
            AskingAbout(suit) => Some(*suit),
            LookingFor(suit) => Some(*suit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    pub game_type: GameType,
    pub dealer: Player,
    pub multiplier: i32,
}

impl Contract {
    pub fn initial(dealer: Player) -> Contract {
        Contract {
            game_type: GameType::NonTriumph,
            dealer,
            multiplier: 1,
        }
    }

    pub fn next_player(&self, player: Player, variant: &Variant) -> Player {
        (player + 1) % variant.number_of_players()
    }

    pub fn dealers_teammate(&self, variant: &Variant) -> Option<Player> {
        use Variant::*;
        match variant {
            ThreePlayers => None,
            FourPlayers => Some((self.dealer + 2) % 4)
        }
    }
}

impl fmt::Display for GameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GameType::*;
        match self {
            NonTriumph => write!(f, "non-triumph"),
            AskingAbout(triumph) => write!(f, "asking-about, triumph: {}", triumph),
            LookingFor(triumph) => write!(f, "looking-for, triumph: {}", triumph),
            Misery => write!(f, "misery"),
            Shower => write!(f, "shower"),
        }
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, multiplier: x{}, dealer: player {}",
            self.game_type, self.multiplier, self.dealer
        )
    }
}
