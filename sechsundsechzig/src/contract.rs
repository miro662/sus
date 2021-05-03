use std::{fmt, iter::once};

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

    pub fn dealers_teammate(&self, variant: &Variant) -> Option<Player> {
        use Variant::*;
        match variant {
            ThreePlayers => None,
            FourPlayers => Some((self.dealer + 2) % 4),
        }
    }

    pub fn dealers_teammate_plays(&self) -> bool {
        use GameType::*;

        match self.game_type {
            Misery | Shower => false,
            _ => true,
        }
    }

    pub fn can_declare(&self) -> bool {
        use GameType::*;
        match self.game_type {
            AskingAbout(_) | LookingFor(_) => true,
            _ => false,
        }
    }

    pub fn parties(&self, variant: &Variant) -> impl Iterator<Item = &Party> {
        use Party::*;
        match (self.game_type, variant) {
            (GameType::NonTriumph, Variant::ThreePlayers) => {
                [SinglePlayer(0), SinglePlayer(1), SinglePlayer(2)].iter()
            }
            _ => [Dealers, NonDealers].iter(),
        }
    }

    pub fn players_in_party(
        &self,
        variant: &Variant,
        party: &Party,
    ) -> Box<dyn Iterator<Item = Player>> {
        use Party::*;

        let dealer = self.dealer;
        let dealers_teammate = self.dealers_teammate(variant);

        let twice = |a, b| once(a).chain(once(b));
        let next = |p| variant.next_player(p);

        match (party, dealers_teammate) {
            (SinglePlayer(player), _) => Box::new(once(*player)),
            (Dealers, None) => Box::new(once(dealer)),
            (Dealers, Some(mate)) => Box::new(twice(dealer, mate)),
            (NonDealers, None) => Box::new(twice(next(dealer), next(next(dealer)))),
            (NonDealers, Some(mate)) => Box::new(twice(next(dealer), next(mate))),
        }
    }

    pub fn players_party(&self, variant: Variant, player: Player) -> Party {
        use Party::*;
        match (self.game_type, variant) {
            (GameType::NonTriumph, Variant::ThreePlayers) => SinglePlayer(player),
            _ if player == self.dealer || Some(player) == self.dealers_teammate(&variant) => {
                Dealers
            }
            _ => NonDealers,
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Party {
    Dealers,
    NonDealers,
    SinglePlayer(Player),
}
