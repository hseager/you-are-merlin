use colored::ColoredString;

use crate::{actions::*, game_data::entities::*};

pub enum PlayerState<'a> {
    Travelling(&'a Vec<Location>),
    Visiting(&'a ColoredString, &'static str),
    Battle(&'a Encounter),
    Quest(&'a Encounter),
    GameOver,
    Win,
}

impl<'a> PlayerState<'a> {
    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            PlayerState::Visiting(_, _) => get_visiting_actions(),
            PlayerState::Battle(_) => get_battle_actions(),
            PlayerState::Quest(_) => get_quest_actions(),
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
