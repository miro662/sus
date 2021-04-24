use crate::core;

pub type Player = u32;

pub trait Game: core::Game
where
    <Self as core::Game>::View: View,
{
    fn no_of_players(&self) -> u32;
}

pub trait View {
    type PlayerView;

    fn current_player(&self) -> Player;
    fn player_view(&self, player: Player) -> Self::PlayerView;
}
