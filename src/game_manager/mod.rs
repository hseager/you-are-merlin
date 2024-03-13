use colored::Colorize;

use crate::{
    actions::{
        get_battle_actions, get_locations_as_actions, get_quest_actions, get_visiting_actions,
        Action, ActionType,
    },
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    encounter::{Encounter, Quest},
    enemy::Enemy,
    location::Location,
    player::Player,
    theme::load_theme,
    utilities::get_random_array_index,
};

mod battle_manager;
mod world_builder;

pub struct GameState {
    pub player: Player,
    pub state: State,
    pub current_location: usize,
    pub locations: Vec<Location>,
    pub actions: Vec<Action>,
}

pub enum State {
    Travelling,
    Visiting,
    Battle,
    Quest,
    GameOver,
}

impl GameState {
    pub fn get_current_location(&self) -> &Location {
        &self.locations.get(self.current_location).unwrap()
    }

    pub fn get_current_prompt(&self) -> () {
        if let Some(location) = self.locations.get(self.current_location) {
            match &self.state {
                State::Visiting => {
                    println!(
                        "You are currently visiting {}. {}",
                        location.display_name(),
                        location.description
                    );
                    println!("What would you like to do?")
                }
                State::Travelling => println!("Where would you like to go?"),
                State::Battle => match self.get_current_location().get_current_encounter() {
                    Encounter::Battle(battle) => {
                        let Enemy {
                            name, life, attack, ..
                        } = battle.enemy;

                        println!(
                            "A wild {} appears! (life: {}, attack: {})",
                            name, life, attack
                        )
                    }
                    _ => (),
                },
                State::Quest => match self.get_current_location().get_current_encounter() {
                    Encounter::Quest(quest) => {
                        match quest {
                            Quest::MainQuest(quest) => {
                                println!(
                                    "You find a calm area. {} wants to ask you something.",
                                    quest.character.bold()
                                );
                                println!(
                                    "\"There is a great evil in this world... {}... They must be stopped...\"", "BOSS_NAME", // TODO get boss into
                                )
                            },
                            Quest::SideQuest(quest) => {
                                println!(
                                    "You find a calm area. {} wants to ask you something.",
                                    quest.character.bold()
                                );
                                println!(
                                    "\"Will you find {} and bring it back to me? I will make it worth your while...\"",
                                    quest.item.bold()
                                )
                            }
                        }

                    }
                    _ => (),
                },
                _ => (),
            }
        } else {
            println!("Couldn't find location.");
        }

        println!("{}", &self.get_actions_display_list());
    }

    pub fn handle_action(&mut self, search: &str) -> () {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => self.state = State::Travelling,
                ActionType::Explore => match self.get_current_location().get_current_encounter() {
                    Encounter::Battle(_) => self.state = State::Battle,
                    Encounter::Quest(_) => self.state = State::Quest
                },
                ActionType::MoveToLocation => {
                    self.state = State::Visiting;
                    self.current_location = self
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .unwrap();
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    self.state = State::Visiting;

                    self.locations
                        .get_mut(self.current_location)
                        .unwrap()
                        .reset_encounters();
                }
                ActionType::Accept => {
                    println!("You agree to the request and continue exploring the area.");

                    self.locations
                        .get_mut(self.current_location)
                        .unwrap()
                        .go_to_next_encounter();

                    self.state = State::Battle;
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }

        self.actions = self.get_actions(&self.state);
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions
            .iter()
            .find(|action| action.name.trim().to_lowercase() == search.to_lowercase())
    }

    fn get_actions(&self, state: &State) -> Vec<Action> {
        match state {
            State::Visiting => get_visiting_actions(),
            State::Battle => get_battle_actions(),
            State::Quest => get_quest_actions(),
            State::Travelling => get_locations_as_actions(&self.locations),
            _ => vec![],
        }
    }

    fn get_actions_display_list(&self) -> String {
        self.get_actions(&self.state)
            .iter()
            .map(|action| action.display_name())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub fn init_game() -> GameState {
    let theme = load_theme();
    let name = theme.main_character;
    let locations = world_builder::build_world(&theme);

    let player = Player {
        name,
        life: PLAYER_LIFE,
        attack: PLAYER_ATTACK,
    };

    GameState {
        player,
        state: State::Visiting,
        current_location: get_random_array_index(&locations),
        locations,
        actions: get_visiting_actions(),
    }
}
