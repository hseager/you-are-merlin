use colored::Colorize;

// TODO split game_manager out and use a trait for handling actions

use crate::{
    actions::{
        get_battle_actions, get_locations_as_actions, get_quest_actions, get_visiting_actions,
        Action, ActionType,
    },
    characters::{Enemy, Player},
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    encounter::{Encounter, Quest},
    theme::load_theme,
    utilities::get_random_array_index,
    world::World,
};

mod battle_manager;
mod world_builder;

pub struct GameState {
    pub player: Player,
    pub state: State,
    pub world: World,
    pub actions: Vec<Action>,
}

pub enum State {
    Travelling,
    Visiting,
    Battle,
    Quest,
    GameOver,
    Win,
}

impl GameState {
    pub fn get_current_prompt(&self) -> () {
        if let Some(location) = self.world.locations.get(self.world.current_location) {
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
                State::Battle => match self.world.get_current_location().get_current_encounter() {
                    Encounter::Battle(battle) => {
                        let Enemy {
                            name,
                            life,
                            attack,
                            description,
                            ..
                        } = &battle.enemy;

                        println!(
                            "A wild {} appears! (life: {}, attack: {})",
                            name, life, attack
                        );
                        println!("{description}")
                    }
                    Encounter::BossFight(battle) => {
                        let Enemy {
                            name,
                            life,
                            attack,
                            description,
                            ..
                        } = &battle.enemy;

                        println!("A great danger approaches...");
                        println!("{} (life: {}, attack: {})", name, life, attack);
                        println!("{description}")
                    }
                    _ => (),
                },
                State::Quest => match self.world.get_current_location().get_current_encounter() {
                    Encounter::Quest(quest) => match quest {
                        Quest::MainQuest(quest) => {
                            println!(
                                "You find a calm area. {} wants to ask you something.",
                                quest.character.bold()
                            );
                            println!(
                                "\"{} is in great danger... {} seeks the destruction of this world... They must be stopped...\"",
                                self.world.name, quest.boss_name,
                            )
                        }
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
                    },
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
                ActionType::Explore => {
                    match self.world.get_current_location().get_current_encounter() {
                        Encounter::Battle(_) => self.state = State::Battle,
                        Encounter::BossFight(_) => self.state = State::Battle,
                        Encounter::Quest(_) => self.state = State::Quest,
                    }
                }
                ActionType::MoveToLocation => {
                    self.state = State::Visiting;
                    self.world.current_location = self
                        .world
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .unwrap();
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    self.state = State::Visiting;

                    self.world
                        .locations
                        .get_mut(self.world.current_location)
                        .unwrap()
                        .reset_encounters();
                }
                ActionType::Continue => {
                    println!("You acknowledge their request and continue exploring the area.");

                    self.world
                        .locations
                        .get_mut(self.world.current_location)
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
            State::Travelling => get_locations_as_actions(&self.world.locations),
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
    let locations = world_builder::build_world(&theme);

    println!("{:#?}", locations);

    let player = Player {
        name: theme.main_character.bold(),
        life: PLAYER_LIFE,
        attack: PLAYER_ATTACK,
    };

    let world: World = World {
        name: &theme.world_name,
        current_location: get_random_array_index(&locations),
        locations,
    };

    GameState {
        player,
        state: State::Visiting,
        world,
        actions: get_visiting_actions(),
    }
}
