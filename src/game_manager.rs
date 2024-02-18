use crate::{
    actions::{get_map_actions, Action},
    location::Location,
};

pub struct GameState {
    pub current_location: Location,
    pub actions: Vec<Action>,
}

impl GameState {
    pub fn get_actions_list(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub fn init_game() -> GameState {
    GameState {
        current_location: Location::DarklingWoods,
        actions: get_map_actions(),
    }
}
