use crate::{game_data::entities::*, items::Item};

pub enum PlayerState<'a> {
    Travelling(&'a Vec<Location>),
    Visiting(&'a Location),
    Battle(&'a Encounter),
    Quest(&'a Quest),
    Treasure(Item),
    GameOver,
    Win,
}
