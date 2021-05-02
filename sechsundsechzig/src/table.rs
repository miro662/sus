use std::fmt;

use tbsux::playered::Player;

use crate::{cards::Card, contract::{Contract, GameType}, error::{SechsUndSechzigError, SusResult}, variant::Variant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    variant: Variant,
    contract: Contract,
    initial_player: Player,
    deals: Vec<(Player, Card)>,
}

impl Table {
    pub fn empty(variant: Variant, contract: Contract, initial_player: Player) -> Table {
        Table {
            variant,
            contract,
            initial_player,
            deals: vec![],
        }
    }

    pub fn current_player(&self) -> Option<Player> {
        if self.size() == self.deals.len() {
            None
        } else {
            Some({
                let next_player = self
                    .deals
                    .last()
                    .map(|(p, _)| self.variant.next_player(*p))
                    .unwrap_or(self.initial_player);
                if self.contract.dealers_teammate(&self.variant) == Some(next_player)
                    && !self.contract.dealers_teammate_plays()
                {
                    self.variant.next_player(next_player)
                } else {
                    next_player
                }
            })
        }
    }

    pub fn play_card(&mut self, card: Card) -> SusResult<()>{
        if let Some(player) = self.current_player() {
            self.deals.push((player, card));
            Ok(())
        } else {
            Err(SechsUndSechzigError::FullTable)
        }
    }

    pub fn drawer(&self) -> Option<Player> {
        // TODO: correct drawer
        Some(0)
    }

    fn size(&self) -> usize {
        use GameType::*;
        match (self.contract.game_type, self.variant) {
            (Misery, _) | (Shower, _) => 3,
            (_, variant) => variant.number_of_players() as usize,
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.deals.len() == 0 {
            writeln!(f, "empty")
        } else {
            for (player, card) in &self.deals {
                writeln!(f, "Player {}: {}", player, card)?
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cards::{Rank, Suit}, contract::GameType};

    #[test]
    fn creates_empty_table() {
        let table = Table::empty(
            Variant::ThreePlayers,
            Contract {
                game_type: GameType::NonTriumph,
                dealer: 0,
                multiplier: 1,
            },
            0,
        );
        assert_eq!(table.deals, vec![])
    }

    #[test]
    fn correctly_calculate_next_player_for_3_players(){
        let card = Card {rank: Rank::Ace, suit: Suit::Spade};
        let mut table = Table::empty(
            Variant::ThreePlayers,
            Contract {
                game_type: GameType::NonTriumph,
                dealer: 0,
                multiplier: 1,
            },
            0,
        );
        assert_eq!(Some(0), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(1), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(2), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(None, table.current_player());
    }

    #[test]
    fn correctly_calculate_next_player_for_4_players_where_everyone_plays() {
        let card = Card {rank: Rank::Ace, suit: Suit::Spade};
        let mut table = Table::empty(
            Variant::FourPlayers,
            Contract {
                game_type: GameType::NonTriumph,
                dealer: 0,
                multiplier: 1,
            },
            0,
        );
        assert_eq!(Some(0), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(1), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(2), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(3), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(None, table.current_player());
    }

    #[test]
    fn correctly_calculate_next_player_for_4_players_where_dealers_teammate_does_not_play() {
        let card = Card {rank: Rank::Ace, suit: Suit::Spade};
        let mut table = Table::empty(
            Variant::FourPlayers,
            Contract {
                game_type: GameType::Misery,
                dealer: 0,
                multiplier: 1,
            },
            0,
        );
        assert_eq!(Some(0), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(1), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(Some(3), table.current_player());

        table.play_card(card).unwrap();
        assert_eq!(None, table.current_player());
    }
}
