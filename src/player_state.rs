use crate::{actions::*, game_data::entities::*, prompts::*};

pub enum PlayerState<'a> {
    Travelling(&'a Vec<Location>),
    Visiting(&'a Location),
    Battle(&'a Encounter),
    Quest(&'a Encounter),
    GameOver,
    Win,
}

impl<'a> PlayerState<'a> {
    pub fn get_prompt(&self) {
        match &self {
            PlayerState::Visiting(location) => {
                get_visiting_prompt(&location.name, location.description)
            }
            PlayerState::Travelling(_) => get_travelling_prompt(),
            PlayerState::Battle(encounter) => get_battle_prompt(encounter),
            PlayerState::Quest(encounter) => get_quest_prompt(encounter),
            _ => panic!("Unhandled PlayerState"),
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            PlayerState::Visiting(location) => get_visiting_actions(location),
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
