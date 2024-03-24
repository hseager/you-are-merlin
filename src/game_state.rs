use std::{thread::sleep, time::Duration};

use crate::{
    actions::{get_visiting_actions, Action, ActionType},
    battle_manager,
    characters::Player,
    config::{PLAYER_ATTACK, PLAYER_LIFE, REST_INTERVAL_SECONDS},
    game_data::{
        entities::{Encounter, Location},
        GameData,
    },
    items::get_encounter_reward,
    player_state::PlayerState,
};

pub struct GameState<'a> {
    pub state: PlayerState<'a>,
    current_location: usize,
    current_encounter: usize,
    pub actions: Vec<Action>,
    pub player: Player<'a>,
    pub game_data: &'a GameData,
    pub items: Vec<&'static str>,
}

impl<'a> GameState<'a> {
    pub fn new(game_data: &GameData) -> GameState {
        let current_location = 0;

        let location = game_data.locations.get(current_location).unwrap();

        let player = Player {
            name: &game_data.main_character,
            max_life: PLAYER_LIFE,
            life: PLAYER_LIFE,
            attack: PLAYER_ATTACK,
        };

        GameState {
            state: PlayerState::Visiting(location),
            current_location,
            current_encounter: 0,
            actions: get_visiting_actions(location),
            player,
            items: game_data.items.clone(),
            game_data,
        }
    }

    pub fn get_prompt(&self) {
        self.state.get_prompt();
        println!("{}", &self.state.get_actions_display_list());
    }

    pub fn handle_action(&mut self, search: &str) {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => {
                    self.state = PlayerState::Travelling(&self.game_data.locations)
                }
                ActionType::Explore => match self.get_current_encounter() {
                    Encounter::Battle(_) => {
                        self.state = PlayerState::Battle(self.get_current_encounter())
                    }
                    Encounter::BossFight(_) => {
                        self.state = PlayerState::Battle(self.get_current_encounter())
                    }
                    Encounter::Quest(_) => {
                        self.state = PlayerState::Quest(self.get_current_encounter())
                    }
                },
                ActionType::MoveToLocation => {
                    let next_location_index = self.game_data
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase().contains(&search.to_lowercase()))
                        .expect("Unable to find location index when moving location and comparing names.");

                    self.current_location = next_location_index;

                    let current_location = self.get_current_location();

                    self.state = PlayerState::Visiting(current_location);
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    let current_location = self.get_current_location();

                    self.state = PlayerState::Visiting(current_location);

                    self.reset_encounters();
                }
                ActionType::Continue => {
                    println!("You acknowledge their request and continue exploring the area.");

                    self.go_to_next_encounter();
                    self.state = PlayerState::Battle(self.get_current_encounter());
                }
                ActionType::Rest => {
                    println!("You stay and rest a while...");

                    while self.player.life < self.player.max_life {
                        let heal_amount = 10;
                        let mut new_life = self.player.life + heal_amount;
                        if new_life > self.player.max_life {
                            new_life = self.player.max_life;
                        }

                        println!(
                            "Your life increases by {} (life: {})",
                            new_life - self.player.life,
                            new_life
                        );
                        self.player.life = new_life;
                        sleep(Duration::from_secs(REST_INTERVAL_SECONDS as u64));
                    }
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }

        self.actions = self.state.get_actions();
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

    pub fn get_current_location(&self) -> &'a Location {
        self.game_data
            .locations
            .get(self.current_location)
            .expect("Failed to get location.")
    }

    pub fn get_current_encounter(&self) -> &'a Encounter {
        let location = self.get_current_location();

        location
            .encounters
            .get(self.current_encounter)
            .expect("Failed to get encounter.")
    }

    pub fn reset_encounters(&mut self) {
        self.current_encounter = 0;
    }

    pub fn go_to_next_encounter(&mut self) {
        let next_encounter = self.current_encounter + 1;

        let location = self.get_current_location();

        if next_encounter < location.encounters.len() {
            self.current_encounter = next_encounter;

            let encounter = self.get_current_encounter();
            match encounter {
                Encounter::Battle(_) | Encounter::BossFight(_) => {
                    self.state = PlayerState::Battle(encounter)
                }
                Encounter::Quest(_) => self.state = PlayerState::Quest(encounter),
            }
        } else {
            println!(
                "You clear {} of danger and find a chest ahead...",
                location.name
            );

            let item = get_encounter_reward(&mut self.items);

            self.player.attack += item.attack;
            self.player.max_life += item.max_life;

            self.state = PlayerState::Treasure(item);
            self.reset_encounters();
        }
    }
}
