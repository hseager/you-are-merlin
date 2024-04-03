use crate::{
    game_data::entities::{Encounter, Location, Quest},
    items::Item,
};

pub enum PlayerState {
    Travelling(Vec<Location>),
    Visiting(Location),
    Battle(Encounter),
    Quest(Quest),
    Treasure(Item),
    Healing,
    Fighting,
    GameOver,
    Win,
}
