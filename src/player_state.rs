use colored::ColoredString;

use crate::{
    actions::*,
    game_data::entities::*
};

// TODO move GameOver and Win into GameState and rename GameState to Game
// TODO Lose coupling of world
pub enum PlayerState<'a> {
    Travelling(&'a Vec<Location>),
    Visiting(&'a ColoredString, &'static str),
    Battle,
    Quest,
    GameOver,
    Win,
}

impl<'a> PlayerState<'a> {

    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            PlayerState::Visiting(_location_name, _location_description) => get_visiting_actions(),
            PlayerState::Battle => get_battle_actions(),
            PlayerState::Quest => get_quest_actions(),
            PlayerState::Travelling(locations) => get_locations_as_actions(locations),
            _ => vec![],
        }
    }

    pub fn get_actions_display_list(&self) -> String {
        self.get_actions()
            .iter()
            .map(|action| action.name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
