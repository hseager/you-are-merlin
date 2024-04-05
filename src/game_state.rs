use crate::game_data::{
    entities::{Location, SideQuest},
    GameData,
};

pub struct GameState {
    current_location: usize,
    current_encounter: usize,
    pub game_data: GameData,
    pub items: Vec<&'static str>,
    pub accepted_quests: Vec<SideQuest>,
    pub completed_locations: Vec<Location>,
}

impl GameState {
    pub fn new(game_data: GameData) -> GameState {
        let current_location = 0;

        GameState {
            current_location,
            current_encounter: 0,
            items: game_data.items.clone(),
            game_data,
            accepted_quests: Vec::new(),
            completed_locations: Vec::new(),
        }
    }

    pub fn get_locations(&self) -> Vec<Location> {
        self.game_data.locations.clone()
    }

    pub fn get_current_location(&self) -> &Location {
        self.game_data
            .locations
            .get(self.current_location)
            .expect("Failed to get location.")
    }

    fn get_location_index_by_name(&self, name: String) -> usize {
        self.game_data
            .locations
            .iter()
            .position(|location| location.name.to_lowercase().contains(&name.to_lowercase()))
            .expect("Unable to find location index when moving location and comparing names.")
    }

    pub fn change_location_by_name(&mut self, name: String) {
        self.current_location = self.get_location_index_by_name(name);
    }
}
