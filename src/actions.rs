use colored::{ColoredString, Colorize};

use crate::game_data::entities::Location;

#[derive(Clone)]
pub enum ActionType {
    Travel,
    Explore,
    Attack,
    Continue,
    Run,
    MoveToLocation,
    Rest,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: ColoredString,
}

impl Action {
    pub fn new(class: ActionType, name: ColoredString) -> Action {
        Action { class, name }
    }
}

pub fn get_visiting_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Travel, "Travel".yellow()),
        Action::new(ActionType::Explore, "Explore".blue()),
        Action::new(ActionType::Rest, "Rest".green()),
    ]
}
pub fn get_battle_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Attack, "Attack".red()),
        Action::new(ActionType::Run, "Run".cyan()),
    ]
}

pub fn get_quest_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Continue, "Continue".green()),
        Action::new(ActionType::Run, "Run".cyan()),
    ]
}

// TODO Fix .to_owned()
pub fn get_locations_as_actions(locations: &[Location]) -> Vec<Action> {
    locations
        .iter()
        .map(|location| Action::new(ActionType::MoveToLocation, location.name.to_owned()))
        .collect()
}
