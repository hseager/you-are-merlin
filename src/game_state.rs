use colored::Colorize;

use crate::{
    actions::*,
    battle_manager,
    characters::Player,
    game_data::{entities::*, GameData},
    items::create_item,
    player_state::PlayerState,
    prompts::*,
};

pub struct GameState {
    pub state: PlayerState,
    current_location: usize,
    current_encounter: usize,
    pub actions: Vec<Action>,
    pub game_data: GameData,
    pub items: Vec<&'static str>,
    pub accepted_quests: Vec<SideQuest>,
    pub completed_locations: Vec<Location>,
}

// TODO getting too big, need to split this out
impl GameState {
    pub fn new(game_data: GameData) -> GameState {
        let current_location = 0;

        let location = game_data
            .locations
            .get(current_location)
            .expect("Unable to get location when creating a new game state.");

        GameState {
            state: PlayerState::Visiting(location.clone()),
            current_location,
            current_encounter: 0,
            actions: get_visiting_actions(location.clone(), Vec::new()),
            items: game_data.items.clone(),
            game_data,
            accepted_quests: Vec::new(),
            completed_locations: Vec::new(),
        }
    }

    pub fn handle_action(&mut self, search: &str, player: &mut Player) {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => {
                    self.state = PlayerState::Travelling(self.game_data.locations.clone())
                }
                ActionType::Explore => match self.get_current_encounter() {
                    Encounter::Battle(_) => {
                        self.state = PlayerState::Battle(self.get_current_encounter().clone())
                    }
                    Encounter::BossFight(_) => {
                        self.state = PlayerState::Battle(self.get_current_encounter().clone())
                    }
                    Encounter::Quest(quest) => self.state = PlayerState::Quest(quest.clone()),
                },
                ActionType::MoveToLocation => {
                    let next_location_index = self.game_data
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase().contains(&search.to_lowercase()))
                        .expect("Unable to find location index when moving location and comparing names.");

                    self.current_location = next_location_index;

                    self.state = PlayerState::Visiting(self.get_current_location().clone());
                }
                ActionType::Attack => battle_manager::handle_battle(self, player),
                ActionType::Run => {
                    self.state = PlayerState::Visiting(self.get_current_location().clone());

                    self.reset_encounters();
                }
                ActionType::Rest => {
                    println!("You stay and rest a while...");

                    player.heal();
                }
                ActionType::Accept => {
                    println!("You accept their request.");

                    let encounter = self.get_current_encounter();

                    if let Encounter::Quest(Quest::SideQuest(side_quest)) = encounter {
                        self.accepted_quests.push(side_quest.clone());
                    }

                    self.state = PlayerState::Visiting(self.get_current_location().clone());
                }
                ActionType::Continue => {
                    println!("You acknowledge their request and continue exploring the area.");

                    self.go_to_next_encounter(player);
                    self.state = PlayerState::Battle(self.get_current_encounter().clone());
                }
                ActionType::GiveItem => {
                    println!("\"Your assistance in retrieving this has been invaluable. Thank you for your help! Please take this.\"");

                    self.completed_locations
                        .push(self.get_current_location().clone());

                    let item = create_item(&mut self.items);
                    player.equip_item(&item);

                    self.state = PlayerState::Treasure(item);
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }

        self.actions = self.get_actions(player);
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions.iter().find(|action| {
            action
                .name
                .trim()
                .to_lowercase()
                .contains(&search.to_lowercase())
        })
    }

    pub fn get_current_location(&self) -> &Location {
        self.game_data
            .locations
            .get(self.current_location)
            .expect("Failed to get location.")
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

    pub fn go_to_next_encounter(&mut self, player: &mut Player) {
        let next_encounter = self.current_encounter + 1;
        let location = self.get_current_location();

        if next_encounter < location.encounters.len() {
            self.current_encounter = next_encounter;

            let encounter = self.get_current_encounter();
            match encounter {
                Encounter::Battle(_) | Encounter::BossFight(_) => {
                    self.state = PlayerState::Battle(encounter.clone())
                }
                Encounter::Quest(quest) => self.state = PlayerState::Quest(quest.clone()),
            }
        } else {
            self.handle_end_of_encounters(location.clone(), player);
        }
    }

    fn handle_end_of_encounters(&mut self, location: Location, player: &mut Player) {
        match location.class {
            LocationType::Dungeon(item) => {
                println!(
                    "You successfully clear {} of dangers and stumble upon a safe area.",
                    location.name
                );

                let is_on_side_quest = self.accepted_quests.iter().any(|q| q.item == item.bold())
                    && !player.has_item_in_inventory(&item.bold());

                if is_on_side_quest {
                    println!("You find {}!", item.bold());
                    player.add_item_to_inventory(item.bold());
                }

                let item = create_item(&mut self.items);
                player.equip_item(&item);

                println!("You spot a chest up ahead and eagerly pry it open.");
                self.state = PlayerState::Treasure(item);
                self.reset_encounters();
            }
            LocationType::SafeZone => {
                self.state = PlayerState::Visiting(self.get_current_location().clone());
                self.reset_encounters();
            }
            _ => (),
        }
    }

    pub fn get_prompt(&self) -> String {
        match &self.state {
            PlayerState::Visiting(location) => {
                get_visiting_prompt(&location.name, location.description)
            }
            PlayerState::Travelling(_) => get_travelling_prompt(),
            PlayerState::Battle(encounter) => get_battle_prompt(encounter),
            PlayerState::Quest(quest) => get_quest_prompt(quest, self.accepted_quests.clone()),
            PlayerState::Treasure(item) => get_treasure_prompt(item),
            PlayerState::Win => get_win_prompt(),
            PlayerState::GameOver => get_game_over_prompt(),
        }
    }

    pub fn get_actions(&self, player: &Player) -> Vec<Action> {
        match &self.state {
            PlayerState::Visiting(location) => {
                get_visiting_actions(location.clone(), self.completed_locations.clone())
            }
            PlayerState::Battle(_) => get_battle_actions(),
            PlayerState::Quest(quest) => {
                get_quest_actions(quest.clone(), self.accepted_quests.clone(), player)
            }
            PlayerState::Travelling(locations) => get_locations_as_actions(locations.clone()),
            PlayerState::Treasure(_) => get_treasure_actions(),
            _ => vec![],
        }
    }

    pub fn get_actions_display_list(&self, player: &Player) -> String {
        self.get_actions(player)
            .iter()
            .map(|action| action.name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
