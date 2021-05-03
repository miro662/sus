use std::{collections::HashSet, fmt};

use tbsux::playered::Player;

use crate::{
    cards::{Card, Suit},
    contract::{Contract, GameType},
    error::{SechsUndSechzigError, SusResult},
    hands::Hand,
    ordering::greatest_card_in_suit,
    variant::Variant,
};

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

    pub fn try_play_card(&mut self, hand: &mut Hand, card: Card) -> SusResult<()> {
        self.check_card(hand, &card)?;
        hand.deal(card)?;
        self.play_card(card)?;
        Ok(())
    }

    pub fn play_card(&mut self, card: Card) -> SusResult<()> {
        if let Some(player) = self.current_player() {
            self.deals.push((player, card));
            Ok(())
        } else {
            Err(SechsUndSechzigError::FullTable)
        }
    }

    pub fn drawer(&self) -> Option<Player> {
        if let Some(_) = self.current_player() {
            None
        } else {
            self.greatest_card().and_then(|greatest| {
                self.deals
                    .iter()
                    .filter(|(_, card)| card == greatest)
                    .map(|(player, _)| *player)
                    .next()
            })
        }
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.deals.iter().map(|(_, card)| card)
    }

    pub fn filter_hand(&self, hand: &Hand) -> impl Iterator<Item = Card> {
        let satisfying_color_condition =
            self.satisfying_condition(hand.full().map(|c| *c), |Card { suit, .. }| {
                self.first_suit()
                    .map_or(true, |first_suit| suit == first_suit)
            });
        let satisfying_overbidding_condition =
            self.satisfying_condition(satisfying_color_condition, |card| {
                self.first_card()
                    .map(|first_card| {
                        let triumph = self.contract.game_type.triumph();
                        let triumph_when_first_card_not_triumph = triumph
                            .map(|t| first_card.suit != t && card.suit == t)
                            .unwrap_or(false);
                        let stronger_in_suit =
                            first_card.suit == card.suit && first_card.rank < card.rank;
                        triumph_when_first_card_not_triumph || stronger_in_suit
                    })
                    .unwrap_or(true)
            });
        satisfying_overbidding_condition
    }

    pub fn check_card(&self, hand: &Hand, card: &Card) -> SusResult<()> {
        let filtered_hand: HashSet<_> = self.filter_hand(hand).collect();
        if filtered_hand.contains(card) {
            Ok(())
        } else {
            Err(SechsUndSechzigError::CardCannotBePlayed)
        }
    }

    fn satisfying_condition<'a>(
        &self,
        cards: impl Iterator<Item = Card>,
        condition: impl Fn(&Card) -> bool,
    ) -> Box<dyn Iterator<Item = Card>> {
        let all_cards: Vec<_> = cards.collect();
        let satisfying_cards: Vec<Card> = all_cards.iter().map(|c| *c).filter(condition).collect();
        match satisfying_cards {
            cards if cards.is_empty() => Box::new(all_cards.into_iter()),
            cards => Box::new(cards.into_iter()),
        }
    }

    fn greatest_card(&self) -> Option<&Card> {
        let greatest_triumph = self
            .contract
            .game_type
            .triumph()
            .and_then(|ref triumph| greatest_card_in_suit(self.cards(), triumph));

        let greatest_in_first_card_suit = self
            .first_suit()
            .and_then(|suit| greatest_card_in_suit(self.cards(), suit));

        greatest_triumph.or(greatest_in_first_card_suit)
    }

    fn size(&self) -> usize {
        use GameType::*;
        match (self.contract.game_type, self.variant) {
            (Misery, _) | (Shower, _) => 3,
            (_, variant) => variant.number_of_players() as usize,
        }
    }

    fn first_card(&self) -> Option<&Card> {
        self.cards().next()
    }

    fn first_suit(&self) -> Option<&Suit> {
        self.first_card().map(|Card { suit, .. }| suit)
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
    use crate::{
        cards::{Rank, Suit},
        contract::GameType,
    };

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
    fn correctly_calculate_next_player_for_3_players() {
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
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
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
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
        let card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
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
