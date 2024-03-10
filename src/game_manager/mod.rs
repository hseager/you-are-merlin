use crate::{
    actions::{
        get_battle_actions, get_locations_as_actions, get_quest_actions, get_visiting_actions,
        Action, ActionType,
    },
    encounter::Encounter,
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
    Exploring,
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
                State::Exploring => {
                    if let Some(encounter) = location.encounters.get(location.current_encounter) {
                        match encounter {
                            Encounter::Battle(battle) => {
                                let Enemy {
                                    name, life, attack, ..
                                } = battle.enemy;

                                println!(
                                    "A wild {} appears! (life: {}, attack: {})",
                                    name, life, attack
                                )
                            }
                            Encounter::Quest(quest) => {
                                println!(
                                    "You find a calm area. {} wants to ask you something.",
                                    quest.character
                                );
                                println!(
                                    "\"Will you find {} and bring it back to me? I will make it worth your while...\"",
                                    quest.item
                                )
                            }
                        }
                    } else {
                        println!("Completed encounters");
                    }
                }
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
                ActionType::Travel => {
                    self.state = State::Travelling;
                    self.actions = self.get_actions(&self.state);
                }
                ActionType::Explore => {
                    self.state = State::Exploring;
                    self.actions = self.get_actions(&self.state);

                    // TODO Handle Accepting quest etc
                }
                ActionType::MoveToLocation => {
                    self.state = State::Visiting;
                    self.actions = self.get_actions(&self.state);
                    self.current_location = self
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .unwrap();
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    self.state = State::Visiting;
                    self.actions = self.get_actions(&self.state);

                    self.locations
                        .get_mut(self.current_location)
                        .unwrap()
                        .reset_encounters();
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions
            .iter()
            .find(|action| action.name.trim().to_lowercase() == search.to_lowercase())
    }

    fn get_actions(&self, state: &State) -> Vec<Action> {
        match state {
            State::Visiting => get_visiting_actions(),
            State::Exploring => {
                let current_encounter = self.get_current_location().get_current_encounter();
                match current_encounter {
                    Encounter::Battle(_) => get_battle_actions(),
                    Encounter::Quest(_) => get_quest_actions(),
                }
            }
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

    // TODO move initial values to config
    let player = Player {
        name,
        life: 100,
        attack: 5,
    };

    GameState {
        player,
        state: State::Visiting,
        current_location: get_random_array_index(&locations),
        locations,
        actions: get_visiting_actions(),
    }
}
