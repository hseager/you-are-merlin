use colored::Colorize;

use crate::{
    actions::{get_visiting_actions, Action, ActionType},
    characters::Player,
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    encounter::Encounter,
    player_state::PlayerState,
    theme::load_theme,
    utilities::get_random_array_index,
    world::World,
};

mod battle_manager;
mod world_builder;

pub struct Game {
    pub player: Player,
    pub state: PlayerState,
    pub world: World,
    pub actions: Vec<Action>,
}

// Fix current_location.name.to_owned()

impl Game {
    pub fn get_current_prompt(&self) -> () {
        self.state.get_prompt(&self.world);

        println!("{}", &self.state.get_actions_display_list());
    }

    pub fn handle_action(&mut self, search: &str) -> () {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => {
                    self.state = PlayerState::Travelling(self.world.locations.clone())
                    // TODO remove clone
                }
                ActionType::Explore => match self.world.get_current_encounter() {
                    Encounter::Battle(_) => self.state = PlayerState::Battle,
                    Encounter::BossFight(_) => self.state = PlayerState::Battle,
                    Encounter::Quest(_) => self.state = PlayerState::Quest,
                },
                ActionType::MoveToLocation => {
                    let next_location_index = self
                        .world
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .expect("Unable to find location index when moving location and comparing names.");

                    self.world.current_location = next_location_index;

                    let current_location = self
                        .world
                        .get_current_location()
                        .expect("Failed to get_current_location() when moving to location");

                    self.state = PlayerState::Visiting(
                        current_location.name.to_owned(),
                        current_location.description,
                    );
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    let current_location = self
                        .world
                        .get_current_location()
                        .expect("Failed to get_current_location() when running.");

                    self.state = PlayerState::Visiting(
                        current_location.name.to_owned(),
                        current_location.description,
                    );

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

                    self.state = PlayerState::Battle;
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }

        self.actions = self.state.get_actions();
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions
            .iter()
            .find(|action| action.name.trim().to_lowercase() == search.to_lowercase())
    }
}

pub fn init_game() -> Game {
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

    let current_location = world
        .get_current_location()
        .expect("Failed to get_current_location() when initialising Game");

    Game {
        player,
        state: PlayerState::Visiting(
            current_location.name.to_owned(),
            current_location.description,
        ),
        world,
        actions: get_visiting_actions(),
    }
}
