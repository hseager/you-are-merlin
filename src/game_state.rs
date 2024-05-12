use rand::Rng;

use crate::item::armour::Armour;
use crate::item::artifact::Artifact;
use crate::item::reward_type::RewardType;
use crate::item::weapon::Weapon;
use crate::theme::ThemeItem;
use crate::{
    game_data::{
        entities::{Encounter, Location, SideQuest},
        GameData,
    },
    item::{Item, ItemType},
};

pub struct GameState {
    current_location: usize,
    current_encounter: usize,
    pub game_data: GameData,
    pub items: Vec<ThemeItem>,
    pub accepted_quests: Vec<SideQuest>,
    pub completed_locations: Vec<Location>,
    pub is_running: bool,
}

impl GameState {
    pub fn new(game_data: GameData) -> GameState {
        GameState {
            current_location: 0,
            current_encounter: 0,
            items: game_data.items.clone(),
            game_data,
            accepted_quests: Vec::new(),
            completed_locations: Vec::new(),
            is_running: true,
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

    pub fn get_current_encounter(&self) -> &Encounter {
        let location = self.get_current_location();

        location
            .encounters
            .get(self.current_encounter)
            .expect("Failed to get encounter.")
    }

    pub fn reset_encounters(&mut self) {
        self.current_encounter = 0;
    }

    pub fn go_to_next_encounter(&mut self) -> Option<&Encounter> {
        let next_encounter = self.current_encounter + 1;
        let location = self.get_current_location();

        if next_encounter < location.encounters.len() {
            self.current_encounter = next_encounter;

            Some(self.get_current_encounter())
        } else {
            None
        }
    }

    fn get_random_item(&mut self) -> ThemeItem {
        // TODO create generic items after running out
        assert!(!self.items.is_empty(), "Out of items..");

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.items.len());

        // Remove the item from the list so that it's unique
        self.items.remove(index)
    }

    pub fn get_reward_item(&mut self, reward_type: RewardType) -> Box<dyn Item> {
        let item = self.get_random_item();

        let rarity = reward_type.roll_rarity();

        match item.item_type {
            ItemType::Weapon => Box::new(Weapon::new(item.name.to_string(), rarity)),
            ItemType::Armour => Box::new(Armour::new(item.name.to_string(), rarity)),
            ItemType::Artifact => Box::new(Artifact::new(item.name.to_string(), rarity)),
            _ => panic!("Unsupported reward item type"),
        }
    }
}
