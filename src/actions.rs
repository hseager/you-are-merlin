use colored::{Color, Colorize};

use crate::location::Location;

#[derive(Clone)]
pub enum ActionType {
    Travel,
    Explore,
    Attack,
    Continue,
    Run,
    MoveToLocation,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: &'static str,
    pub name_color: Color,
}

impl Action {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }

    pub fn new(class: ActionType, name: &'static str, name_color: Color) -> Action {
        Action {
            class,
            name,
            name_color,
        }
    }
}

pub fn get_visiting_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Travel, "Travel", Color::Yellow),
        Action::new(ActionType::Explore, "Explore", Color::Blue),
    ]
}
pub fn get_battle_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Attack, "Attack", Color::Red),
        Action::new(ActionType::Run, "Run", Color::Cyan),
    ]
}

pub fn get_quest_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Continue, "Continue", Color::Green),
        Action::new(ActionType::Run, "Run", Color::Cyan),
    ]
}

pub fn get_locations_as_actions(locations: &Vec<Location>) -> Vec<Action> {
    locations
        .iter()
        .map(|location| {
            Action::new(
                ActionType::MoveToLocation,
                &location.name,
                location.name_color,
            )
        })
        .collect()
}
