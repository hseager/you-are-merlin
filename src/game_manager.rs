use colored::Color;

use crate::{
    actions::{Action, ActionItem},
    location::Location,
};

pub struct GameState {
    pub current_location: Location,
    pub actions: Vec<ActionItem>,
}

impl GameState {
    pub fn get_actions_list(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.display_label())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn has_action(&self, search: &str) -> bool {
        self.actions
            .iter()
            .any(|action| action.label.to_lowercase() == search.to_lowercase())
    }
}

pub fn init_game() -> GameState {
    let initial_actions = vec![
        ActionItem::new(Action::Travel, "Travel", Color::Blue),
        ActionItem::new(Action::Explore, "Explore", Color::Yellow)
    ];

    GameState {
        current_location: Location::DarklingWoods,
        actions: initial_actions,
    }
}
